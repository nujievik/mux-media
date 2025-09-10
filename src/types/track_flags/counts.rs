use crate::{TrackFlagType, TrackType};
use enum_map::EnumMap;

/// Counts of `true` track flags by type.
#[derive(Default)]
pub struct TrackFlagsCounts(EnumMap<TrackFlagType, EnumMap<TrackType, u64>>);

impl TrackFlagsCounts {
    /// Increments the counter for the given flag and track type.
    #[inline]
    pub fn add(&mut self, flag_type: TrackFlagType, track_type: TrackType) {
        self.0[flag_type][track_type] += 1;
    }

    /// Returns the current count for the given flag and track type.
    #[inline]
    pub fn val(&self, flag_type: TrackFlagType, track_type: TrackType) -> u64 {
        self.0[flag_type][track_type]
    }
}
