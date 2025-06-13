use super::{AudioTracks, ButtonTracks, SubTracks, Tracks, VideoTracks, id::TrackID};
use crate::{IsDefault, LangCode, MuxError, deref_tuple_fields};
use std::collections::HashSet;
use std::str::FromStr;

deref_tuple_fields!(AudioTracks, Tracks, @all, no_flag: bool);
deref_tuple_fields!(SubTracks, Tracks, @all, no_flag: bool);
deref_tuple_fields!(VideoTracks, Tracks, @all, no_flag: bool);
deref_tuple_fields!(ButtonTracks, Tracks, @all, no_flag: bool);

impl IsDefault for Tracks {
    fn is_default(&self) -> bool {
        !self.no_flag && !self.inverse && self.ids_hashed.is_none() && self.ids_unhashed.is_none()
    }
}

impl Tracks {
    // Every Media Track has 2 mkvmerge supported TrackID: TrackID::U32 and TrackID::Lang.
    // We use them as u32 and LangCode
    pub fn save_track(&self, id_u32: u32, id_lang: LangCode) -> bool {
        if self.no_flag {
            return false;
        }

        if self.is_default() {
            return true;
        }

        if self.contains(TrackID::U32(id_u32)) {
            return !self.inverse;
        }

        if self.ids_hashed_contains(TrackID::Lang(id_lang)) {
            return !self.inverse;
        }

        self.inverse
    }

    #[inline]
    fn contains(&self, id: TrackID) -> bool {
        self.ids_hashed_contains(id) || self.ids_unhashed_contains(id)
    }

    #[inline]
    fn ids_hashed_contains(&self, id: TrackID) -> bool {
        self.ids_hashed
            .as_ref()
            .map_or(false, |ids| ids.contains(&id))
    }

    #[inline]
    fn ids_unhashed_contains(&self, id: TrackID) -> bool {
        self.ids_unhashed
            .as_ref()
            .map_or(false, |ids| ids.iter().any(|s_id| s_id.contains(id)))
    }
}

impl FromStr for Tracks {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (inverse, s) = if s.starts_with('!') {
            (true, &s[1..])
        } else {
            (false, s)
        };

        let mut ids_hashed: Option<HashSet<TrackID>> = None;
        let mut ids_unhashed: Option<Vec<TrackID>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let id = TrackID::from_str(part)?;
            if id.is_range() {
                ids_unhashed.get_or_insert_with(Vec::new).push(id);
            } else {
                ids_hashed.get_or_insert_with(HashSet::new).insert(id);
            }
        }

        if ids_hashed.is_none() && ids_unhashed.is_none() {
            return Err(MuxError::from("No track IDs found"));
        }

        Ok(Self {
            no_flag: false,
            inverse,
            ids_hashed,
            ids_unhashed,
        })
    }
}
