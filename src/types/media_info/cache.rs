mod attach;
mod track;

use super::mkvinfo::Mkvinfo;
use crate::{AttachID, AttachType, IsDefault, LangCode, Target, TargetGroup, TrackID, TrackType};
use enum_map::EnumMap;
use std::collections::{BTreeSet, HashMap};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}

#[derive(Clone, Default)]
pub struct CacheMI {
    pub common: CacheMICommon,
    pub of_files: HashMap<PathBuf, CacheMIOfFile>,
}

impl CacheMI {
    pub fn is_empty(&self) -> bool {
        self.of_files.is_empty() && self.common.is_default()
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct CacheMICommon {
    pub stem: CacheState<OsString>,
}

#[derive(Clone, Default)]
pub struct CacheMIOfFile {
    pub char_encoding: CacheState<String>,
    pub mkvinfo: CacheState<Mkvinfo>,
    pub mkvmerge_i: CacheState<Vec<String>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub saved_tracks: CacheState<EnumMap<TrackType, BTreeSet<u64>>>,
    pub target_group: CacheState<TargetGroup>,
    pub targets: CacheState<[Target; 3]>,

    pub tracks_info: CacheState<HashMap<u64, CacheMIOfFileTrack>>,
    pub attachs_info: CacheState<HashMap<u64, CacheMIOfFileAttach>>,
}

#[derive(Clone, Default)]
pub struct CacheMIOfFileTrack {
    pub mkvinfo_cutted: Option<Mkvinfo>,
    pub mkvmerge_id_line: String,
    pub track_type: TrackType,
    pub lang: CacheState<LangCode>,
    pub name: CacheState<String>,
    pub track_ids: CacheState<[TrackID; 2]>,
}

#[derive(Clone)]
pub struct CacheMIOfFileAttach {
    pub id: AttachID,
    pub attach_type: AttachType,
    pub mkvmerge_id_line: String,
}
