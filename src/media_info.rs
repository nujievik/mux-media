pub(crate) mod builders;
pub(crate) mod cache;
mod finalize;
mod it_signs;
mod lazy_fields;

pub use cache::{MediaInfoCache, MediaInfoCacheOfFile, MediaInfoCacheOfGroup};
pub use lazy_fields::{
    MarkMediaInfoAudioDuration, MarkMediaInfoCacheOfFile, MarkMediaInfoPathTail,
    MarkMediaInfoPlayableDuration, MarkMediaInfoRelativeUpmost, MarkMediaInfoStem,
    MarkMediaInfoStreams, MarkMediaInfoStreamsOrder, MarkMediaInfoSubCharEncoding,
    MarkMediaInfoTargetPaths, MarkMediaInfoVideoDuration,
};

use crate::{ArcPathBuf, Config, Result, i18n::logs};
use cache::CacheState;
use rayon::prelude::*;
use std::{collections::HashMap, path::Path};

/// Extracts and caches a media information.
///
/// User-defined settings from [`MediaInfo::cfg`] take precedence over extracted values.
#[derive(Debug)]
#[non_exhaustive]
pub struct MediaInfo<'a> {
    pub cfg: &'a Config,
    pub cache: MediaInfoCache,
    /// Job number. Separates access to temp files.
    pub job: u8,
}

impl MediaInfo<'_> {
    pub fn new<'a>(cfg: &'a Config, job: u8) -> MediaInfo<'a> {
        MediaInfo {
            cfg,
            cache: Default::default(),
            job,
        }
    }

    /// Clears caches.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.cache.of_group = MediaInfoCacheOfGroup::default();
        self.cache.of_files.clear();
    }

    /// Tries insert a `src` file into the cache.
    ///
    /// Returns `Ok` if [`MediaInfoCacheOfFile::streams`] for `src` is already cached.
    ///
    /// # Errors
    ///
    /// - Fail build [`MediaInfoCacheOfFile::streams`].
    pub fn try_insert(&mut self, src: impl Into<ArcPathBuf> + AsRef<Path>) -> Result<()> {
        match self.try_key_cache_to_insert(src, true, false) {
            Ok(Some((key, cache))) => {
                self.cache.of_files.insert(key, cache);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Tries insert multiple `sources` files into the cache.
    ///
    /// Returns `Ok` if [`MediaInfoCacheOfFile::streams`] for `sources` is already cached.
    ///
    /// # Errors
    ///
    /// - **Only if** [`MediaInfo::cfg`] `exit_on_err` is `true`.
    ///
    /// - Fail build [`MediaInfoCacheOfFile::streams`] for a source.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn) and [`MediaInfo::cfg`] `exit_on_err` is `false`.
    ///
    /// - Warning: Fail build [`MediaInfoCacheOfFile::streams`] for a source.
    pub fn try_insert_many(
        &mut self,
        sources: impl IntoParallelIterator<Item = impl Into<ArcPathBuf> + AsRef<Path>>,
    ) -> Result<()> {
        let exit_on_err = self.cfg.exit_on_err;
        let cache_is_empty = self.cache.of_files.is_empty();

        let map = sources
            .into_par_iter()
            .filter_map(|src| {
                match self.try_key_cache_to_insert(src, exit_on_err, cache_is_empty) {
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
        src: impl Into<ArcPathBuf> + AsRef<Path>,
        exit_on_err: bool,
        cache_is_empty: bool,
    ) -> Result<Option<(ArcPathBuf, MediaInfoCacheOfFile)>> {
        let m = src.as_ref();

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
                let mut cache = MediaInfoCacheOfFile::default();
                cache.streams = CacheState::Cached(streams);
                Ok(Some((src.into(), cache)))
            }
            Err(e) if exit_on_err => Err(e),
            Err(e) => {
                logs::warn_not_recognized_media(m, e);
                Ok(None)
            }
        }
    }
}
