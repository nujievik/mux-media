use super::{Attachs, id::AttachID};
use crate::IsDefault;

impl Attachs {
    // Every Attach has only 1 mkvmerge supported AttachID: AttachID::U32.
    // We use this as u32
    pub fn save_attach(&self, id: u32) -> bool {
        if self.no_flag {
            return false;
        }

        if self.is_default() {
            return true;
        }

        let id = AttachID::U32(id);

        let mut val = self
            .ids_hashed
            .as_ref()
            .map_or(false, |ids| ids.contains(&id));

        if !val {
            val = self
                .ids_unhashed
                .as_ref()
                .map_or(false, |ids| ids.iter().any(|s_id| s_id.contains(id)));
        }

        val != self.inverse
    }
}
