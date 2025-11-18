use super::MediaInfo;
use crate::{
    ArcPathBuf, CacheMIOfFile,
    CacheState::{self, Cached, Failed, NotCached},
    Duration, LazyField, LazyPathField, Result, Stream, StreamsOrder, SubCharset, Target,
};
use std::{ffi::OsString, mem, path::Path};

/// Initializes a [`MediaInfo`](crate::MediaInfo) value if need and immutably borrows it.
#[macro_export]
macro_rules! immut {
    (@try, $ref_mut:ident, $marker:path) => {
        match $ref_mut.try_init_cmn($marker) {
            Ok(()) => $ref_mut.try_immut_cmn($marker),
            Err(e) => Err(e),
        }
    };

    ($ref_mut:ident, $marker:path) => {
        match $ref_mut.init_cmn($marker) {
            Some(()) => $ref_mut.immut_cmn($marker),
            None => None,
        }
    };

    (@try, $ref_mut:ident, $marker:path, $src:expr) => {
        match $ref_mut.try_init($marker, $src) {
            Ok(()) => $ref_mut.try_immut($marker, $src),
            Err(e) => Err(e),
        }
    };

    ($ref_mut:ident, $marker:ident, $src:expr) => {
        match $ref_mut.init($marker, $src) {
            Some(()) => $ref_mut.immut($marker, $src),
            None => None,
        }
    };
}

macro_rules! lazy_fields_methods {
    (
        $trait:ident, $doc_field:expr, $doc_markers:expr;
        $try_init:ident, $init:ident, $try_mut:ident, $get_mut:ident, $try_get:ident, $get:ident,
        $try_immut:ident, $immut:ident, $try_take:ident, $take:ident, $set:ident;
        $( $src:ident )?
    ) => {
        #[doc = concat!("Initializes the", $doc_field, "field for the marker",
            $doc_markers, ", if it hasn't been initialized yet.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_init<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Result<()>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_init(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Initializes the", $doc_field, "field for the marker",
            $doc_markers, ", if it hasn't been initialized yet.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $init<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Option<()>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::init(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a mutable reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_mut<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Result<&mut <Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_mut(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a mutable reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $get_mut<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Option<&mut <Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::get_mut(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns an error if the value could not be initialized.
        #[inline(always)]
        pub fn $try_get<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Result<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_get(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary.")]
        ///
        /// Returns [`None`] if the value could not be initialized.
        #[inline(always)]
        pub fn $get<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Option<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::get(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", if already initialized.")]
        ///
        /// Returns an error if the field is uninitialized or an error occurred.
        ///
        /// Use the [`immut`](crate::immut) macro for initialize brevity.
        #[inline(always)]
        pub fn $try_immut<F>(&self, _: F $(, $src: impl AsRef<Path> )? ) -> Result<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_immut(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Returns a reference to the", $doc_field, "field value for the marker",
            $doc_markers, ", if already initialized.")]
        ///
        /// Returns [`None`] if the field is uninitialized or an error occurred.
        ///
        /// Use the [`immut`](crate::immut) macro for initialize brevity.
        #[inline(always)]
        pub fn $immut<F>(&self, _: F $(, $src: impl AsRef<Path> )? ) -> Option<&<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::immut(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Takes the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary, and replaces it with a default.")]
        ///
        /// Returns an error if the field is uninitialized or an error occurred.
        #[inline(always)]
        pub fn $try_take<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Result<<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::try_take(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Takes the", $doc_field, "field value for the marker",
            $doc_markers, ", initializing it if necessary, and replaces it with a default.")]
        ///
        /// Returns [`None`] if the field is uninitialized or an error occurred.
        #[inline(always)]
        pub fn $take<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? ) -> Option<<Self as $trait<F>>::FieldType>
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::take(self $(, $src.as_ref() )? )
        }

        #[doc = concat!("Sets the", $doc_field, "field value for the marker",
            $doc_markers, " manually, replacing an existing value.")]
        #[inline(always)]
        pub fn $set<F>(&mut self, _: F $(, $src: impl AsRef<Path> )? , value: <Self as $trait<F>>::FieldType)
        where
            Self: $trait<F>,
        {
            <Self as $trait<F>>::set(self $(, $src.as_ref() )? , value)
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
        LazyPathField, " ", " `F` and `src`";
        try_init, init, try_mut, get_mut, try_get, get,
        try_immut, immut, try_take, take, set;
        src
    );
}

