use super::AICache;
use crate::{AttachType, MuxError};

impl AICache {
    pub fn try_init(num: u64, mkvmerge_id_line: String) -> Result<Self, MuxError> {
        let attach_type = Self::try_init_attach_type(&mkvmerge_id_line)?;
        Ok(Self {
            num,
            attach_type,
            mkvmerge_id_line,
        })
    }

    #[inline]
    fn try_init_attach_type(mkvmerge_id_line: &str) -> Result<AttachType, MuxError> {
        for at in AttachType::iter() {
            if mkvmerge_id_line.contains(at.as_str_mkvtoolnix()) {
                return Ok(at);
            }
        }
        Err("Unrecognized track type".into())
    }
}
