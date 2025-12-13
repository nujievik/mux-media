pub(crate) mod builders;
pub(crate) mod cache;
mod finalize;
mod it_signs;
pub(crate) mod lazy_fields;

use crate::{ArcPathBuf, Config, Result, Tools, i18n::logs};
use cache::{CacheMI, CacheMIOfFile, CacheMIOfGroup, CacheState};
use rayon::prelude::*;
use std::{collections::HashMap, path::Path};

/// Extracts and caches media information.
///
/// User-defined settings from [`MediaInfo::cfg`] take precedence over extracted values.
#[derive(Debug)]
#[non_exhaustive]
pub struct MediaInfo<'a> {
    pub cfg: &'a Config,
    pub tools: Tools<'a>,
    pub cache: CacheMI,
    /// Job number. Separates access to temp files.
    pub job: u8,
}

impl MediaInfo<'_> {
    pub fn new<'a>(cfg: &'a Config, job: u8) -> MediaInfo<'a> {
        MediaInfo {
            cfg,
            tools: cfg.into(),
            cache: Default::default(),
            job,
        }
    }

    /// Clears caches.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.cache.of_group = CacheMIOfGroup::default();
        self.cache.of_files.clear();
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
    ) -> Result<()> {
        let exit_on_err = self.cfg.exit_on_err;
        let cache_is_empty = self.cache.of_files.is_empty();

        let map = media
            .into_par_iter()
            .filter_map(|media| {
                match self.try_key_cache_to_insert(media, exit_on_err, cache_is_empty) {
                    Ok(Some((key, cache))) => Some(Ok((key, cache))),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<HashMap<_, _>>>()?;

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
            && self
                .cache
                .of_files
                .get(m)
                .map_or(false, |c| c.streams.is_cached())
        {
            return Ok(None);
        }

        match self.build_streams(m) {
            Ok(streams) => {
                let mut cache = CacheMIOfFile::default();
                cache.streams = CacheState::Cached(streams);
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
