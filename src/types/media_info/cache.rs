pub(crate) mod attach;
pub(crate) mod track;

use crate::{
    ArcPathBuf, Duration, FfmpegStream, IsDefault, MuxError, Result, SubCharset, Target,
    TargetGroup, TrackOrder, TrackType,
};
use attach::CacheMIOfFileAttach;
use enum_map::EnumMap;
use matroska::Matroska;
use std::{collections::HashMap, ffi::OsString, mem, path::PathBuf, sync::Arc};
use track::CacheMIOfFileTrack;

/// A state of cache field.
#[derive(Clone, Debug, Default, IsDefault)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed(Box<MuxError>),
}

/// Cache of [`MediaInfo`](crate::MediaInfo).
#[derive(Clone, Debug, Default, IsDefault)]
pub struct CacheMI {
    pub common: CacheMICommon,
    pub of_group: CacheMIOfGroup,
    pub of_files: HashMap<ArcPathBuf, CacheMIOfFile>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) is common for all.
#[derive(Clone, Debug, Default, IsDefault)]
pub struct CacheMICommon {
    pub external_fonts: CacheState<Arc<Vec<PathBuf>>>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) common for stem-grouped media.
#[derive(Clone, Debug, Default, IsDefault)]
pub struct CacheMIOfGroup {
    pub stem: CacheState<OsString>,
    pub track_order: CacheState<TrackOrder>,
}

/// Cache of [`MediaInfo`](crate::MediaInfo) is separate for each media.
#[derive(Clone, Debug, Default, IsDefault)]
pub struct CacheMIOfFile {
    pub ffmpeg_streams: CacheState<Vec<FfmpegStream>>,
    pub matroska: CacheState<Matroska>,

    pub path_tail: CacheState<String>,
    pub words_path_tail: CacheState<Vec<String>>,
    pub relative_upmost: CacheState<String>,
    pub words_relative_upmost: CacheState<Vec<String>>,

    pub saved_tracks: CacheState<EnumMap<TrackType, Vec<u64>>>,
    pub sub_charset: CacheState<SubCharset>,
    pub target_group: CacheState<TargetGroup>,
    pub targets: CacheState<Vec<Target>>,

    pub audio_duration: CacheState<Duration>,
    pub video_duration: CacheState<Duration>,
    pub playable_duration: CacheState<Duration>,

    pub tracks_info: CacheState<HashMap<u64, CacheMIOfFileTrack>>,
    pub attachs_info: CacheState<HashMap<u64, CacheMIOfFileAttach>>,
}

impl<T> CacheState<T> {
    pub(crate) fn from_res(res: Result<T>) -> CacheState<T> {
        match res {
            Ok(v) => CacheState::Cached(v),
            Err(e) => CacheState::Failed(Box::new(e)),
        }
    }

    pub(crate) fn try_get(&self) -> Result<&T> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(*e.clone()),
        }
    }

    pub(crate) const fn get(&self) -> Option<&T> {
        match self {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn try_mut(&mut self) -> Result<&mut T> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(*e.clone()),
        }
    }

    pub(crate) const fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }

    pub(crate) fn try_take(&mut self) -> Result<T> {
        match mem::take(self) {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err("Not cached any".into()),
            CacheState::Failed(e) => Err(*e.clone()),
        }
    }

    pub(crate) fn take(&mut self) -> Option<T> {
        match mem::take(self) {
            CacheState::Cached(val) => Some(val),
            _ => None,
        }
    }

    #[inline]
    pub(crate) const fn is_cached(&self) -> bool {
        matches!(self, CacheState::Cached(_))
    }
}
