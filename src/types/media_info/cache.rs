use crate::{
    ArcPathBuf, CharEncoding, Duration, IsDefault, MuxError, Result, Stream, StreamsOrder, Target,
};
use std::{collections::HashMap, ffi::OsString, mem};

/// A state of cache field.
#[derive(Clone, Debug, Default, IsDefault)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed(Box<MuxError>),
}

/// A cache of [`MediaInfo`](crate::MediaInfo).
#[derive(Clone, Debug, Default, IsDefault)]
pub struct CacheMI {
    pub of_group: CacheMIOfGroup,
    pub of_files: HashMap<ArcPathBuf, CacheMIOfFile>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) common for stem-grouped files.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct CacheMIOfGroup {
    pub stem: CacheState<OsString>,
    pub streams_order: CacheState<StreamsOrder>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) is separate for each file.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct CacheMIOfFile {
    pub streams: CacheState<Vec<Stream>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub sub_char_encoding: CacheState<CharEncoding>,

    /// Targets from file path and parent path, existed in [`Config::targets`](
    /// crate::Config::targets).
    pub target_paths: CacheState<Vec<Target>>,

    pub audio_duration: CacheState<Duration>,
    pub video_duration: CacheState<Duration>,
    pub playable_duration: CacheState<Duration>,
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

    pub(crate) const fn is_cached(&self) -> bool {
        matches!(self, CacheState::Cached(_))
    }
}
