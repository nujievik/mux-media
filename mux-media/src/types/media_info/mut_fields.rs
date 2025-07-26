use super::MediaInfo;
use super::cache::{CacheMIOfFile, CacheState};
use crate::{
    ArcPathBuf, CacheMIOfFileAttach, CacheMIOfFileTrack, LangCode, MutField, MutPathField,
    MutPathNumField, MuxError, SubCharset, Target, TargetGroup, TrackID, TrackType,
};
use enum_map::EnumMap;
use matroska::Matroska;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    path::Path,
};

macro_rules! set_get_fields {
    ($( $cache:ident, $field:ident, $ty:ty, $builder:ident, $to_doc:expr => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] ", $to_doc, " field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl MutField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self) -> Result<(), MuxError> {
                let (state, result) = match self.$builder() {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => {
                        let state = format!("Previously failed: {}", e);
                        let state = CacheState::Failed(state.into());
                        (state, Err(e))
                    }
                };

                self.cache.$cache.$field = state;
                result
            }

            fn try_get(&mut self) -> Result<&Self::FieldType, MuxError> {
                if let CacheState::NotCached = self.cache.$cache.$field {
                    <Self as MutField::<$marker>>::try_set(self)?;
                }

                match &self.cache.$cache.$field {
                    CacheState::Cached(val) => Ok(val),
                    CacheState::Failed(e) => Err(e.clone()),
                    CacheState::NotCached => Err("Unexpected NotCached state".into()),
                }
            }

            fn get(&mut self) -> Option<&Self::FieldType> {
                match <Self as MutField::<$marker>>::try_get(self) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut(&self) -> Option<&Self::FieldType> {
                match &self.cache.$cache.$field {
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

        impl MutPathField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self, path: &Path) -> Result<(), MuxError> {
                let (state, result) = match self.$builder(path) {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => {
                        let state = format!("Previously failed: {}", e);
                        let state = CacheState::Failed(state.into());
                        (state, Err(e))
                    }
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
                    Some(CacheState::Cached(_)) | Some(CacheState::Failed(_)) => {}
                    _ => {
                        <Self as MutPathField::<$marker>>::try_set(self, path)?;
                    }
                }

                match self.cache.of_files.get(path).map(|e| &e.$map_field) {
                    Some(CacheState::Cached(val)) => Ok(val),
                    Some(CacheState::Failed(e)) => Err(e.clone()),
                    _ => Err("Cache entry missing".into()),
                }
            }

            fn get(&mut self, path: &Path) -> Option<&Self::FieldType> {
                match <Self as MutPathField::<$marker>>::try_get(self, path) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut(&self, path: &Path) -> Option<&Self::FieldType> {
                match self.cache.of_files.get(path).map(|e| &e.$map_field) {
                    Some(CacheState::Cached(val)) => Some(val),
                    _ => None,
                }
            }
        }
    )* };

    (@ti;
    $( $map_field:ident, $tic_field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] track field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl MutPathNumField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_set(&mut self, path: &Path, num: u64) -> Result<(), MuxError> {
                let _ = self.get_mut_track_cache(path, num).ok_or("None CacheMIOfFileTrack")?;

                let (state, result) = match self.$builder(path, num) {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => {
                        let state = format!("Previously failed: {}", e);
                        let state = CacheState::Failed(state.into());
                        (state, Err(e))
                    }
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
                    <Self as MutPathNumField::<$marker>>::try_set(self, path, num)?;
                }

                let tic = self.get_mut_track_cache(path, num).expect("None CacheMIOfFileTrack");
                match &tic.$tic_field {
                    CacheState::Cached(val) => Ok(val),
                    CacheState::Failed(e) => Err(e.clone()),
                    _ => Err("Cache entry missing".into()),
                }
            }

            fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType> {
                match <Self as MutPathNumField::<$marker>>::try_get(self, path, num) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut(&self, path: &Path, num: u64) -> Option<&Self::FieldType> {
                self.cache.of_files.get(path)
                    .and_then(|cache| match &cache.$map_field {
                        CacheState::Cached(map) => map.get(&num),
                        _ => None,
                    })
                    .and_then(|cache| match &cache.$tic_field {
                        CacheState::Cached(val) => Some(val),
                        _ => None,
                    })
            }
        }
    )* };
}

set_get_fields!(
    common, regex_aid, Regex, build_regex_aid, "common" => MICmnRegexAttachID;
    common, regex_tid, Regex, build_regex_tid, "common" => MICmnRegexTrackID;
    common, regex_word, Regex, build_regex_word, "common" => MICmnRegexWord;
    of_group, stem, OsString, build_stem, "stem-grouped media" => MIGroupStem;
);

set_get_path_fields!(
    matroska, Matroska, build_matroska => MIMatroska;
    mkvmerge_i, Vec<String>, build_mkvmerge_i => MIMkvmergeI;

    path_tail, String, build_path_tail => MIPathTail;
    words_path_tail, Vec<String>, build_words_path_tail => MIWordsPathTail;
    relative_upmost, String, build_relative_upmost => MIRelativeUpmost;
    words_relative_upmost, Vec<String>, build_words_relative_upmost => MIWordsRelativeUpmost;

    saved_tracks, EnumMap<TrackType, BTreeSet<u64>>, build_saved_tracks => MISavedTracks;
    sub_charset, SubCharset, build_sub_charset => MISubCharset;
    target_group, TargetGroup, build_target_group => MITargetGroup;
    targets, Vec<Target>, build_targets => MITargets;

    tracks_info, HashMap<u64, CacheMIOfFileTrack>, build_tracks_info => MITracksInfo;
    attachs_info, HashMap<u64, CacheMIOfFileAttach>, build_attachs_info => MIAttachsInfo;
);

set_get_path_fields!(
    @ti;
    tracks_info, lang, LangCode, build_ti_lang => MITILang;
    tracks_info, name, String, build_ti_name => MITIName;
    tracks_info, words_name, Vec<String>, build_ti_words_name => MITIWordsName;
    tracks_info, track_ids, [TrackID; 2], build_ti_track_ids => MITITrackIDs;
);
