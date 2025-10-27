use crate::{MuxError, RangeU64};
use std::{fmt, str::FromStr};

/// Media attachment identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AttachID {
    Num(u64),
    Range(RangeU64),
}

impl AttachID {
    /// Returns `true` if the ID is [`AttachID::Range`].
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{AttachID, RangeU64};
    /// #
    /// let x = RangeU64::try_from((2, 3)).unwrap();
    /// assert!(AttachID::Range(x).is_range());
    /// assert!(!AttachID::Num(2).is_range());
    /// ```
    #[inline]
    pub const fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    /// Returns `true` if the given value is contained in the `self`.
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{AttachID, RangeU64};
    /// #
    /// let x = AttachID::Num(2);
    /// assert!(x.contains(&x));
    /// assert!(!x.contains(&AttachID::Num(3)));
    ///
    /// let a = AttachID::Range(RangeU64::try_from((2, 5)).unwrap());
    /// let b = AttachID::Range(RangeU64::try_from((3, 4)).unwrap());
    /// let c = AttachID::Range(RangeU64::try_from((1, 5)).unwrap());
    /// assert!(a.contains(&a));
    /// assert!(a.contains(&b));
    /// assert!(!a.contains(&c));
    /// assert!(a.contains(&AttachID::Num(2)));
    /// assert!(a.contains(&AttachID::Num(5)));
    /// assert!(!a.contains(&AttachID::Num(6)));
    /// ```
    pub fn contains(&self, other: &Self) -> bool {
        match self {
            Self::Num(_) => self == other,
            Self::Range(rng) => match other {
                Self::Num(n) => rng.contains(n),
                Self::Range(id_rng) => rng.contains_range(id_rng),
            },
        }
    }
}

impl FromStr for AttachID {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(n) = s.parse::<u64>() {
            return if n != 0 {
                Ok(Self::Num(n))
            } else {
                Err(err!("Attach ID '{}' must be >= 1", s))
            };
        }

        if let Ok(rng) = s.parse::<RangeU64>() {
            return if rng.start != 0 {
                Ok(Self::Range(rng))
            } else {
                Err(err!("Attach ID '{}' must be >= 1", s))
            };
        }

        Err(err!("Attach ID '{}' must be num or range (n-m) of num", s))
    }
}

impl fmt::Display for AttachID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(num) => write!(f, "{}", num),
            Self::Range(rng) => write!(f, "{}", rng),
        }
    }
}
