mod ai_cache;
mod builders;
mod getters;
mod mkvinfo;
mod mkvmerge_args;
pub(crate) mod set_get_path_field;
mod ti_cache;

use crate::{
    AttachID, AttachType, LangCode, MCInput, MCTools, MIAttachsInfo, MITracksInfo, MuxConfig,
    MuxError, TFlagsCounts, Target, TargetGroup, Tools, TrackID, TrackType,
};
use log::warn;
use mkvinfo::Mkvinfo;
use set_get_path_field::{MIMkvmergeI, MISavedAudioU32IDs};
use std::collections::{BTreeSet, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct MediaInfo<'a> {
    pub mc: &'a MuxConfig,
    pub counts: TFlagsCounts,
    cache: HashMap<PathBuf, MICache>,
    stem: OsString,
    tools: &'a Tools,
    upmost: &'a Path,
}

#[derive(Clone, Default)]
pub struct MICache {
    char_encoding: CacheState<String>,
    mkvinfo: CacheState<Mkvinfo>,
    mkvmerge_i: CacheState<Vec<String>>,
    target_group: CacheState<TargetGroup>,
    targets: CacheState<[Target; 3]>,

    tracks_info: CacheState<HashMap<TrackID, TICache>>,
    saved_audio_u32_ids: CacheState<BTreeSet<u32>>,
    saved_sub_u32_ids: CacheState<BTreeSet<u32>>,
    saved_video_u32_ids: CacheState<BTreeSet<u32>>,
    saved_button_u32_ids: CacheState<BTreeSet<u32>>,

    path_tail: CacheState<String>,
    relative_upmost: CacheState<String>,

    attachs_info: CacheState<HashMap<AttachID, AICache>>,
}

#[derive(Clone, Default)]
pub struct TICache {
    pub id_u32: u32,
    pub track_type: TrackType,
    pub mkvmerge_id_line: String,
    mkvinfo_cutted: Option<Mkvinfo>,
    lang: CacheState<LangCode>,
    name: CacheState<String>,
}

#[derive(Clone, Default)]
pub struct AICache {
    pub id_u32: u32,
    pub attach_type: AttachType,
    pub mkvmerge_id_line: String,
}

#[derive(Clone, Default)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}

impl<'a> From<&'a MuxConfig> for MediaInfo<'a> {
    fn from(mc: &'a MuxConfig) -> Self {
        let tools = mc.get::<MCTools>();
        let upmost = mc.get::<MCInput>().get_upmost();
        Self {
            mc,
            tools,
            upmost,
            cache: HashMap::new(),
            counts: TFlagsCounts::default(),
            stem: OsString::new(),
        }
    }
}

impl MediaInfo<'_> {
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn len_cache(&self) -> usize {
        self.cache.len()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear()
    }

    pub fn take_cache(&mut self) -> HashMap<PathBuf, MICache> {
        std::mem::take(&mut self.cache)
    }

    pub fn upd_cache(&mut self, cache: HashMap<PathBuf, MICache>) {
        self.cache = cache;
    }

    pub fn clear_counts(&mut self) {
        self.counts = TFlagsCounts::default();
    }

    pub fn upd_stem(&mut self, stem: impl Into<OsString>) {
        self.stem = stem.into()
    }

    pub fn try_insert(&mut self, path: impl AsRef<Path>) -> Result<(), MuxError> {
        let _ = self.try_get::<MIMkvmergeI>(path.as_ref())?;
        Ok(())
    }

    pub fn try_insert_paths_with_empty_filter(
        &mut self,
        paths: &[PathBuf],
        exit_on_err: bool,
    ) -> Result<(), MuxError> {
        for path in paths {
            let mut skip = false;

            if let Err(e) = self.try_insert(&path) {
                if exit_on_err {
                    return Err(e);
                } else {
                    skip = true;
                    warn!("Unrecognized file '{}': {}. Skipping", path.display(), e);
                }
            }

            if !skip {
                let mut empty_ti = false;
                let _ = self.try_get::<MISavedAudioU32IDs>(&path);
                if let Some(ti) = self.get::<MITracksInfo>(&path) {
                    if ti.is_empty() {
                        empty_ti = true;
                    }
                }

                if empty_ti {
                    if let Some(ai) = self.get::<MIAttachsInfo>(&path) {
                        if ai.is_empty() {
                            skip = true;
                        }
                    }
                }

                if skip {
                    warn!(
                        "File '{}' has not any save Track or Attach. Skipping",
                        path.display()
                    );
                    self.cache.remove(path);
                }
            }
        }
        Ok(())
    }
}
