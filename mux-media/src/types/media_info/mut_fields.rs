use super::MediaInfo;
use crate::{
    ArcPathBuf, CacheMIOfFile, CacheMIOfFileAttach, CacheMIOfFileTrack,
    CacheState::{self, Cached, Failed, NotCached},
    LangCode, LazyField, LazyPathField, LazyPathNumField, MuxError, SubCharset, Target,
    TargetGroup, TrackID, TrackOrder, TrackType, Value, mux_err,
};
use enum_map::EnumMap;
use matroska::Matroska;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    ffi::OsString,
    mem,
    path::{Path, PathBuf},
};

macro_rules! lazy_fields_methods {
    (
        $trait:ident, $doc_field:expr, $doc_markers:expr;
        $try_init:ident, $init:ident, $try_mut:ident, $get_mut:ident, $try_get:ident, $get:ident,
        $try_immut:ident, $immut:ident, $try_take:ident, $take:ident, $set:ident;
        $( ( $arg:ident : $ty:ty ) ),* $(,)?
    ) => {
        #[doc = concat!("Initializes the", $doc_field, "field for the marker",
            $doc_markers, ", if it hasn't been initialized yet.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_init<F>(&mut self $(, $arg : $ty )* ) -> Result<(), MuxError>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_init(self $(, $arg )* )
        }

        #[doc = concat!("Initializes the", $doc_field, "field for the marker",
            $doc_markers, ", if it hasn't been initialized yet.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $init<F>(&mut self $(, $arg : $ty )* ) -> Option<()>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::init(self $(, $arg )* )
        }

        #[doc = concat!("Returns a mutable reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_mut<F>(&mut self $(, $arg : $ty )* ) -> Result<&mut <Self as $trait<F>>::FieldType, MuxError>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_mut(self $(, $arg )* )
        }

        #[doc = concat!("Returns a mutable reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $get_mut<F>(&mut self $(, $arg : $ty )* ) -> Option<&mut <Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::get_mut(self $(, $arg )* )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_get<F>(&mut self $(, $arg : $ty )* ) -> Result<&<Self as $trait<F>>::FieldType, MuxError>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_get(self $(, $arg )* )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $get<F>(&mut self $(, $arg : $ty )* ) -> Option<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::get(self $(, $arg )* )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", if already initialized.")]
        ///
        /// Returns an error if the field is uninitialized or an error occurred.
        ///
        /// Use the [`immut`](crate::immut) macro for initialize brevity.
        #[inline(always)]
        pub fn $try_immut<F>(&self $(, $arg : $ty )* ) -> Result<&<Self as $trait<F>>::FieldType, MuxError>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_immut(self $(, $arg )* )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", if already initialized.")]
        ///
        /// Returns [`None`] if the field is uninitialized or an error occurred.
        ///
        /// Use the [`immut`](crate::immut) macro for initialize brevity.
        #[inline(always)]
        pub fn $immut<F>(&self $(, $arg : $ty )* ) -> Option<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::immut(self $(, $arg )* )
        }

        #[doc = concat!("Takes the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary, and replaces it with a default.")]
        ///
        /// Returns an error if the field is uninitialized or an error occurred.
        #[inline(always)]
        pub fn $try_take<F>(&mut self $(, $arg : $ty )* ) -> Result<<Self as $trait<F>>::FieldType, MuxError>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_take(self $(, $arg )* )
        }

        #[doc = concat!("Takes the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary, and replaces it with a default.")]
        ///
        /// Returns [`None`] if the field is uninitialized or an error occurred.
        #[inline(always)]
        pub fn $take<F>(&mut self $(, $arg : $ty )* ) -> Option<<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::take(self $(, $arg )* )
        }

        #[doc = concat!("Sets the", $doc_field, "field value for the marker",
            $doc_markers, " manually, replacing an existing value.")]
        #[inline(always)]
        pub fn $set<F>(&mut self $(, $arg : $ty )* , value: <Self as $trait<F>>::FieldType)
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::set(self $(, $arg )* , value)
        }
    };
}

