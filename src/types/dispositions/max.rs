use super::*;
use crate::{DispositionType, StreamType};

impl Dispositions {
    /// Returns a user-defined max if defined, otherwise, returns default max for `ty`.
    pub fn max(&self, ty: DispositionType) -> usize {
        self.max_in_auto.unwrap_or(match ty {
            DispositionType::Default => 1,
            DispositionType::Forced => 0,
        })
    }

    pub(crate) fn max_for_stream_type(&self, ty: DispositionType, stream_ty: StreamType) -> usize {
        self.max_in_auto.unwrap_or(match stream_ty {
            StreamType::Attach | StreamType::Font => 0,
            _ => match ty {
                DispositionType::Default => 1,
                DispositionType::Forced => 0,
            },
        })
    }
}
