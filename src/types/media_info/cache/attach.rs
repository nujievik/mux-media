use crate::{AttachID, AttachType, Result};

/// Cache of [`crate::MediaInfo`] is separate for each attach in media.
#[derive(Clone, Debug)]
pub struct CacheMIOfFileAttach {
    pub id: AttachID,
    pub attach_type: AttachType,
    pub raw_ty_line: String,
}

impl CacheMIOfFileAttach {
    pub fn try_init(num: u64, raw_ty_line: &str) -> Result<Self> {
        let id = AttachID::Num(num);
        let attach_type = Self::try_init_attach_type(raw_ty_line)?;
        Ok(Self {
            id,
            attach_type,
            raw_ty_line: raw_ty_line.to_owned(),
        })
    }

    #[inline(always)]
    fn try_init_attach_type(raw_ty_line: &str) -> Result<AttachType> {
        for at in AttachType::iter() {
            if raw_ty_line.contains(at.as_str_mkvtoolnix()) {
                return Ok(at);
            }
        }
        Err("Unrecognized track type".into())
    }
}
