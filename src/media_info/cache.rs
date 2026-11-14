mod caching;
pub(crate) mod durations;
pub(crate) mod streams_order;

use crate::{ArcPathBuf, FxHashMap, IsDefault, Result, Stream, config::PathConfigs};
use durations::VideoAndAudioDurations;
use std::{ffi::OsString, mem};
use streams_order::StreamsOrder;

/// A state of cache field.
#[derive(Clone, Debug, Default, IsDefault)]
pub enum CacheState<T> {
    #[default]
    NotCached,
    Cached(T),
    Failed,
}
use CacheState::*;

/// A cache of [`MediaInfo`](crate::MediaInfo).
#[derive(Clone, Debug, Default)]
pub struct MediaInfoCache<'a> {
    pub of_group: MediaInfoCacheOfGroup,
    pub of_files: FxHashMap<ArcPathBuf, MediaInfoCacheOfFile<'a>>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) common for stem-grouped files.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct MediaInfoCacheOfGroup {
    pub shortest_file_stem: CacheState<OsString>,
    pub streams_order: CacheState<StreamsOrder>,
}

/// A cache of [`MediaInfo`](crate::MediaInfo) is separate for each file.
#[derive(Clone, Debug, Default, IsDefault)]
#[non_exhaustive]
pub struct MediaInfoCacheOfFile<'a> {
    pub path_configs: Option<PathConfigs<'a>>,
    pub streams: CacheState<Vec<Stream>>,
    pub path_tail: CacheState<String>,
    pub relative_upmost: CacheState<String>,
    pub video_and_audio_durations: CacheState<VideoAndAudioDurations>,
}

impl<T> CacheState<T> {
    pub(crate) fn set(&mut self, result: Result<T>) -> Result<&mut T> {
        match result {
            Ok(v) => {
                *self = Cached(v);
                match self {
                    Cached(v) => Ok(v),
                    _ => unreachable!(),
                }
            }
            Err(e) => {
                *self = Failed;
                Err(e)
            }
        }
    }

    pub(crate) fn try_get(&self) -> Result<&T> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
        }
    }

    pub(crate) fn try_mut(&mut self) -> Result<&mut T> {
        match self {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
        }
    }

    pub(crate) fn try_take(&mut self) -> Result<T> {
        match mem::take(self) {
            CacheState::Cached(val) => Ok(val),
            CacheState::NotCached => Err(err!("not cached any")),
            CacheState::Failed => Err(err!("previously failed")),
        }
    }

    pub(crate) const fn is_cached(&self) -> bool {
        matches!(self, CacheState::Cached(_))
    }
}
