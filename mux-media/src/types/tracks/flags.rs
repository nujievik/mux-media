pub(crate) mod counts;
pub(crate) mod flag_type;
mod from_arg_matches;
mod from_str;
mod to_json_args;
mod to_mkvmerge_args;

use crate::{IsDefault, TrackID, deref_singleton_tuple_fields};
use flag_type::TFlagType;
use std::collections::HashMap;

/// Settings for `default` track flags.
#[derive(Clone)]
pub struct DefaultTFlags(TFlags);

/// Settings for `forced` track flags.
#[derive(Clone)]
pub struct ForcedTFlags(TFlags);

/// Settings for `enabled` track flags.
#[derive(Clone)]
pub struct EnabledTFlags(TFlags);

/// Common interface for settings of track flags by type.
#[derive(Clone, Default, IsDefault)]
pub struct TFlags {
    lim_for_unset: Option<u64>,
    unmapped: Option<bool>,
    map_hashed: Option<HashMap<TrackID, bool>>,
    map_unhashed: Option<Vec<(TrackID, bool)>>,
}

deref_singleton_tuple_fields!(DefaultTFlags, TFlags, @all, lim_for_unset: Option<u64>);
deref_singleton_tuple_fields!(ForcedTFlags, TFlags, @all, lim_for_unset: Option<u64>);
deref_singleton_tuple_fields!(EnabledTFlags, TFlags, @all, lim_for_unset: Option<u64>);

impl TFlags {
    /// Returns auto-value of flag based on `true` limit.
    pub fn auto_val(&self, cnt: u64, ft: TFlagType) -> bool {
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

    #[inline(always)]
    fn auto_lim_for_unset(ft: TFlagType) -> u64 {
        match ft {
            TFlagType::Default => 1,
            TFlagType::Forced => 0,
            TFlagType::Enabled => u64::MAX,
        }
    }
}
