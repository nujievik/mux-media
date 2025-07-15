pub(crate) mod attach;
pub(crate) mod track;

use crate::{ArcPathBuf, IsDefault, Mkvinfo, SubCharset, Target, TargetGroup, TrackType};
use attach::CacheMIOfFileAttach;
use enum_map::EnumMap;
use regex::Regex;
use smallvec::SmallVec;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    sync::Arc,
};
use track::CacheMIOfFileTrack;

/// A state of cache field.
#[derive(Clone, Default, Debug)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}

/// Cache of [`crate::MediaInfo`].
#[derive(Clone, Default)]
pub struct CacheMI {
    pub common: CacheMICommon,
    pub of_group: CacheMIOfGroup,
    pub of_files: HashMap<ArcPathBuf, CacheMIOfFile>,
}

impl IsDefault for CacheMI {
    fn is_default(&self) -> bool {
        self.of_group.is_default() && self.of_files.is_empty()
    }
}

/// Cache of [`crate::MediaInfo`] is common for all.
#[derive(Clone, Default)]
pub struct CacheMICommon {
    pub regex_aid: CacheState<Regex>,
    pub regex_tid: CacheState<Regex>,
    pub regex_word: CacheState<Regex>,
}

/// Cache of [`crate::MediaInfo`] common for stem-grouped media.
#[derive(Clone, Default)]
pub struct CacheMIOfGroup {
    pub stem: CacheState<Arc<OsString>>,
}

/// Cache of [`crate::MediaInfo`] is separate for each media.
#[derive(Clone, Default)]
pub struct CacheMIOfFile {
    pub mkvinfo: CacheState<Mkvinfo>,
    pub mkvmerge_i: CacheState<Vec<String>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub saved_tracks: CacheState<EnumMap<TrackType, BTreeSet<u64>>>,
    pub sub_charset: CacheState<SubCharset>,
    pub target_group: CacheState<TargetGroup>,
    pub targets: CacheState<SmallVec<[Target; 3]>>,

    pub tracks_info: CacheState<HashMap<u64, CacheMIOfFileTrack>>,
    pub attachs_info: CacheState<HashMap<u64, CacheMIOfFileAttach>>,
}

impl<T> IsDefault for CacheState<T> {
    fn is_default(&self) -> bool {
        match self {
            Self::NotCached => true,
            _ => false,
        }
    }
}

impl IsDefault for CacheMIOfGroup {
    fn is_default(&self) -> bool {
        self.stem.is_default()
    }
}
