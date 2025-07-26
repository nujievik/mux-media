use std::collections::HashMap;

/// Checks whether a value is equal to its type's default.
pub trait IsDefault {
    /// Returns `true` if `self` is equal to the default value for its type.
    fn is_default(&self) -> bool;
}

impl IsDefault for bool {
    #[inline(always)]
    fn is_default(&self) -> bool {
        !self
    }
}

impl<T> IsDefault for Option<T> {
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

impl<K, V> IsDefault for HashMap<K, V> {
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
