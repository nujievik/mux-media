use super::{CacheState, MediaInfo, TICache};
use crate::{MITracksInfo, MuxError, SetGetPathField, SetGetPathTrackField};
use std::collections::HashMap;
use std::path::Path;

impl MediaInfo<'_> {
    pub fn get<F>(&mut self, path: &Path) -> Option<&<Self as SetGetPathField<F>>::FieldType>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::get(self, path)
    }

    pub fn try_get<F>(
        &mut self,
        path: &Path,
    ) -> Result<&<Self as SetGetPathField<F>>::FieldType, MuxError>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::try_get(self, path)
    }

    // Use this if value is already cached in get() or try_get() methods
    pub fn unmut_get<F>(&self, path: &Path) -> Option<&<Self as SetGetPathField<F>>::FieldType>
    where
        Self: SetGetPathField<F>,
    {
        <Self as SetGetPathField<F>>::unmut_get(self, path)
    }

    pub fn get_ti<F>(
        &mut self,
        path: &Path,
        num: u64,
    ) -> Option<&<Self as SetGetPathTrackField<F>>::FieldType>
    where
        Self: SetGetPathTrackField<F>,
    {
        <Self as SetGetPathTrackField<F>>::get(self, path, num)
    }

    pub fn try_get_ti<F>(
        &mut self,
        path: &Path,
        num: u64,
    ) -> Result<&<Self as SetGetPathTrackField<F>>::FieldType, MuxError>
    where
        Self: SetGetPathTrackField<F>,
    {
        <Self as SetGetPathTrackField<F>>::try_get(self, path, num)
    }

    pub fn get_mut_tracks_info(&mut self, path: &Path) -> Option<&mut HashMap<u64, TICache>> {
        let _ = self.get::<MITracksInfo>(path)?;
        match self.cache.get_mut(path) {
            Some(entry) => match &mut entry.tracks_info {
                CacheState::Cached(val) => Some(val),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_mut_ti_cache(&mut self, path: &Path, num: u64) -> Option<&mut TICache> {
        let ti = self.get_mut_tracks_info(path)?;
        ti.get_mut(&num)
    }
}
