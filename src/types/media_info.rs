mod builders;
pub(crate) mod cache;
mod getters;
mod mkvmerge_args;
pub(crate) mod set_get_field;

use crate::{
    ArcPathBuf, IsDefault, MuxConfig, MuxError, ProFlags, Tools,
    i18n::logs,
    markers::{MCInput, MCProFlags, MCTools, MIAttachsInfo, MIMkvmergeI, MISavedTracks},
};
use cache::{CacheMI, CacheMICommon, CacheMIOfFile, CacheMIOfGroup, CacheState};
use std::{ffi::OsString, path::Path, sync::Arc};

/// Extracts and caches media information.
///
/// User-defined settings from `MuxConfig` take precedence over extracted values.
pub struct MediaInfo<'a> {
    pub mc: &'a MuxConfig,
    pub pro_flags: &'a ProFlags,
    tools: &'a Tools,
    upmost: &'a Path,
    cache: CacheMI,
}

impl<'a> From<&'a MuxConfig> for MediaInfo<'a> {
    fn from(mc: &'a MuxConfig) -> Self {
        let pro_flags = mc.get::<MCProFlags>();
        let tools = mc.get::<MCTools>();
        let upmost = mc.get::<MCInput>().get_dir();
        Self {
            mc,
            pro_flags,
            tools,
            upmost,
            cache: CacheMI::default(),
        }
    }
}

impl MediaInfo<'_> {
    /// Clears cache.
    pub fn clear(&mut self) {
        self.cache = CacheMI::default();
    }

    /// Clears cache of current group and files.
    pub fn clear_current(&mut self) {
        self.cache.of_group = CacheMIOfGroup::default();
        self.cache.of_files.clear();
    }

    /// Clears common cache.
    pub fn clear_common(&mut self) {
        self.cache.common = CacheMICommon::default();
    }

    /// Clears cache of current group.
    pub fn clear_group(&mut self) {
        self.cache.of_group = CacheMIOfGroup::default();
    }

    /// Clears cache of current files.
    pub fn clear_files(&mut self) {
        self.cache.of_files.clear();
    }

    /// Returns the length cache of files.
    pub fn len(&self) -> usize {
        self.cache.of_files.len()
    }

    /// Returns `true` if not cached any.
    pub fn is_empty(&self) -> bool {
        self.cache.of_group.is_default() && self.cache.of_files.is_empty()
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

    /// Updates stem for stem-grouped media.
    pub fn upd_group_stem(&mut self, stem: Arc<OsString>) {
        self.cache.of_group.stem = CacheState::Cached(stem);
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
