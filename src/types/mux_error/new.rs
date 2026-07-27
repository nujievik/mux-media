use super::{MuxError, MuxErrorOther, MuxErrorParse};
use std::num;

macro_rules! from_any_parse {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for MuxError {
            fn from(err: $ty) -> MuxError {
                MuxError::Parse(MuxErrorParse::$variant(err))
            }
        }
    };
}

from_any_parse!(num::ParseFloatError, Float);
from_any_parse!(num::ParseIntError, Int);
from_any_parse!(rsubs_lib::SRTError, SrtSubtitles);
from_any_parse!(rsubs_lib::SSAError, SsaSubtitles);
from_any_parse!(rsubs_lib::VTTError, VttSubtitles);

impl MuxError {
    pub(crate) fn new_with(message: String) -> MuxError {
        MuxError::Other(MuxErrorOther { code: 1, message })
    }

    pub(crate) fn new_ok() -> MuxError {
        MuxError::Other(MuxErrorOther {
            code: 0,
            message: String::new(),
        })
    }
}