impl MediaInfo<'_> {
    lazy_fields_methods!(
        LazyField, " common ", " `F`";
        try_init_cmn, init_cmn, try_mut_cmn, get_mut_cmn, try_get_cmn, get_cmn,
        try_immut_cmn, immut_cmn, try_take_cmn, take_cmn, set_cmn;
    );

    lazy_fields_methods!(
        LazyPathField, " ", " `F` and `media`";
        try_init, init, try_mut, get_mut, try_get, get,
        try_immut, immut, try_take, take, set;
        (media: &Path),
    );

    lazy_fields_methods!(
        LazyPathNumField, " track ", " `F`, `media` and `track`";
        try_init_ti, init_ti, try_mut_ti, get_mut_ti, try_get_ti, get_ti,
        try_immut_ti, immut_ti, try_take_ti, take_ti, set_ti;
        (media: &Path), (track: u64),
    );
}

#[inline]
fn new_state_and_result<T>(res: Result<T, MuxError>) -> (CacheState<T>, Result<(), MuxError>) {
    match res {
        Ok(val) => (CacheState::Cached(val), Ok(())),
        Err(e) => {
            let state = format!("Previously failed: {}", e);
            let state = CacheState::Failed(state.into());
            (state, Err(e))
        }
    }
}

macro_rules! lazy_fields {
    ($( $cache:ident, $field:ident, $ty:ty, $builder:ident, $to_doc:expr => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] ", $to_doc, " field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl LazyField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_init(&mut self) -> Result<(), MuxError> {
                if let NotCached = self.cache.$cache.$field {
                    let (state, result) = new_state_and_result(self.$builder());
                    self.cache.$cache.$field = state;
                    return result;
                }

                self.cache.$cache.$field.try_get().map(|_| ())
            }

            fn try_mut(&mut self) -> Result<&mut Self::FieldType, MuxError> {
                <Self as LazyField::<$marker>>::try_init(self)?;
                self.cache.$cache.$field.try_mut()
            }

            #[inline]
            fn try_immut(&self) -> Result<&Self::FieldType, MuxError> {
                self.cache.$cache.$field.try_get()
            }

            fn try_take(&mut self) -> Result<Self::FieldType, MuxError> {
                <Self as LazyField::<$marker>>::try_init(self)?;
                self.cache.$cache.$field.try_take()
            }

            #[inline]
            fn set(&mut self, value: Self::FieldType) {
                self.cache.$cache.$field = Cached(value);
            }

            fn init(&mut self) -> Option<()> {
                if let NotCached = self.cache.$cache.$field {
                    let (state, result) = new_state_and_result(self.$builder());
                    self.cache.$cache.$field = state;
                    return result.ok();
                }

                match self.cache.$cache.$field {
                    Cached(_) => Some(()),
                    _ => None,
                }
            }

            fn get_mut(&mut self) -> Option<&mut Self::FieldType> {
                <Self as LazyField::<$marker>>::init(self)?;
                self.cache.$cache.$field.get_mut()
            }

            fn immut(&self) -> Option<&Self::FieldType> {
                self.cache.$cache.$field.get()
            }

            fn take(&mut self) -> Option<Self::FieldType> {
                <Self as LazyField::<$marker>>::init(self)?;
                self.cache.$cache.$field.take()
            }
        }
    )* };
}

