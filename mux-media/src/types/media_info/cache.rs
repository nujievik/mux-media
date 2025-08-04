pub(crate) mod attach;
pub(crate) mod track;

use crate::{
    ArcPathBuf, IsDefault, MuxError, SubCharset, Target, TargetGroup, TrackOrder, TrackType,
};
use attach::CacheMIOfFileAttach;
use enum_map::EnumMap;
use matroska::Matroska;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    mem,
    path::PathBuf,
};
use track::CacheMIOfFileTrack;

/// A state of cache field.
#[derive(Clone, Default, Debug, IsDefault)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed(MuxError),
}

/// Cache of [`MediaInfo`](crate::MediaInfo).
#[derive(Clone, Default, IsDefault)]
pub struct CacheMI {
    pub common: CacheMICommon,
    pub of_group: CacheMIOfGroup,
    pub of_files: HashMap<ArcPathBuf, CacheMIOfFile>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) is common for all.
#[derive(Clone, Default, IsDefault)]
pub struct CacheMICommon {
    pub external_fonts: CacheState<Vec<PathBuf>>,
    pub regex_attach_id: CacheState<Regex>,
    pub regex_codec: CacheState<Regex>,
    pub regex_track_id: CacheState<Regex>,
    pub regex_word: CacheState<Regex>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) common for stem-grouped media.
#[derive(Clone, Default, IsDefault)]
pub struct CacheMIOfGroup {
    pub stem: CacheState<OsString>,
    pub track_order: CacheState<TrackOrder>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) is separate for each media.
#[derive(Clone, Default, IsDefault)]
pub struct CacheMIOfFile {
    pub matroska: CacheState<Matroska>,
    pub mkvmerge_i: CacheState<Vec<String>>,

    pub path_tail: CacheState<String>,
    pub words_path_tail: CacheState<Vec<String>>,
    pub relative_upmost: CacheState<String>,
    pub words_relative_upmost: CacheState<Vec<String>>,

    pub saved_tracks: CacheState<EnumMap<TrackType, BTreeSet<u64>>>,
    pub sub_charset: CacheState<SubCharset>,
    pub target_group: CacheState<TargetGroup>,
    pub targets: CacheState<Vec<Target>>,

    pub tracks_info: CacheState<HashMap<u64, CacheMIOfFileTrack>>,
    pub attachs_info: CacheState<HashMap<u64, CacheMIOfFileAttach>>,
}

impl<T> CacheState<T> {
    pub(crate) fn try_get(&self) -> Result<&T, MuxError> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(e.clone()),
        }
    }

    pub(crate) const fn get(&self) -> Option<&T> {
        match self {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn try_mut(&mut self) -> Result<&mut T, MuxError> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(e.clone()),
        }
    }

    pub(crate) const fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn try_take(&mut self) -> Result<T, MuxError> {
        match mem::take(self) {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(e.clone()),
        }
    }

    pub(crate) fn take(&mut self) -> Option<T> {
        match mem::take(self) {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }
}
