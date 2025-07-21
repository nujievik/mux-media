use super::MediaInfo;
use crate::{
    CacheMIOfFileTrack, CacheState, MuxError, SetGetField, SetGetPathField, SetGetPathTrackField,
    markers::MITracksInfo,
};
use std::{collections::HashMap, path::Path};

impl MediaInfo<'_> {
    /// Returns a reference to the value associated with the marker type `F` and the given `path`,
    /// retrieving and caching it if not already present.
    ///
    /// Returns `None` if the value cannot be retrieved.
    ///
    /// # Logging
    ///
    /// Emits error to `stdout` **only if** `MuxLogger` is initialized with at least `LevelFilter::Trace`.
    ///
    /// Error are logged if the value cannot be retrieved.
    pub fn get<F>(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Option<&<Self as SetGetPathField<F>>::FieldType>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::get(self, path.as_ref())
    }

    /// Returns a reference to the value associated with the marker type `F` and the given `path`,
    /// retrieving and caching it if not already present.
    ///
    /// Returns `MuxError` if the value cannot be retrieved.
    pub fn try_get<F>(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<&<Self as SetGetPathField<F>>::FieldType, MuxError>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::try_get(self, path.as_ref())
    }

    /// Returns a reference to the cached value for the given path, if already cached.
    ///
    /// Does not attempt to retrieve or cache the value. Use after [`self.get`] or [`self.try_get`].
    pub fn unmut_get<F>(
        &self,
        path: impl AsRef<Path>,
    ) -> Option<&<Self as SetGetPathField<F>>::FieldType>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::unmut_get(self, path.as_ref())
    }

    /// Returns a reference to the value associated with the marker type `F`,
    /// `path` and track number `num`, retrieving and caching it if not already present.
    ///
    /// Returns `None` if the value cannot be retrieved.
    ///
    /// # Logging
    ///
    /// Emits error to `stdout` **only if** `MuxLogger` is initialized with at least `LevelFilter::Trace`.
    ///
    /// Error are logged if the value cannot be retrieved.
    pub fn get_ti<F>(
        &mut self,
        path: impl AsRef<Path>,
        num: u64,
    ) -> Option<&<Self as SetGetPathTrackField<F>>::FieldType>
    where
        Self: SetGetPathTrackField<F>,
    {
        <Self as SetGetPathTrackField<F>>::get(self, path.as_ref(), num)
    }

    /// Returns a reference to the value associated with the marker type `F`,
    /// `path` and track number `num`, retrieving and caching it if not already present.
    ///
    /// Returns `MuxError` if the value cannot be retrieved.
    pub fn try_get_ti<F>(
        &mut self,
        path: impl AsRef<Path>,
        num: u64,
    ) -> Result<&<Self as SetGetPathTrackField<F>>::FieldType, MuxError>
    where
        Self: SetGetPathTrackField<F>,
    {
        <Self as SetGetPathTrackField<F>>::try_get(self, path.as_ref(), num)
    }

    /// Returns a reference to the common or common for stem-grouped media value,
    /// associated with the marker type `F`.
    ///
    /// Returns `MuxError` if the value cannot be retrieved.
    pub fn try_get_cmn<F>(&mut self) -> Result<&<Self as SetGetField<F>>::FieldType, MuxError>
    where
        Self: SetGetField<F>,
    {
        <Self as SetGetField<F>>::try_get(self)
    }

    pub(super) fn get_mut_tracks_info(
        &mut self,
        path: &Path,
    ) -> Option<&mut HashMap<u64, CacheMIOfFileTrack>> {
        let _ = self.get::<MITracksInfo>(path)?;
        self.cache
            .of_files
            .get_mut(path)
            .and_then(|entry| match &mut entry.tracks_info {
                CacheState::Cached(val) => Some(val),
                _ => None,
            })
    }

    pub(super) fn get_mut_track_cache(
        &mut self,
        path: &Path,
        num: u64,
    ) -> Option<&mut CacheMIOfFileTrack> {
        let ti = self.get_mut_tracks_info(path)?;
        ti.get_mut(&num)
    }
}
