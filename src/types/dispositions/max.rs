use super::*;
use crate::DispositionType;

impl Dispositions {
    /// Returns a user-defined max if defined, otherwise, returns default max for `ty`.
    pub fn max(&self, ty: DispositionType) -> usize {
        self.max_in_auto.unwrap_or(match ty {
            DispositionType::Default => 1,
            DispositionType::Forced => 0,
        })
    }
}
