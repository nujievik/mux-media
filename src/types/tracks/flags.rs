pub(crate) mod counts;
pub(crate) mod flag_type;
mod from_arg_matches;
mod from_str;
mod to_json_args;
mod to_mkvmerge_args;

use crate::{IsDefault, TrackID, deref_tuple_fields};
use flag_type::TFlagType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DefaultTFlags(TFlags);
#[derive(Clone)]
pub struct ForcedTFlags(TFlags);
#[derive(Clone)]
pub struct EnabledTFlags(TFlags);

#[derive(Clone, Default)]
pub struct TFlags {
    lim_for_unset: Option<u64>,
    unmapped: Option<bool>,
    map_hashed: Option<HashMap<TrackID, bool>>,
    map_unhashed: Option<Vec<(TrackID, bool)>>,
}

deref_tuple_fields!(DefaultTFlags, TFlags, @all, lim_for_unset: Option<u64>);
deref_tuple_fields!(ForcedTFlags, TFlags, @all, lim_for_unset: Option<u64>);
deref_tuple_fields!(EnabledTFlags, TFlags, @all, lim_for_unset: Option<u64>);

impl TFlags {
    pub fn auto_val(&self, cnt: u64, ft: TFlagType) -> bool {
        cnt < self.lim_for_unset.unwrap_or(Self::auto_lim_for_unset(ft))
    }

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

impl IsDefault for TFlags {
    fn is_default(&self) -> bool {
        self.lim_for_unset.is_none()
            && self.unmapped.is_none()
            && self.map_hashed.is_none()
            && self.map_unhashed.is_none()
    }
}
