use crate::{
    ArcPathBuf, CharEncoding, FxHashMap, IsDefault, Result, Stream, StreamsOrder, Target, Time,
};
use std::{ffi::OsString, mem};

/// A state of cache field.
#[derive(Clone, Debug, Default, IsDefault)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}

/// A cache of [`MediaInfo`](crate::MediaInfo).
#[derive(Clone, Debug, Default)]
pub struct MediaInfoCache {
    pub of_group: MediaInfoCacheOfGroup,
    pub of_files: FxHashMap<ArcPathBuf, MediaInfoCacheOfFile>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) common for stem-grouped files.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct MediaInfoCacheOfGroup {
    pub stem: CacheState<OsString>,
    pub streams_order: CacheState<StreamsOrder>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) is separate for each file.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct MediaInfoCacheOfFile {
    pub streams: CacheState<Vec<Stream>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub sub_char_encoding: CacheState<CharEncoding>,

    /// Targets from file path and parent path, existed in [`Config::targets`](
    /// crate::Config::config_targets).
    pub target_paths: CacheState<Vec<Target>>,

    pub audio_duration: CacheState<Time>,
    pub video_duration: CacheState<Time>,
    pub playable_duration: CacheState<Time>,
}

impl IsDefault for MediaInfoCache {
    fn is_default(&self) -> bool {
        self.of_group.is_default() && self.of_files.is_empty()
    }
}

impl<T> CacheState<T> {
    pub(crate) fn convert_result(result: Result<T>) -> (CacheState<T>, Result<()>) {
        match result {
            Ok(v) => (CacheState::Cached(v), Ok(())),
            Err(e) => (CacheState::Failed, Err(e)),
        }
    }

    pub(crate) fn try_get(&self) -> Result<&T> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
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
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
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
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
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
