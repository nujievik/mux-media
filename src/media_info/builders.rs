mod streams;

use super::*;
use crate::{Result, StreamsOrder, helpers};
use std::{ffi::OsString, path::Path};

impl MediaInfo<'_> {
    pub(super) fn build_stem(&self) -> Result<OsString> {
        let shortest = self
            .cache
            .of_files
            .iter()
            .filter_map(|(p, _)| p.file_stem())
            .min_by_key(|s| s.len())
            .ok_or_else(|| err!("Not found any file_stem()"))?;

        Ok(shortest.to_owned())
    }

    pub(super) fn build_streams_order(&mut self) -> Result<StreamsOrder> {
        StreamsOrder::new(self)
    }

    pub(crate) fn build_path_tail(&mut self, src: &Path) -> Result<String> {
        let cmn_stem = self.try_get_cmn(MarkMediaInfoStem)?;
        src.file_stem()
            .ok_or_else(|| err!("Path has not file_stem()"))
            .and_then(|stem| {
                helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(crate) fn build_relative_upmost(&self, src: &Path) -> Result<String> {
        src.parent()
            .ok_or_else(|| err!("Path has not parent()"))
            .and_then(|parent| {
                helpers::os_str_tail(self.cfg.input.dir().as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(super) fn build_durations(&mut self, src: &Path) -> Result<VideoAndAudioDurations> {
        VideoAndAudioDurations::try_from_media_file(self, src)
    }
}
