pub(crate) mod counts;
mod from_str;
mod to_mkvmerge_args;

use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags, TFlags, id::TrackID};
use crate::{IsDefault, deref_tuple_fields};

deref_tuple_fields!(DefaultTFlags, TFlags, @all, lim_for_unset: Option<u32>);
deref_tuple_fields!(ForcedTFlags, TFlags, @all, lim_for_unset: Option<u32>);
deref_tuple_fields!(EnabledTFlags, TFlags, @all, lim_for_unset: Option<u32>);

impl TFlags {
    #[inline]
    pub fn less_cnt(&self, cnt: u32) -> bool {
        cnt < self.lim_for_unset.unwrap_or(0)
    }

    pub fn get_or_less_cnt(&self, tid: &TrackID, cnt: u32) -> bool {
        self.get(tid).unwrap_or(self.less_cnt(cnt))
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

        if let Some(vals) = &self.map_unhashed {
            for (id, val) in vals.iter() {
                if id.contains(&tid) {
                    return Some(*val);
                }
            }
        }

        None
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
