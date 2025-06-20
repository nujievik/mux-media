mod ai_cache;
mod builders;
mod getters;
mod mkvinfo;
mod mkvmerge_args;
pub(crate) mod set_get_field;
mod ti_cache;

use crate::{
    AttachID, AttachType, LangCode, MCInput, MCOffOnPro, MCTools, MIAttachsInfo, MITracksInfo,
    MuxConfig, MuxError, OffOnPro, Target, TargetGroup, Tools, TrackType,
};
use enum_map::EnumMap;
use log::warn;
use mkvinfo::Mkvinfo;
use set_get_field::{MIMkvmergeI, MISavedTracks};
use std::collections::{BTreeSet, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct MediaInfo<'a> {
    pub mc: &'a MuxConfig,
    pub off_on_pro: &'a OffOnPro,
    tools: &'a Tools,
    upmost: &'a Path,
    stem: OsString,
    cache: HashMap<PathBuf, MICache>,
}

#[derive(Clone, Default)]
pub struct MICache {
    char_encoding: CacheState<String>,
    mkvinfo: CacheState<Mkvinfo>,
    mkvmerge_i: CacheState<Vec<String>>,
    path_tail: CacheState<String>,
    relative_upmost: CacheState<String>,
    target_group: CacheState<TargetGroup>,
    targets: CacheState<[Target; 3]>,

    tracks_info: CacheState<HashMap<u64, TICache>>,
    saved_tracks: CacheState<EnumMap<TrackType, BTreeSet<u64>>>,
    attachs_info: CacheState<HashMap<AttachID, AICache>>,
}

#[derive(Clone, Default)]
pub struct TICache {
    pub num: u64,
    pub track_type: TrackType,
    pub mkvmerge_id_line: String,
    lang: CacheState<LangCode>,
    mkvinfo_cutted: Option<Mkvinfo>,
    name: CacheState<String>,
}

#[derive(Clone, Default)]
pub struct AICache {
    pub num: u64,
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
        let off_on_pro = mc.get::<MCOffOnPro>();
        let tools = mc.get::<MCTools>();
        let upmost = mc.get::<MCInput>().get_upmost();
        Self {
            mc,
            off_on_pro,
            tools,
            upmost,
            cache: HashMap::new(),
            stem: OsString::new(),
        }
    }
}

impl MediaInfo<'_> {
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn len_cache(&self) -> usize {
        self.cache.len()
    }

    pub fn get_cache(&self) -> &HashMap<PathBuf, MICache> {
        &self.cache
    }

    pub fn take_cache(&mut self) -> HashMap<PathBuf, MICache> {
        std::mem::take(&mut self.cache)
    }

    pub fn upd_cache(&mut self, cache: HashMap<PathBuf, MICache>) {
        self.cache = cache;
    }

    pub fn upd_stem(&mut self, stem: impl Into<OsString>) {
        self.stem = stem.into()
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
                let _ = self.try_get::<MISavedTracks>(&path);
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
