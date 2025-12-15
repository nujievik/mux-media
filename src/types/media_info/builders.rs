mod durations;
mod streams;

use super::MediaInfo;
use crate::{CharEncoding, Extension, Result, StreamsOrder, Target, markers::*, types::helpers};
use std::{ffi::OsString, path::Path};

impl MediaInfo<'_> {
    pub(super) fn build_stem(&self) -> Result<OsString> {
        let shortest = self
            .cache
            .of_files
            .iter()
            .filter_map(|(p, _)| p.file_stem())
            .min_by_key(|s| s.len())
            .ok_or("Not found any file_stem()")?;

        Ok(shortest.to_owned())
    }

    pub(super) fn build_streams_order(&mut self) -> Result<StreamsOrder> {
        StreamsOrder::new(self)
    }

    pub(crate) fn build_path_tail(&mut self, src: &Path) -> Result<String> {
        let cmn_stem = self.try_get_cmn(MICmnStem)?;
        src.file_stem()
            .ok_or_else(|| err!("Path '{}' has not file_stem()", src.display()))
            .and_then(|stem| {
                helpers::os_str_tail(cmn_stem, stem).map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(crate) fn build_relative_upmost(&self, src: &Path) -> Result<String> {
        src.parent()
            .ok_or_else(|| err!("Path '{}' has not parent()", src.display()))
            .and_then(|parent| {
                helpers::os_str_tail(self.cfg.input.dir.as_os_str(), parent.as_os_str())
                    .map(|os| os.to_string_lossy().into_owned())
            })
    }

    pub(super) fn build_sub_char_encoding(&self, src: &Path) -> Result<CharEncoding> {
        if src.extension().map_or(false, |ext| {
            Extension::new_and_is_subs(ext.as_encoded_bytes())
        }) {
            Ok(CharEncoding::new(src))
        } else {
            Err(err!("Is not subtitle file"))
        }
    }

    pub(super) fn build_target_paths(&self, src: &Path) -> Result<Vec<Target>> {
        let mut targets = Vec::<Target>::new();

        if let Some(trg) = self.cfg.get_key(src) {
            targets.push(trg);
        }

        if let Some(trg) = src.parent().and_then(|p| self.cfg.get_key(p)) {
            targets.push(trg);
        }

        Ok(targets)
    }
}
