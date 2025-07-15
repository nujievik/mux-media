use super::MediaInfo;
use super::cache::{CacheMIOfFile, CacheMIOfFileAttach, CacheMIOfFileTrack, CacheState};
use crate::{
    ArcPathBuf, LangCode, Mkvinfo, MuxError, SetGetField, SetGetPathField, SetGetPathTrackField,
    SubCharset, Target, TargetGroup, TrackID, TrackType,
};
use enum_map::EnumMap;
use smallvec::SmallVec;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    path::Path,
    sync::Arc,
};

macro_rules! set_get_fields {
    ($( $field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] common field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl SetGetField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self) -> Result<(), MuxError> {
                let (state, result) = match self.$builder() {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => (CacheState::Failed, Err(e)),
                };

                self.cache.common.$field = state;
                result
            }

            fn try_get(&mut self) -> Result<&Self::FieldType, MuxError> {
                if let CacheState::NotCached = self.cache.common.$field {
                    <Self as SetGetField::<$marker>>::try_set(self)?;
                }

                match &self.cache.common.$field {
                    CacheState::Cached(val) => Ok(val),
                    CacheState::Failed => Err("Previously failed to load".into()),
                    CacheState::NotCached => Err("Unexpected NotCached state".into()),
                }
            }

            fn get(&mut self) -> Option<&Self::FieldType> {
                match <Self as SetGetField::<$marker>>::try_get(self) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut_get(&self) -> Option<&Self::FieldType> {
                match &self.cache.common.$field {
                    CacheState::Cached(val) => Some(val),
                    _ => None,
                }
            }
        }
    )* };
}

macro_rules! set_get_path_fields {
    ($( $map_field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] fields, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl SetGetPathField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self, path: &Path) -> Result<(), MuxError> {
                let (state, result) = match self.$builder(path) {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => (CacheState::Failed, Err(e)),
                };

                match self.cache.of_files.get_mut(path) {
                    Some(fields) => fields.$map_field = state,
                    None => {
                        let mut cache = CacheMIOfFile::default();
                        cache.$map_field = state;
                        self.cache.of_files.insert(ArcPathBuf::from(path), cache);
                    }
                }

                result
            }

            fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError> {
                match self.cache.of_files.get(path).map(|e| &e.$map_field) {
                    Some(CacheState::Cached(_)) | Some(CacheState::Failed) => {}
                    _ => {
                        <Self as SetGetPathField::<$marker>>::try_set(self, path)?;
                    }
                }

                match self.cache.of_files.get(path).map(|e| &e.$map_field) {
                    Some(CacheState::Cached(val)) => Ok(val),
                    Some(CacheState::Failed) => Err("Previously failed to load".into()),
                    _ => Err("Cache entry missing".into()),
                }
            }

            fn get(&mut self, path: &Path) -> Option<&Self::FieldType> {
                match <Self as SetGetPathField::<$marker>>::try_get(self, path) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut_get(&self, path: &Path) -> Option<&Self::FieldType> {
                match self.cache.of_files.get(path).map(|e| &e.$map_field) {
                    Some(CacheState::Cached(val)) => Some(val),
                    _ => None,
                }
            }
        }
    )* };

    (@ti;
    $( $tic_field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] track field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl SetGetPathTrackField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self, path: &Path, num: u64) -> Result<(), MuxError> {
                let _ = self.get_mut_track_cache(path, num).ok_or("None CacheMIOfFileTrack")?;

                let (state, result) = match self.$builder(path, num) {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => (CacheState::Failed, Err(e)),
                };

                let tic = self.get_mut_track_cache(path, num).expect("None CacheMIOfFileTrack");
                tic.$tic_field = state;
                result
            }

            fn try_get(&mut self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError> {
                let tic = self.get_mut_track_cache(path, num).ok_or("None CacheMIOfFileTrack")?;
                let need_try_set = match tic.$tic_field {
                    CacheState::NotCached => true,
                    _ => false,
                };

                if need_try_set {
                    <Self as SetGetPathTrackField::<$marker>>::try_set(self, path, num)?;
                }

                let tic = self.get_mut_track_cache(path, num).expect("None CacheMIOfFileTrack");
                match &tic.$tic_field {
                    CacheState::Cached(val) => Ok(val),
                    CacheState::Failed => Err("Previously failed to load".into()),
                    _ => Err("Cache entry missing".into()),
                }
            }

            fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType> {
                match <Self as SetGetPathTrackField::<$marker>>::try_get(self, path, num) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }
        }
    )* };
}

set_get_fields!(
    stem, Arc<OsString>, build_stem => MICmnStem;
);

set_get_path_fields!(
    mkvinfo, Mkvinfo, build_mkvinfo => MIMkvinfo;
    mkvmerge_i, Vec<String>, build_mkvmerge_i => MIMkvmergeI;
    path_tail, String, build_path_tail => MIPathTail;
    relative_upmost, String, build_relative_upmost => MIRelativeUpmost;
    saved_tracks, EnumMap<TrackType, BTreeSet<u64>>, build_saved_tracks => MISavedTracks;
    sub_charset, SubCharset, build_sub_charset => MISubCharset;
    target_group, TargetGroup, build_target_group => MITargetGroup;
    targets, SmallVec<[Target; 3]>, build_targets => MITargets;
    tracks_info, HashMap<u64, CacheMIOfFileTrack>, build_tracks_info => MITracksInfo;
    attachs_info, HashMap<u64, CacheMIOfFileAttach>, build_attachs_info => MIAttachsInfo;
);

set_get_path_fields!(
    @ti;
    lang, LangCode, build_ti_lang => MITILang;
    name, String, build_ti_name => MITIName;
    track_ids, [TrackID; 2], build_ti_track_ids => MITITrackIDs;
);
