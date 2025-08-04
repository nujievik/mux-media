mod builders;
pub(crate) mod cache;
pub(crate) mod mut_fields;

use crate::{
    ArcPathBuf, AutoFlags, IsDefault, MuxConfig, MuxError, Tools,
    i18n::logs,
    markers::{
        MCAutoFlags, MCInput, MCTools, MIAttachsInfo, MIMatroska, MIMkvmergeI, MISavedTracks,
    },
};
use cache::{CacheMI, CacheMICommon, CacheMIOfFile, CacheMIOfGroup};
use std::{mem, path::Path};

/// Extracts and caches media information.
///
/// User-defined settings from [`MuxConfig`] take precedence over extracted values.
pub struct MediaInfo<'a> {
    pub mux_config: &'a MuxConfig,
    auto_flags: &'a AutoFlags,
    tools: &'a Tools,
    upmost: &'a Path,
    cache: CacheMI,
}

impl<'a> From<&'a MuxConfig> for MediaInfo<'a> {
    fn from(mux_config: &'a MuxConfig) -> Self {
        let auto_flags = mux_config.field::<MCAutoFlags>();
        let tools = mux_config.field::<MCTools>();
        let upmost = mux_config.field::<MCInput>().dir();
        Self {
            mux_config,
            auto_flags,
            tools,
            upmost,
            cache: CacheMI::default(),
        }
    }
}

impl MediaInfo<'_> {
    /// Returns the cache value.
    #[inline(always)]
    pub fn cache(&self) -> &CacheMI {
        &self.cache
    }

    /// Replaces the cache to default, returning current value.
    #[inline(always)]
    pub fn take_cache(&mut self) -> CacheMI {
        mem::take(&mut self.cache)
    }

    /// Sets the cache.
    #[inline(always)]
    pub fn set_cache(&mut self, cache: CacheMI) {
        self.cache = cache;
    }

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
        self.cache.of_files.is_empty()
    }

    /// Attempts to insert a `path` into the cache.
    ///
    /// Returns an error if retrieving either matroska or mkvmerge info fails.
    pub fn try_insert(&mut self, path: impl Into<ArcPathBuf>) -> Result<(), MuxError> {
        let path = path.into();

        if let None = self.cache.of_files.get(&path) {
            self.cache
                .of_files
                .insert(path.clone(), CacheMIOfFile::default());
        }

        if let Some(()) = self.init::<MIMatroska>(&path) {
            return Ok(());
        }

        if let Err(e) = self.try_init::<MIMkvmergeI>(&path) {
            self.cache.of_files.remove(&path);
            return Err(e);
        }

        Ok(())
    }

    /// Attempts to insert multiple `paths` into the cache.
    ///
    /// Skips paths that contain no tracks or attachments marked for saving.
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
    ///   log::LevelFilter::Warn).
    ///
    /// - No tracks or attachments marked for saving were found in the media.
    ///
    /// - Retrieving either matroska or mkvmerge info fails and `exit_on_err` is `false`.
    pub fn try_insert_many_filtered(
        &mut self,
        paths: impl IntoIterator<Item = impl Into<ArcPathBuf>>,
        exit_on_err: bool,
    ) -> Result<(), MuxError> {
        for path in paths {
            let path = path.into();

            match self.try_insert(path.clone()) {
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
    pub(crate) fn auto_flags(&self) -> &AutoFlags {
        self.auto_flags
    }

    #[inline(always)]
    pub(crate) fn tools(&self) -> &Tools {
        self.tools
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
