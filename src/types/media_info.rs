mod builders;
pub(crate) mod cache;
mod getters;
mod mkvinfo;
mod mkvmerge_args;
pub(crate) mod set_get_field;

use crate::{
    ArcPathBuf, MCInput, MCOffOnPro, MCTools, MIAttachsInfo, MuxConfig, MuxError, OffOnPro, Tools,
    i18n::logs,
};
use cache::{CacheMI, CacheMIOfFile, CacheState};
use set_get_field::{MIMkvmergeI, MISavedTracks};
use std::{ffi::OsString, path::Path, sync::Arc};

/// Extracts and caches media information.
///
/// User-defined settings from `MuxConfig` take precedence over extracted values.
pub struct MediaInfo<'a> {
    pub mc: &'a MuxConfig,
    pub off_on_pro: &'a OffOnPro,
    tools: &'a Tools,
    upmost: &'a Path,
    cache: CacheMI,
}

impl<'a> From<&'a MuxConfig> for MediaInfo<'a> {
    fn from(mc: &'a MuxConfig) -> Self {
        let off_on_pro = mc.get::<MCOffOnPro>();
        let tools = mc.get::<MCTools>();
        let upmost = mc.get::<MCInput>().get_upmost();
        Self {
            mc,
            off_on_pro,
            tools,
            upmost,
            cache: CacheMI::default(),
        }
    }
}

impl MediaInfo<'_> {
    /// Clears `MediaInfo` cache.
    pub fn clear(&mut self) {
        self.cache = CacheMI::default();
    }

    /// Returns the length cache of files.
    pub fn len(&self) -> usize {
        self.cache.of_files.len()
    }

    /// Returns `true` if cache empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns `true` if cache of files empty.
    pub fn is_no_files(&self) -> bool {
        self.cache.of_files.is_empty()
    }

    /// Returns cache reference.
    pub fn get_cache(&self) -> &CacheMI {
        &self.cache
    }

    /// Returns cache. Resets `self` cache to auto.
    pub fn take_cache(&mut self) -> CacheMI {
        std::mem::take(&mut self.cache)
    }

    /// Updates cache.
    pub fn upd_cache(&mut self, cache: CacheMI) {
        self.cache = cache;
    }

    /// Updates common stem.
    pub fn upd_cmn_stem(&mut self, stem: Arc<OsString>) {
        self.cache.common.stem = CacheState::Cached(stem);
    }

    /// Attempts to insert a single `path` into the cache.
    ///
    /// Returns an error if retrieving mkvmerge info fails.
    pub fn try_insert_path(&mut self, path: ArcPathBuf) -> Result<(), MuxError> {
        if let None = self.cache.of_files.get(&path) {
            self.cache
                .of_files
                .insert(path.clone(), CacheMIOfFile::default());
        }

        if let Err(e) = self.try_get::<MIMkvmergeI>(&path) {
            self.cache.of_files.remove(&path);
            return Err(e);
        }

        Ok(())
    }

    /// Attempts to insert multiple `paths` into the cache.
    /// Skips paths that contain no tracks or attachments marked for saving.
    ///
    /// Returns an error immediately if `exit_on_err` is `true` and retrieving mkvmerge info fails.
    ///
    /// # Logging
    ///
    /// Emits warnings to `stderr` **only if** `MuxLogger` is initialized with at least `LevelFilter::Warn`.
    ///
    /// Warnings are logged in the following cases:
    ///
    /// 1. No tracks or attachments marked for saving were found in the media.
    /// 2. Retrieving mkvmerge info fails and `exit_on_err` is `false`.
    pub fn try_insert_paths_with_filter(
        &mut self,
        paths: impl IntoIterator<Item = ArcPathBuf>,
        exit_on_err: bool,
    ) -> Result<(), MuxError> {
        for path in paths {
            match self.try_insert_path(path.clone()) {
                Ok(()) if !self.save_any_track_or_attach(&path) => {
                    logs::warn_not_saved_track_or_attach(&path);
                    self.cache.of_files.remove(&path);
                }
                Err(e) if exit_on_err => return Err(e),
                Err(e) if !exit_on_err => logs::warn_not_recognized_media(&path, e),
                _ => {}
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn save_any_track_or_attach(&mut self, path: &Path) -> bool {
        self.get::<MISavedTracks>(path)
            .map_or(false, |saved| saved.values().flatten().next().is_some())
            || self
                .get::<MIAttachsInfo>(path)
                .map_or(false, |ai| !ai.is_empty())
    }
}
