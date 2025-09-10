use super::{Tracks, id::TrackID};
use crate::{IsDefault, MuxError};
use std::collections::HashSet;

impl Tracks {
    pub fn save_track(&self, tid: &TrackID, other_tid: &TrackID) -> bool {
        if self.no_flag {
            return false;
        }

        if self.is_default() {
            return true;
        }

        let contains = |id: &TrackID| match id {
            TrackID::Num(_) => self.contains(id),
            TrackID::Lang(_) => self.ids_hashed_contains(id),
            TrackID::Range(_) => self.ids_unhashed_contains(id),
        };

        if contains(tid) {
            return !self.inverse;
        }

        if contains(other_tid) {
            return !self.inverse;
        }

        self.inverse
    }

    #[inline]
    fn contains(&self, id: &TrackID) -> bool {
        self.ids_hashed_contains(id) || self.ids_unhashed_contains(id)
    }

    #[inline]
    fn ids_hashed_contains(&self, id: &TrackID) -> bool {
        self.ids_hashed
            .as_ref()
            .map_or(false, |ids| ids.contains(id))
    }

    #[inline]
    fn ids_unhashed_contains(&self, id: &TrackID) -> bool {
        self.ids_unhashed
            .as_ref()
            .map_or(false, |ids| ids.iter().any(|s_id| s_id.contains(id)))
    }
}

impl std::str::FromStr for Tracks {
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
            let id = part.parse::<TrackID>()?;
            if id.is_range() {
                ids_unhashed.get_or_insert_with(Vec::new).push(id);
            } else {
                ids_hashed.get_or_insert_with(HashSet::new).insert(id);
            }
        }

        if ids_hashed.is_none() && ids_unhashed.is_none() {
            return Err("No track IDs found".into());
        }

        Ok(Self {
            no_flag: false,
            inverse,
            ids_hashed,
            ids_unhashed,
        })
    }
}
