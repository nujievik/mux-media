use crate::{TFlagType, TrackType};
use enum_map::EnumMap;

/// Counts of `true` track flags by type.
#[derive(Default)]
pub struct TFlagsCounts(EnumMap<TFlagType, EnumMap<TrackType, u64>>);

impl TFlagsCounts {
    pub fn add(&mut self, ft: TFlagType, tt: TrackType) {
        self.0[ft][tt] += 1;
    }

    pub fn get(&self, ft: TFlagType, tt: TrackType) -> u64 {
        self.0[ft][tt]
    }
}
