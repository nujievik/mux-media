use super::*;
use crate::DispositionType;

impl Dispositions {
    pub(crate) fn max(&self, ty: DispositionType) -> usize {
        self.max_in_auto.unwrap_or(match ty {
            DispositionType::Default => 1,
            DispositionType::Forced => 0,
        })
    }
}
