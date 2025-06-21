mod builders;
pub(crate) mod cache;
mod getters;
mod mkvinfo;
mod mkvmerge_args;
pub(crate) mod set_get_field;

use crate::{MCInput, MCOffOnPro, MCTools, MIAttachsInfo, MuxConfig, MuxError, OffOnPro, Tools};
use cache::{CacheMI, CacheMICommon, CacheState};
use log::warn;
use set_get_field::{MIMkvmergeI, MISavedTracks};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

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
    pub fn clear(&mut self) {
        self.cache.common = CacheMICommon::default();
        self.cache.of_files.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.of_files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn is_no_files(&self) -> bool {
        self.cache.of_files.is_empty()
    }

    pub fn get_cache(&self) -> &CacheMI {
        &self.cache
    }

    pub fn take_cache(&mut self) -> CacheMI {
        std::mem::take(&mut self.cache)
    }

    pub fn upd_cache(&mut self, cache: CacheMI) {
        self.cache = cache;
    }

    pub fn upd_cmn_stem(&mut self, stem: OsString) {
        self.cache.common.stem = CacheState::Cached(stem);
    }

    pub fn try_insert(&mut self, path: impl AsRef<Path>) -> Result<(), MuxError> {
        let _ = self.try_get::<MIMkvmergeI>(path.as_ref())?;
        Ok(())
    }

    pub fn try_insert_paths_with_filter(
        &mut self,
        paths: &[PathBuf],
        exit_on_err: bool,
    ) -> Result<(), MuxError> {
        for path in paths {
            match self.try_insert(&path) {
                Ok(()) if !self.save_any_track_or_attach(path) => {
                    warn!(
                        "Not found any save Track or Attach in media '{}'. Skipping",
                        path.display()
                    );
                    self.cache.of_files.remove(path);
                }
                Err(e) if exit_on_err => return Err(e),
                Err(e) if !exit_on_err => {
                    warn!("Unrecognized file '{}': {}. Skipping", path.display(), e);
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn save_any_track_or_attach(&mut self, path: &Path) -> bool {
        self.get::<MISavedTracks>(path)
            .map_or(false, |saved| saved.values().next().is_some())
            || self
                .get::<MIAttachsInfo>(path)
                .map_or(false, |ai| !ai.is_empty())
    }
}