#[inline]
fn new_state_and_result<T>(res: Result<T>) -> (CacheState<T>, Result<()>) {
    let r = match res {
        Ok(_) => Ok(()),
        Err(ref e) => Err(e.clone()),
    };
    let state = CacheState::from_res(res);
    (state, r)
}

macro_rules! lazy_fields {
    ($( $field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] field, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl LazyField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            #[inline]
            fn try_init(&mut self) -> Result<()> {
                if let NotCached = self.cache.of_group.$field {
                    let (state, result) = new_state_and_result(self.$builder());
                    self.cache.of_group.$field = state;
                    return result;
                }

                self.cache.of_group.$field.try_get().map(|_| ())
            }

            #[inline]
            fn init(&mut self) -> Option<()> {
                if let NotCached = self.cache.of_group.$field {
                    let (state, result) = new_state_and_result(self.$builder());
                    self.cache.of_group.$field = state;
                    return result.ok();
                }

                match self.cache.of_group.$field {
                    Cached(_) => Some(()),
                    _ => None,
                }
            }

            #[inline]
            fn try_mut(&mut self) -> Result<&mut Self::FieldType> {
                <Self as LazyField::<$marker>>::try_init(self)?;
                self.cache.of_group.$field.try_mut()
            }

            #[inline]
            fn get_mut(&mut self) -> Option<&mut Self::FieldType> {
                <Self as LazyField::<$marker>>::init(self)?;
                self.cache.of_group.$field.get_mut()
            }

            #[inline]
            fn try_immut(&self) -> Result<&Self::FieldType> {
                self.cache.of_group.$field.try_get()
            }

            #[inline]
            fn immut(&self) -> Option<&Self::FieldType> {
                self.cache.of_group.$field.get()
            }

            #[inline]
            fn try_take(&mut self) -> Result<Self::FieldType> {
                <Self as LazyField::<$marker>>::try_init(self)?;
                self.cache.of_group.$field.try_take()
            }

            #[inline]
            fn take(&mut self) -> Option<Self::FieldType> {
                <Self as LazyField::<$marker>>::init(self)?;
                self.cache.of_group.$field.take()
            }

            #[inline]
            fn set(&mut self, value: Self::FieldType) {
                self.cache.of_group.$field = Cached(value);
            }
        }
    )* };
}

/// Marker of [`MediaInfo`] fields, that stores
pub struct MICache;

impl LazyPathField<MICache> for MediaInfo<'_> {
    type FieldType = CacheMIOfFile;

    fn try_init(&mut self, src: &Path) -> Result<()> {
        if let None = self.cache.of_files.get(src) {
            let _ = self.cache.of_files.insert(src.into(), Default::default());
        }
        Ok(())
    }

    fn try_mut(&mut self, src: &Path) -> Result<&mut Self::FieldType> {
        <Self as LazyPathField<MICache>>::try_init(self, src).unwrap();
        Ok(self.cache.of_files.get_mut(src).unwrap())
    }

    fn try_immut(&self, src: &Path) -> Result<&Self::FieldType> {
        self.cache
            .of_files
            .get(src)
            .ok_or_else(|| err!("Not cached file '{}'", src.display()))
    }

    fn try_take(&mut self, src: &Path) -> Result<Self::FieldType> {
        <Self as LazyPathField<MICache>>::try_init(self, src).unwrap();
        Ok(mem::take(self.cache.of_files.get_mut(src).unwrap()))
    }

    fn set(&mut self, src: &Path, value: CacheMIOfFile) {
        let _ = self.cache.of_files.insert(src.into(), value);
    }
}

