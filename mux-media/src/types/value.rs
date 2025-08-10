use std::{fmt, ops::Deref};

/// A value marked as either auto or user-defined.
#[derive(Clone, Debug)]
pub enum Value<T> {
    Auto(T),
    User(T),
}

impl<T> Value<T> {
    /// Returns a reference to the internal value.
    #[inline]
    pub const fn inner(&self) -> &T {
        match self {
            Value::Auto(v) | Value::User(v) => v,
        }
    }

    /// Consumes the enum and returns the internal value.
    #[inline]
    pub fn into_inner(self) -> T {
        match self {
            Value::Auto(v) | Value::User(v) => v,
        }
    }

    /// Returns `true` if the value is a [`Value::Auto`].
    ///
    /// # Examples
    /// ```
    /// # use mux_media::Value;
    /// #
    /// let x: Value<u32> = Value::Auto(2);
    /// assert_eq!(x.is_auto(), true);
    ///
    /// let x: Value<u32> = Value::User(2);
    /// assert_eq!(x.is_auto(), false);
    /// ```
    #[inline]
    pub const fn is_auto(&self) -> bool {
        matches!(self, Value::Auto(_))
    }

    /// Returns `true` if the value is a [`Value::User`].
    ///
    /// # Examples
    /// ```
    /// # use mux_media::Value;
    /// #
    /// let x: Value<u32> = Value::User(2);
    /// assert_eq!(x.is_user(), true);
    ///
    /// let x: Value<u32> = Value::Auto(2);
    /// assert_eq!(x.is_user(), false);
    /// ```
    #[inline]
    pub const fn is_user(&self) -> bool {
        !self.is_auto()
    }
}

impl<T> Deref for Value<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<T: fmt::Display> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}
