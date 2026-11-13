use super::*;
use crate::Config;
use std::path::Path;

impl<'a> MediaInfoCache<'a> {
    pub(crate) fn caching_file_cache<'b>(
        &'b mut self,
        src: &Path,
    ) -> &'b mut MediaInfoCacheOfFile<'a> {
        if !self.of_files.contains_key(src) {
            self.of_files.insert(src.into(), Default::default());
        }
        match self.of_files.get_mut(src) {
            Some(cache) => cache,
            _ => unreachable!(),
        }
    }
}

impl<'a> MediaInfoCacheOfFile<'a> {
    pub(crate) fn caching_path_configs(&mut self, cfg: &'a Config, src: &Path) -> PathConfigs<'a> {
        *self
            .path_configs
            .get_or_insert_with(|| PathConfigs::new(cfg, src))
    }
}