macro_rules! lazy_path_fields {
    ($( $map_field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] fields, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl LazyPathField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_init(&mut self, media: &Path) -> Result<(), MuxError> {
                match self.cache.of_files.get(media).map(|e| &e.$map_field) {
                    Some(Cached(_)) => return Ok(()),
                    Some(Failed(e)) => return Err(e.clone()),
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(media));

                match self.cache.of_files.get_mut(media) {
                    Some(fields) => fields.$map_field = state,
                    None => {
                        let mut cache = CacheMIOfFile::default();
                        cache.$map_field = state;
                        self.cache.of_files.insert(ArcPathBuf::from(media), cache);
                    }
                }

                result
            }

            fn try_mut(&mut self, media: &Path) -> Result<&mut Self::FieldType, MuxError> {
                <Self as LazyPathField::<$marker>>::try_init(self, media)?;

                match self.cache.of_files.get_mut(media) {
                    Some(cache) => cache.$map_field.try_mut(),
                    None => Err("Unexpected None cache".into()),
                }
            }

            fn try_immut(&self, media: &Path) -> Result<&Self::FieldType, MuxError> {
                self.cache
                    .of_files.get(media)
                    .ok_or_else(|| "Not cached media".into())
                    .and_then(|cache| cache.$map_field.try_get())
            }

            fn try_take(&mut self, media: &Path) -> Result<Self::FieldType, MuxError> {
                <Self as LazyPathField::<$marker>>::try_init(self, media)?;

                self.cache
                    .of_files
                    .get_mut(media)
                    .ok_or_else(|| "Not cached media".into())
                    .and_then(|cache| cache.$map_field.try_take())
            }

            fn set(&mut self, media: &Path, value: Self::FieldType) {
                if let Some(fields) = self.cache.of_files.get_mut(media) {
                    fields.$map_field = Cached(value);
                }
            }

            fn init(&mut self, media: &Path) -> Option<()> {
                match self.cache.of_files.get(media).map(|e| &e.$map_field) {
                    Some(Cached(_)) => return Some(()),
                    Some(Failed(_)) => return None,
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(media));

                match self.cache.of_files.get_mut(media) {
                    Some(fields) => fields.$map_field = state,
                    None => {
                        let mut cache = CacheMIOfFile::default();
                        cache.$map_field = state;
                        self.cache.of_files.insert(ArcPathBuf::from(media), cache);
                    }
                }

                result.ok()
            }

            fn get_mut(&mut self, media: &Path) -> Option<&mut Self::FieldType> {
                <Self as LazyPathField::<$marker>>::init(self, media)?;
                self.cache.of_files.get_mut(media).and_then(|cache| cache.$map_field.get_mut())
            }

            fn immut(&self, media: &Path) -> Option<&Self::FieldType> {
                self.cache.of_files.get(media).and_then(|cache| cache.$map_field.get())
            }

            fn take(&mut self, media: &Path) -> Option<Self::FieldType> {
                <Self as LazyPathField::<$marker>>::init(self, media)?;
                self.cache.of_files.get_mut(media).and_then(|cache| cache.$map_field.take())
            }
        }
    )* };
}

/// Marker of [`MediaInfo`] track field, that stores [`CacheMIOfFileTrack`].
pub struct MITICache;

impl LazyPathNumField<MITICache> for MediaInfo<'_> {
    type FieldType = CacheMIOfFileTrack;

    fn try_init(&mut self, media: &Path, track: u64) -> Result<(), MuxError> {
        <Self as LazyPathNumField<MITICache>>::try_mut(self, media, track).map(|_| ())
    }

    fn try_mut(&mut self, media: &Path, track: u64) -> Result<&mut Self::FieldType, MuxError> {
        <Self as LazyPathField<MITracksInfo>>::try_mut(self, media)?
            .get_mut(&track)
            .ok_or_else(|| {
                mux_err!(
                    "Not found track {} cache: track not exists or not saves",
                    track
                )
            })
    }

    fn try_immut(&self, media: &Path, track: u64) -> Result<&Self::FieldType, MuxError> {
        <Self as LazyPathField<MITracksInfo>>::try_immut(self, media)?
            .get(&track)
            .ok_or_else(|| {
                mux_err!(
                    "Not found track {} cache: track not exists or not saves",
                    track
                )
            })
    }

    fn try_take(&mut self, media: &Path, track: u64) -> Result<Self::FieldType, MuxError> {
        <Self as LazyPathNumField<MITICache>>::try_mut(self, media, track)
            .map(|cache| mem::take(cache))
    }

    fn set(&mut self, media: &Path, track: u64, value: Self::FieldType) {
        if let Some(mut map) = self.take::<MITracksInfo>(media) {
            map.insert(track, value);
            self.set::<MITracksInfo>(media, map);
        }
    }

    fn init(&mut self, media: &Path, track: u64) -> Option<()> {
        <Self as LazyPathNumField<MITICache>>::get(self, media, track).map(|_| ())
    }

    fn get_mut(&mut self, media: &Path, track: u64) -> Option<&mut Self::FieldType> {
        <Self as LazyPathField<MITracksInfo>>::get_mut(self, media)?.get_mut(&track)
    }

    fn immut(&self, media: &Path, track: u64) -> Option<&Self::FieldType> {
        <Self as LazyPathField<MITracksInfo>>::immut(self, media)?.get(&track)
    }

    fn take(&mut self, media: &Path, track: u64) -> Option<Self::FieldType> {
        <Self as LazyPathField<MITracksInfo>>::get_mut(self, media)?
            .get_mut(&track)
            .map(|cache| mem::take(cache))
    }
}

