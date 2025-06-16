use super::{Attachs, id::AttachID};
use crate::IsDefault;

impl Attachs {
    // Every Attach has only 1 mkvmerge supported AttachID: AttachID::Num.
    // Use this
    pub fn save_attach(&self, id: &AttachID) -> bool {
        if self.no_flag {
            return false;
        }

        if self.is_default() {
            return true;
        }

        let mut val = self
            .ids_hashed
            .as_ref()
            .map_or(false, |ids| ids.contains(id));

        if !val {
            val = self
                .ids_unhashed
                .as_ref()
                .map_or(false, |ids| ids.iter().any(|s_id| s_id.contains(id)));
        }

        val != self.inverse
    }
}
