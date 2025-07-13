mod attach;
mod track;

use super::mkvinfo::Mkvinfo;
use crate::{
    AttachID, AttachType, IsDefault, LangCode, SubCharset, Target, TargetGroup, TrackID, TrackType,
};
use enum_map::EnumMap;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    path::PathBuf,
};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}

/// Cache of `MediaInfo`.
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

/// Cache of `MediaInfo` is common for all media.
#[derive(Clone, Default, PartialEq)]
pub struct CacheMICommon {
    pub stem: CacheState<OsString>,
}

/// Cache of `MediaInfo` is separate for each media.
#[derive(Clone, Default)]
pub struct CacheMIOfFile {
    pub mkvinfo: CacheState<Mkvinfo>,
    pub mkvmerge_i: CacheState<Vec<String>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub saved_tracks: CacheState<EnumMap<TrackType, BTreeSet<u64>>>,
    pub sub_charset: CacheState<SubCharset>,
    pub target_group: CacheState<TargetGroup>,
    pub targets: CacheState<[Target; 3]>,

    pub tracks_info: CacheState<HashMap<u64, CacheMIOfFileTrack>>,
    pub attachs_info: CacheState<HashMap<u64, CacheMIOfFileAttach>>,
}

/// Cache of `MediaInfo` is separate for each track in media.
#[derive(Clone, Default)]
pub struct CacheMIOfFileTrack {
    pub mkvinfo_cutted: Option<Mkvinfo>,
    pub mkvmerge_id_line: String,
    pub track_type: TrackType,
    pub lang: CacheState<LangCode>,
    pub name: CacheState<String>,
    pub track_ids: CacheState<[TrackID; 2]>,
}

/// Cache of `MediaInfo` is separate for each attach in media.
#[derive(Clone)]
pub struct CacheMIOfFileAttach {
    pub id: AttachID,
    pub attach_type: AttachType,
    pub mkvmerge_id_line: String,
}
