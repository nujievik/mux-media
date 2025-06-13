use crate::{LangCode, MuxError, Range};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TrackID {
    U32(u32),
    Lang(LangCode),
    Range(Range<u32>),
}

impl TrackID {
    pub fn contains(self, id: TrackID) -> bool {
        match self {
            Self::U32(_) => self == id,
            Self::Lang(_) => self == id,
            Self::Range(rng) => match id {
                Self::U32(n) => rng.contains(n),
                Self::Lang(_) => false,
                Self::Range(id_rng) => rng.contains_range(id_rng),
            },
        }
    }
}

impl FromStr for TrackID {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(n) = s.parse::<u32>() {
            Ok(Self::U32(n))
        } else if let Ok(rng) = Range::<u32>::from_str(s) {
            Ok(Self::Range(rng))
        } else {
            match LangCode::from_str(s) {
                Ok(code) => Ok(Self::Lang(code)),
                Err(_) => Err(MuxError::from(format!(
                    "Invalid track ID '{}' (must be num, range (n-m) of num or lang code)",
                    s
                ))),
            }
        }
    }
}

impl TrackID {
    pub fn is_range(self) -> bool {
        match self {
            TrackID::Range(_) => true,
            _ => false,
        }
    }
}