macro_rules! lazy_path_num_fields {
    ( $( $map_field:ident, $tic_field:ident, $ty:ty, $builder:ident => $marker:ident; )* ) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] track field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl LazyPathNumField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            fn try_init(&mut self, media: &Path, track: u64) -> Result<(), MuxError> {
                match &<Self as LazyPathNumField<MITICache>>::try_mut(self, media, track)?.$tic_field {
                    Cached(_) => return Ok(()),
                    Failed(e) => return Err(e.clone()),
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(media, track));

                <Self as LazyPathNumField<MITICache>>::try_mut(self, media, track)?
                    .$tic_field = state;

                result
            }

            fn try_mut(&mut self, media: &Path, track: u64) -> Result<&mut Self::FieldType, MuxError> {
                <Self as LazyPathNumField<$marker>>::try_init(self, media, track)?;

                <Self as LazyPathNumField<MITICache>>::try_mut(self, media, track)?
                    .$tic_field.try_mut()
            }

            fn try_immut(&self, media: &Path, track: u64) -> Result<&Self::FieldType, MuxError> {
                <Self as LazyPathNumField<MITICache>>::try_immut(self, media, track)?
                    .$tic_field.try_get()
            }

            fn try_take(&mut self, media: &Path, track: u64) -> Result<Self::FieldType, MuxError> {
                <Self as LazyPathNumField<$marker>>::try_init(self, media, track)?;

                <Self as LazyPathNumField<MITICache>>::try_mut(self, media, track)?
                    .$tic_field.try_take()
            }

            fn set(&mut self, media: &Path, track: u64, value: Self::FieldType) {
                if let Ok(cache) = <Self as LazyPathNumField::<MITICache>>::try_mut(self, media, track) {
                    cache.$tic_field = Cached(value);
                }
            }

            fn init(&mut self, media: &Path, track: u64) -> Option<()> {
                match &<Self as LazyPathNumField<MITICache>>::get_mut(self, media, track)?.$tic_field {
                    Cached(_) => return Some(()),
                    Failed(_) => return None,
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(media, track));

                <Self as LazyPathNumField<MITICache>>::get_mut(self, media, track)?
                    .$tic_field = state;

                result.ok()
            }

            fn get_mut(&mut self, media: &Path, track: u64) -> Option<&mut Self::FieldType> {
                <Self as LazyPathNumField<$marker>>::init(self, media, track)?;

                <Self as LazyPathNumField<MITICache>>::get_mut(self, media, track)?
                    .$tic_field.get_mut()
            }

            fn immut(&self, media: &Path, track: u64) -> Option<&Self::FieldType> {
                <Self as LazyPathNumField<MITICache>>::immut(self, media, track)?
                    .$tic_field.get()
            }

            fn take(&mut self, media: &Path, track: u64) -> Option<Self::FieldType> {
                <Self as LazyPathNumField<$marker>>::init(self, media, track)?;

                <Self as LazyPathNumField<MITICache>>::get_mut(self, media, track)?
                    .$tic_field.take()
            }
        }
    )* };
}

lazy_fields!(
    common, external_fonts, Vec<PathBuf>, build_external_fonts, "common" => MICmnExternalFonts;
    common, regex_attach_id, Regex, build_regex_attach_id, "common" => MICmnRegexAttachID;
    common, regex_codec, Regex, build_regex_codec, "common" => MICmnRegexCodec;
    common, regex_track_id, Regex, build_regex_track_id, "common" => MICmnRegexTrackID;
    common, regex_word, Regex, build_regex_word, "common" => MICmnRegexWord;

    of_group, stem, OsString, build_stem, "stem-grouped media" => MICmnStem;
    of_group, track_order, TrackOrder, build_track_order, "stem-grouped media" => MICmnTrackOrder;
);

lazy_path_fields!(
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

lazy_path_num_fields!(
    tracks_info, lang, Value<LangCode>, build_ti_lang => MITILang;
    tracks_info, name, Value<String>, build_ti_name => MITIName;
    tracks_info, words_name, Vec<String>, build_ti_words_name => MITIWordsName;
    tracks_info, track_ids, [TrackID; 2], build_ti_track_ids => MITITrackIDs;
    tracks_info, codec, String, build_ti_codec => MITICodec;
    tracks_info, it_signs, bool, build_ti_it_signs => MITIItSigns;
);
