use super::{AICache, CacheState, MediaInfo, TICache, mkvinfo::Mkvinfo};
use crate::{
    AttachID, LangCode, MuxError, SetGetField, SetGetPathField, SetGetPathTrackField, Target,
    TargetGroup, TrackID, TrackOrder, TrackType,
};
use enum_map::EnumMap;
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

macro_rules! set_get_fields {
    ($( $field:ident, $field_ty:ty, $builder:ident => $marker:ident; )*) => {
        $(
            pub struct $marker;

            impl SetGetField<$marker> for MediaInfo<'_> {
                type FieldType = $field_ty;

                fn try_set(&mut self) -> Result<(), MuxError> {
                    let (state, result) = match self.$builder() {
                        Ok(val) => (CacheState::Cached(val), Ok(())),
                        Err(e) => (CacheState::Failed, Err(e)),
                    };

                    self.$field = state;
                    result
                }


                fn try_get(&mut self) -> Result<&Self::FieldType, MuxError> {
                    match &self.$field {
                        CacheState::NotCached => <Self as SetGetField::<$marker>>::try_set(self)?,
                        _ => {}
                    }

                    match &self.$field {
                        CacheState::Cached(val) => Ok(val),
                        _ => Err("Previously failed to load".into()),
                    }
                }

                fn get(&mut self) -> Option<&Self::FieldType> {
                    match <Self as SetGetField::<$marker>>::try_get(self) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            log::trace!("{}", e);
                            None
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! set_get_path_fields {
    ($( $map_field:ident, $field_ty:ty, $builder:ident => $marker:ident; )*) => {
        $(
            pub struct $marker;

            impl SetGetPathField<$marker> for MediaInfo<'_> {
                type FieldType = $field_ty;

                fn try_set(&mut self, path: &Path) -> Result<(), MuxError> {
                    let (state, result) = match self.$builder(path) {
                        Ok(val) => (CacheState::Cached(val), Ok(())),
                        Err(e) => (CacheState::Failed, Err(e)),
                    };

                    self.cache
                        .entry(path.to_path_buf())
                        .or_insert_with(Default::default)
                        .$map_field = state;

                    result
                }


                fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError> {
                    match self.cache.get(path).map(|e| &e.$map_field) {
                        Some(CacheState::Cached(_)) | Some(CacheState::Failed) => {}
                        _ => {
                            <Self as SetGetPathField::<$marker>>::try_set(self, path)?;
                        }
                    }

                    match self.cache.get(path).map(|e| &e.$map_field) {
                        Some(CacheState::Cached(val)) => Ok(val),
                        Some(CacheState::Failed) => Err("Previously failed to load".into()),
                        _ => Err("Cache entry missing".into()),
                    }
                }

                fn get(&mut self, path: &Path) -> Option<&Self::FieldType> {
                    match <Self as SetGetPathField::<$marker>>::try_get(self, path) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            log::trace!("{}", e);
                            None
                        }
                    }
                }
            }
        )*
    };

    (@ti;
    $( $tic_field:ident, $field_ty:ty, $builder:ident => $marker:ident; )*) => {
        $(
            pub struct $marker;

            impl SetGetPathTrackField<$marker> for MediaInfo<'_> {
                type FieldType = $field_ty;

                fn try_set(&mut self, path: &Path, tid: &TrackID) -> Result<(), MuxError> {
                    let _ = self.get_mut_ti_cache(path, tid).ok_or("None TICache")?;

                    let (state, result) = match self.$builder(path, tid) {
                        Ok(val) => (CacheState::Cached(val), Ok(())),
                        Err(e) => (CacheState::Failed, Err(e)),
                    };

                    let tic = self.get_mut_ti_cache(path, tid).expect("None TICache");
                    tic.$tic_field = state;
                    result
                }

                fn try_get(&mut self, path: &Path, tid: &TrackID) -> Result<&Self::FieldType, MuxError> {
                    let tic = self.get_mut_ti_cache(path, tid).ok_or("None TICache")?;
                    let need_try_set = match tic.$tic_field {
                        CacheState::NotCached => true,
                        _ => false,
                    };

                    if need_try_set {
                        <Self as SetGetPathTrackField::<$marker>>::try_set(self, path, tid)?;
                    }

                    let tic = self.get_mut_ti_cache(path, tid).expect("None TICache");
                    match &tic.$tic_field {
                        CacheState::Cached(val) => Ok(val),
                        CacheState::Failed => Err("Previously failed to load".into()),
                        _ => Err("Cache entry missing".into()),
                    }
                }

                fn get(&mut self, path: &Path, tid: &TrackID) -> Option<&Self::FieldType> {
                    match <Self as SetGetPathTrackField::<$marker>>::try_get(self, path, tid) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            log::trace!("{}", e);
                            None
                        }
                    }
                }
            }
        )*
    }
}

set_get_fields!(
    track_order, TrackOrder, build_track_order => MICmnTrackOrder;
);

set_get_path_fields!(
    char_encoding, String, build_char_encoding => MICharEncoding;
    mkvinfo, Mkvinfo, build_mkvinfo => MIMkvinfo;
    mkvmerge_i, Vec<String>, build_mkvmerge_i => MIMkvmergeI;
    path_tail, String, build_path_tail => MIPathTail;
    relative_upmost, String, build_relative_upmost => MIRelativeUpmost;
    target_group, TargetGroup, build_target_group => MITargetGroup;
    targets, [Target; 3], build_targets => MITargets;
    tracks_info, HashMap<TrackID, TICache>, build_tracks_info => MITracksInfo;
    saved_track_nums, EnumMap<TrackType, BTreeSet<u64>>, build_saved_track_nums => MISavedTrackNums;
    attachs_info, HashMap<AttachID, AICache>, build_attachs_info => MIAttachsInfo;
);

set_get_path_fields!(
    @ti;
    name, String, build_ti_name => MITIName;
    lang, LangCode, build_ti_lang => MITILang;
);
