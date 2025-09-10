use super::{Attachs, id::AttachID};
use crate::MuxError;
use std::collections::HashSet;

impl std::str::FromStr for Attachs {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (inverse, s) = if s.starts_with('!') {
            (true, &s[1..])
        } else {
            (false, s)
        };

        let mut ids_hashed: Option<HashSet<AttachID>> = None;
        let mut ids_unhashed: Option<Vec<AttachID>> = None;

        for part in s.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let id = AttachID::from_str(part)?;
            if id.is_range() {
                ids_unhashed.get_or_insert_with(Vec::new).push(id);
            } else {
                ids_hashed.get_or_insert_with(HashSet::new).insert(id);
            }
        }

        if ids_hashed.is_none() && ids_unhashed.is_none() {
            return Err("Not attach ID found".into());
        }

        Ok(Self {
            no_flag: false,
            inverse,
            ids_hashed,
            ids_unhashed,
        })
    }
}
