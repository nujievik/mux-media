use crate::IsDefault;

/// A wrapper around `bool`.
#[derive(Copy, Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Bool(pub bool);

deref_singleton_tuple_struct!(Bool, bool);

impl Bool {
    /// # Examples
    /// ```
    /// use mux_media::Bool;
    ///
    /// assert_eq!("true", Bool(true).as_str());
    /// assert_eq!("false", Bool(false).as_str());
    /// ```
    pub const fn as_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }
}
