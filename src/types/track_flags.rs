pub(crate) mod counts;
pub(crate) mod flag_type;
mod new;
mod to_args;

use crate::{IsDefault, TrackID, deref_singleton_tuple_struct};
use flag_type::TrackFlagType;
use std::collections::HashMap;

/// Settings for `default` track flags.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct DefaultTrackFlags(pub TrackFlags);

/// Settings for `forced` track flags.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct ForcedTrackFlags(pub TrackFlags);

/// Settings for `enabled` track flags.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct EnabledTrackFlags(pub TrackFlags);

/// Common interface for settings of track flags by type.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct TrackFlags {
    pub lim_for_unset: Option<u64>,
    pub unmapped: Option<bool>,
    pub map_hashed: Option<HashMap<TrackID, bool>>,
    pub map_unhashed: Option<Vec<(TrackID, bool)>>,
}

deref_singleton_tuple_struct!(DefaultTrackFlags, TrackFlags, @all);
deref_singleton_tuple_struct!(ForcedTrackFlags, TrackFlags, @all);
deref_singleton_tuple_struct!(EnabledTrackFlags, TrackFlags, @all);

impl TrackFlags {
    /// Returns auto-value of flag based on `true` limit.
    pub fn auto_val(&self, cnt: u64, ft: TrackFlagType) -> bool {
        cnt < self.lim_for_unset.unwrap_or(Self::auto_lim_for_unset(ft))
    }

    /// Returns user-defined value of flag if exists; otherwise, returns `None`.
    pub fn get(&self, tid: &TrackID) -> Option<bool> {
        if let Some(val) = &self.unmapped {
            return Some(*val);
        }

        if let Some(vals) = &self.map_hashed {
            if let Some(val) = vals.get(&tid) {
                return Some(*val);
            }
        }

        if !tid.is_range() {
            return self.map_unhashed.as_ref().and_then(|vals| {
                vals.iter()
                    .find(|(id, _)| id.contains(&tid))
                    .map(|(_, val)| *val)
            });
        }

        None
    }

    #[inline]
    const fn auto_lim_for_unset(ft: TrackFlagType) -> u64 {
        match ft {
            TrackFlagType::Default => 1,
            TrackFlagType::Forced => 0,
            TrackFlagType::Enabled => u64::MAX,
        }
    }
}