macro_rules! lazy_path_fields {
    ($( $map_field:ident, $ty:ty, $builder:ident => $marker:ident; )*) => { $(
        #[doc = concat!("Marker of [`MediaInfo`] fields, that stores [`", stringify!($ty), "`].")]
        pub struct $marker;

        impl LazyPathField<$marker> for MediaInfo<'_> {
            type FieldType = $ty;

            #[inline]
            fn try_init(&mut self, src: &Path) -> Result<()> {
                match self.cache.of_files.get(src).map(|e| &e.$map_field) {
                    Some(Cached(_)) => return Ok(()),
                    Some(Failed(e)) => return Err(*e.clone()),
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(src));

                match self.cache.of_files.get_mut(src) {
                    Some(fields) => fields.$map_field = state,
                    None => {
                        let mut cache = CacheMIOfFile::default();
                        cache.$map_field = state;
                        self.cache.of_files.insert(ArcPathBuf::from(src), cache);
                    }
                }

                result
            }

            #[inline]
            fn init(&mut self, src: &Path) -> Option<()> {
                match self.cache.of_files.get(src).map(|e| &e.$map_field) {
                    Some(Cached(_)) => return Some(()),
                    Some(Failed(_)) => return None,
                    _ => {}
                }

                let (state, result) = new_state_and_result(self.$builder(src));

                match self.cache.of_files.get_mut(src) {
                    Some(fields) => fields.$map_field = state,
                    None => {
                        let mut cache = CacheMIOfFile::default();
                        cache.$map_field = state;
                        self.cache.of_files.insert(ArcPathBuf::from(src), cache);
                    }
                }

                result.ok()
            }

            #[inline]
            fn try_mut(&mut self, src: &Path) -> Result<&mut Self::FieldType> {
                <Self as LazyPathField::<$marker>>::try_init(self, src)?;

                match self.cache.of_files.get_mut(src) {
                    Some(cache) => cache.$map_field.try_mut(),
                    None => Err("Unexpected None cache".into()),
                }
            }

            #[inline]
            fn get_mut(&mut self, src: &Path) -> Option<&mut Self::FieldType> {
                <Self as LazyPathField::<$marker>>::init(self, src)?;
                self.cache.of_files.get_mut(src).and_then(|cache| cache.$map_field.get_mut())
            }

            #[inline]
            fn try_immut(&self, src: &Path) -> Result<&Self::FieldType> {
                self.cache
                    .of_files.get(src)
                    .ok_or_else(|| err!("Not cached file '{}'", src.display()))
                    .and_then(|cache| cache.$map_field.try_get())
            }

            #[inline]
            fn immut(&self, src: &Path) -> Option<&Self::FieldType> {
                self.cache.of_files.get(src).and_then(|cache| cache.$map_field.get())
            }

            #[inline]
            fn try_take(&mut self, src: &Path) -> Result<Self::FieldType> {
                <Self as LazyPathField::<$marker>>::try_init(self, src)?;

                self.cache
                    .of_files
                    .get_mut(src)
                    .ok_or_else(|| err!("Not cached file '{}'", src.display()))
                    .and_then(|cache| cache.$map_field.try_take())
            }

            #[inline]
            fn take(&mut self, src: &Path) -> Option<Self::FieldType> {
                <Self as LazyPathField::<$marker>>::init(self, src)?;
                self.cache.of_files.get_mut(src).and_then(|cache| cache.$map_field.take())
            }

            #[inline]
            fn set(&mut self, src: &Path, value: Self::FieldType) {
                if let Some(fields) = self.cache.of_files.get_mut(src) {
                    fields.$map_field = Cached(value);
                }
            }
        }
    )* };
}

lazy_fields!(
    stem, OsString, build_stem => MICmnStem;
    streams_order, StreamsOrder, build_streams_order => MICmnStreamsOrder;
);

lazy_path_fields!(
    streams, Vec<Stream>, build_streams => MIStreams;
    path_tail, String, build_path_tail => MIPathTail;
    relative_upmost, String, build_relative_upmost => MIRelativeUpmost;

    sub_charset, SubCharset, build_sub_charset => MISubCharset;
    target_paths, Vec<Target>, build_target_paths => MITargetPaths;

    audio_duration, Duration, build_audio_duration => MIAudioDuration;
    video_duration, Duration, build_video_duration => MIVideoDuration;
    playable_duration, Duration, build_playable_duration => MIPlayableDuration;
);
