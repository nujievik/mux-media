use super::{TICache, mkvinfo::Mkvinfo};
use crate::{MuxError, TrackType};
use log::trace;

impl TICache {
    pub fn try_init(
        id_u32: u32,
        mkvmerge_id_line: String,
        mkvinfo: Option<&Mkvinfo>,
    ) -> Result<Self, MuxError> {
        let track_type = Self::init_track_type(&mkvmerge_id_line)?;
        let mkvinfo_cutted = Self::init_mkvinfo_cutted(id_u32, mkvinfo);
        Ok(Self {
            id_u32,
            track_type,
            mkvmerge_id_line,
            mkvinfo_cutted,
            ..Default::default()
        })
    }

    fn init_track_type(mkvmerge_id_line: &str) -> Result<TrackType, MuxError> {
        for tt in TrackType::iter() {
            if mkvmerge_id_line.contains(tt.as_str_mkvtoolnix()) {
                return Ok(tt);
            }
        }
        Err("Unrecognized track type".into())
    }

    fn init_mkvinfo_cutted(id: u32, mkvinfo: Option<&Mkvinfo>) -> Option<Mkvinfo> {
        let mkvinfo = mkvinfo?;
        // mkvinfo uses 1-based indexing (add 1 to id for mkvmerge)
        let id = id + 1;

        let start_pattern = format!("Track number: {}", id);
        let end_pattern = format!("Track number: {}", id + 1);
        let mut start_idx = None;
        let mut end_idx = None;

        for (i, line) in mkvinfo.iter().enumerate() {
            if start_idx.is_none() && line.contains(&start_pattern) {
                start_idx = Some(i);
            } else if start_idx.is_some() && line.contains(&end_pattern) {
                end_idx = Some(i);
                break;
            }
        }

        match start_idx {
            Some(start) => {
                let end = end_idx.unwrap_or_else(|| mkvinfo.len());
                let cutted = mkvinfo[start..end].to_vec();
                Some(cutted.into())
            }
            None => {
                trace!(
                    "{}",
                    format!(
                        "Start mkvinfo for TID {} (mkvinfo {}) not found",
                        id - 1,
                        id
                    )
                );
                None
            }
        }
    }
}
