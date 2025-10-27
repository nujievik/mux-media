use crate::{LangCode, MuxError, RangeU64};
use std::fmt;

/// Media track identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TrackID {
    Num(u64),
    Lang(LangCode),
    Range(RangeU64),
}

impl TrackID {
    /// Returns `true` if the ID is [`TrackID::Range`].
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{LangCode, RangeU64, TrackID};
    /// #
    /// let x = RangeU64::try_from((2, 3)).unwrap();
    /// assert!(TrackID::Range(x).is_range());
    /// assert!(!TrackID::Num(2).is_range());
    /// assert!(!TrackID::Lang(LangCode::Eng).is_range());
    /// ```
    #[inline]
    pub const fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    /// Returns `true` if the given value is contained in the `self`.
    ///
    /// # Examples
    /// ```
    /// # use mux_media::{LangCode, RangeU64, TrackID};
    /// #
    /// let x = TrackID::Num(2);
    /// assert!(x.contains(&x));
    /// assert!(!x.contains(&TrackID::Num(3)));
    ///
    /// let x = TrackID::Lang(LangCode::Eng);
    /// assert!(x.contains(&x));
    /// assert!(!x.contains(&TrackID::Lang(LangCode::Rus)));
    ///
    /// let a = TrackID::Range(RangeU64::try_from((2, 5)).unwrap());
    /// let b = TrackID::Range(RangeU64::try_from((3, 4)).unwrap());
    /// let c = TrackID::Range(RangeU64::try_from((1, 5)).unwrap());
    /// assert!(a.contains(&a));
    /// assert!(a.contains(&b));
    /// assert!(!a.contains(&c));
    /// assert!(a.contains(&TrackID::Num(2)));
    /// assert!(a.contains(&TrackID::Num(5)));
    /// assert!(!a.contains(&TrackID::Num(6)));
    /// assert!(!a.contains(&TrackID::Lang(LangCode::Eng)));
    /// ```
    pub fn contains(&self, other: &Self) -> bool {
        match self {
            Self::Num(_) => self == other,
            Self::Lang(_) => self == other,
            Self::Range(rng) => match other {
                Self::Num(n) => rng.contains(n),
                Self::Lang(_) => false,
                Self::Range(id_rng) => rng.contains_range(id_rng),
            },
        }
    }
}

impl std::str::FromStr for TrackID {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(n) = s.parse::<u64>() {
            return Ok(Self::Num(n));
        }

        if let Ok(rng) = s.parse::<RangeU64>() {
            return Ok(Self::Range(rng));
        }

        match s.parse::<LangCode>() {
            Ok(code) => Ok(Self::Lang(code)),
            Err(_) => Err(err!(
                "Invalid track ID '{}' (must be num, range (n-m) of num or lang code)",
                s
            )),
        }
    }
}

impl fmt::Display for TrackID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(num) => write!(f, "{}", num),
            Self::Lang(lang) => write!(f, "{}", lang),
            Self::Range(rng) => write!(f, "{}", rng),
        }
    }
}
