use super::CacheMIOfFileAttach;
use crate::{AttachID, AttachType, MuxError};

impl CacheMIOfFileAttach {
    pub fn try_init(num: u64, mkvmerge_id_line: String) -> Result<Self, MuxError> {
        let id = AttachID::Num(num);
        let attach_type = Self::try_init_attach_type(&mkvmerge_id_line)?;
        Ok(Self {
            id,
            attach_type,
            mkvmerge_id_line,
        })
    }

    #[inline(always)]
    fn try_init_attach_type(mkvmerge_id_line: &str) -> Result<AttachType, MuxError> {
        for at in AttachType::iter() {
            if mkvmerge_id_line.contains(at.as_str_mkvtoolnix()) {
                return Ok(at);
            }
        }
        Err("Unrecognized track type".into())
    }
}
