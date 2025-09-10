pub(crate) mod builders;
pub(crate) mod cache;
pub(crate) mod lazy_fields;

use crate::{ArcPathBuf, AutoFlags, IsDefault, MuxConfig, Result, Tools, i18n::logs};
use cache::{CacheMI, CacheMICommon, CacheMIOfFile, CacheMIOfGroup, CacheState};
use rayon::prelude::*;
use std::{collections::HashMap, mem, path::Path};

/// Extracts and caches media information.
///
/// User-defined settings from [`MuxConfig`] take precedence over extracted values.
#[derive(Debug)]
pub struct MediaInfo<'a> {
    pub cfg: &'a MuxConfig,
    pub tools: Tools<'a>,
    pub cache: CacheMI,
    pub(crate) auto_flags: &'a AutoFlags,
    pub(crate) thread: u8,
}

impl<'a> From<&'a MuxConfig> for MediaInfo<'a> {
    fn from(cfg: &'a MuxConfig) -> MediaInfo<'a> {
        MediaInfo {
            cfg,
            tools: cfg.into(),
            cache: CacheMI::default(),
            auto_flags: &cfg.auto_flags,
            thread: 0,
        }
    }
}

impl MediaInfo<'_> {
    /// Clears all caches.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.cache = CacheMI::default();
    }

    /// Clears the caches of the current group and files.
    #[inline(always)]
    pub fn clear_current(&mut self) {
        self.cache.of_group = CacheMIOfGroup::default();
        self.cache.of_files.clear();
    }

    /// Clears the common cache.
    #[inline(always)]
    pub fn clear_common(&mut self) {
        self.cache.common = CacheMICommon::default();
    }

    /// Clears the cache of the current group.
    #[inline(always)]
    pub fn clear_group(&mut self) {
        self.cache.of_group = CacheMIOfGroup::default();
    }

    /// Clears the cache of the current files.
    #[inline(always)]
    pub fn clear_files(&mut self) {
        self.cache.of_files.clear();
    }

    /// Returns the length cache of files.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.cache.of_files.len()
    }

    /// Returns `true` if not cached any.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_default()
    }

    /// Returns `true` if cache of files empty.
    #[inline(always)]
    pub fn is_no_files(&self) -> bool {
        self.cache.of_files.is_default()
    }

    /// Attempts to insert a `media` file into the cache.
    /// Does nothing if matroska or mkvmerge info is already cached.
    ///
    /// # Errors
    ///
    /// - Retrieving either matroska or mkvmerge info fails.
    pub fn try_insert(&mut self, media: impl Into<ArcPathBuf> + AsRef<Path>) -> Result<()> {
        match self.try_key_cache_to_insert(media, true, false) {
            Ok(Some((key, cache))) => {
                self.cache.of_files.insert(key, cache);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Attempts to insert multiple `media` files into the cache.
    /// Does nothing if matroska or mkvmerge info is already cached.
    ///
    /// # Errors
    ///
    /// - **Only if** `exit_on_err` is `true`.
    ///
    /// - Retrieving either matroska or mkvmerge info fails.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn) and `exit_on_err` is `false`.
    ///
    /// - Retrieving either matroska or mkvmerge info fails.
    pub fn try_insert_many(
        &mut self,
        media: impl IntoParallelIterator<Item = impl Into<ArcPathBuf> + AsRef<Path>>,
        exit_on_err: bool,
    ) -> Result<()> {
        let cache_is_empty = self.cache.of_files.is_empty();
        // take JSON to exclude data race
        let json = mem::take(&mut self.tools.json);

        let map = media
            .into_par_iter()
            .filter_map(|media| {
                match self.try_key_cache_to_insert(media, exit_on_err, cache_is_empty) {
                    Ok(Some((key, cache))) => Some(Ok((key, cache))),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<HashMap<_, _>>>();

        self.tools.json = json;
        let map = map?;

        if cache_is_empty {
            self.cache.of_files = map;
        } else {
            self.cache.of_files.extend(map);
        }

        Ok(())
    }

    #[inline]
    fn try_key_cache_to_insert(
        &self,
        media: impl Into<ArcPathBuf> + AsRef<Path>,
        exit_on_err: bool,
        cache_is_empty: bool,
    ) -> Result<Option<(ArcPathBuf, CacheMIOfFile)>> {
        let m = media.as_ref();

        if !cache_is_empty
            && self.cache.of_files.get(m).map_or(false, |c| {
                c.matroska.is_cached() || c.mkvmerge_i.is_cached()
            })
        {
            return Ok(None);
        }

        if let Ok(mat) = self.build_matroska(m) {
            let mut cache = CacheMIOfFile::default();
            cache.matroska = CacheState::Cached(mat);
            return Ok(Some((media.into(), cache)));
        }

        match self.build_mkvmerge_i(m) {
            Ok(mkvmerge) => {
                let mut cache = CacheMIOfFile::default();
                cache.mkvmerge_i = CacheState::Cached(mkvmerge);
                Ok(Some((media.into(), cache)))
            }
            Err(e) if exit_on_err => Err(e),
            Err(e) => {
                logs::warn_not_recognized_media(m, e);
                Ok(None)
            }
        }
    }
}
