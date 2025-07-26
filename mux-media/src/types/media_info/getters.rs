use super::MediaInfo;
use crate::{
    CacheMIOfFileTrack, CacheState, MutField, MutPathField, MutPathNumField, MuxError,
    markers::MITracksInfo,
};
use std::{collections::HashMap, path::Path};

impl MediaInfo<'_> {
    /// Caches and returns the common value for marker `F`.
    ///
    /// Returns an error if the value cannot be retrieved.
    #[inline(always)]
    pub fn try_get_cmn<F>(&mut self) -> Result<&<Self as MutField<F>>::FieldType, MuxError>
    where
        Self: MutField<F>,
    {
        <Self as MutField<F>>::try_get(self)
    }

    /// Caches and returns the common value for marker `F`.
    ///
    /// Returns [`None`] if the value cannot be retrieved.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Trace`](
    ///   log::LevelFilter::Trace).
    ///
    /// - The value cannot be retrieved.
    #[inline(always)]
    pub fn get_cmn<F>(&mut self) -> Option<&<Self as MutField<F>>::FieldType>
    where
        Self: MutField<F>,
    {
        <Self as MutField<F>>::get(self)
    }

    /// Gets the common value for marker `F`.
    ///
    /// # Warning
    ///
    /// Does not cache the value. Assumes [`Self::get_cmn`] or [`Self::try_get_cmn`]
    /// was called beforehand. Otherwise, returns [`None`].
    ///
    /// Use the [`unmut`](crate::unmut) macro for cache brevity.
    #[inline(always)]
    pub fn unmut_cmn<F>(&self) -> Option<&<Self as MutField<F>>::FieldType>
    where
        Self: MutField<F>,
    {
        <Self as MutField<F>>::unmut(self)
    }

    /// Caches and returns the value for marker `F` and media.
    ///
    /// Returns an error if the value cannot be retrieved.
    #[inline(always)]
    pub fn try_get<F>(
        &mut self,
        media: impl AsRef<Path>,
    ) -> Result<&<Self as MutPathField<F>>::FieldType, MuxError>
    where
        Self: MutPathField<F>,
    {
        <Self as MutPathField<F>>::try_get(self, media.as_ref())
    }

    /// Caches and returns the value for marker `F` and media.
    ///
    /// Returns [`None`] if the value cannot be retrieved.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Trace`](
    ///   log::LevelFilter::Trace).
    ///
    /// - The value cannot be retrieved.
    #[inline(always)]
    pub fn get<F>(
        &mut self,
        media: impl AsRef<Path>,
    ) -> Option<&<Self as MutPathField<F>>::FieldType>
    where
        Self: MutPathField<F>,
    {
        <Self as MutPathField<F>>::get(self, media.as_ref())
    }

    /// Gets the value for marker `F` and media.
    ///
    /// # Warning
    ///
    /// Does not cache the value. Assumes [`Self::get`] or [`Self::try_get`] was called beforehand.
    /// Otherwise, returns [`None`].
    ///
    /// ```
    /// # use mux_media::{MuxConfig, MediaInfo, TargetGroup, markers::MITargetGroup};
    /// # use std::path::Path;
    /// #
    /// # let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    /// #     .join("tests")
    /// #     .join("test_data");
    /// # let args = [Path::new("-i"), &data_dir];
    /// # let mux_config = MuxConfig::try_from_args(args).unwrap();
    /// let mut mi = MediaInfo::from(&mux_config);
    /// let path = data_dir.join("srt.srt");
    /// assert_eq!(None, mi.unmut::<MITargetGroup>(&path));
    /// assert_eq!(Some(&TargetGroup::Subs), mi.get::<MITargetGroup>(&path));
    /// assert_eq!(Some(&TargetGroup::Subs), mi.unmut::<MITargetGroup>(&path));
    /// ```
    ///
    /// Use the [`unmut`](crate::unmut) macro for brevity.
    /// ```
    /// # use mux_media::{MuxConfig, MediaInfo, TargetGroup, markers::MITargetGroup, unmut};
    /// # use std::path::Path;
    /// #
    /// # let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    /// #     .join("tests")
    /// #     .join("test_data");
    /// # let args = [Path::new("-i"), &data_dir];
    /// # let mux_config = MuxConfig::try_from_args(args).unwrap();
    /// let mut mi = MediaInfo::from(&mux_config);
    /// let path = data_dir.join("srt.srt");
    /// assert_eq!(Some(&TargetGroup::Subs), unmut!(mi, MITargetGroup, &path));
    /// ```
    #[inline(always)]
    pub fn unmut<F>(&self, media: impl AsRef<Path>) -> Option<&<Self as MutPathField<F>>::FieldType>
    where
        Self: MutPathField<F>,
    {
        <Self as MutPathField<F>>::unmut(self, media.as_ref())
    }

    /// Caches and returns the value for marker `F`, media and track.
    ///
    /// Returns an error if the value cannot be retrieved.
    #[inline(always)]
    pub fn try_get_ti<F>(
        &mut self,
        media: impl AsRef<Path>,
        track: u64,
    ) -> Result<&<Self as MutPathNumField<F>>::FieldType, MuxError>
    where
        Self: MutPathNumField<F>,
    {
        <Self as MutPathNumField<F>>::try_get(self, media.as_ref(), track)
    }

    /// Caches and returns the value for marker `F`, media and track.
    ///
    /// Returns [`None`] if the value cannot be retrieved.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Trace`](
    ///   log::LevelFilter::Trace).
    ///
    /// - The value cannot be retrieved.
    #[inline(always)]
    pub fn get_ti<F>(
        &mut self,
        media: impl AsRef<Path>,
        track: u64,
    ) -> Option<&<Self as MutPathNumField<F>>::FieldType>
    where
        Self: MutPathNumField<F>,
    {
        <Self as MutPathNumField<F>>::get(self, media.as_ref(), track)
    }

    /// Gets the value for marker `F`, media and track.
    ///
    /// # Warning
    ///
    /// Does not cache the value. Assumes [`Self::get_ti`] or [`Self::try_get_ti`]
    /// was called beforehand. Otherwise, returns [`None`].
    ///
    /// Use the [`unmut`](crate::unmut) macro for cache brevity.
    #[inline(always)]
    pub fn unmut_ti<F>(
        &self,
        media: impl AsRef<Path>,
        track: u64,
    ) -> Option<&<Self as MutPathNumField<F>>::FieldType>
    where
        Self: MutPathNumField<F>,
    {
        <Self as MutPathNumField<F>>::unmut(self, media.as_ref(), track)
    }

    pub(super) fn get_mut_tracks_info(
        &mut self,
        media: &Path,
    ) -> Option<&mut HashMap<u64, CacheMIOfFileTrack>> {
        let _ = self.get::<MITracksInfo>(media)?;
        self.cache
            .of_files
            .get_mut(media)
            .and_then(|entry| match &mut entry.tracks_info {
                CacheState::Cached(val) => Some(val),
                _ => None,
            })
    }

    pub(super) fn get_mut_track_cache(
        &mut self,
        media: &Path,
        num: u64,
    ) -> Option<&mut CacheMIOfFileTrack> {
        let ti = self.get_mut_tracks_info(media)?;
        ti.get_mut(&num)
    }
}
