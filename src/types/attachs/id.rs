use crate::{MuxError, Range};
use std::str::FromStr;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum AttachID {
    U32(u32),
    Range(Range<u32>),
}

impl AttachID {
    pub fn contains(self, id: Self) -> bool {
        match self {
            Self::U32(_) => self == id,
            Self::Range(rng) => match id {
                Self::U32(n) => rng.contains(n),
                Self::Range(id_rng) => rng.contains_range(id_rng),
            },
        }
    }
}

impl FromStr for AttachID {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(n) = s.parse::<u32>() {
            match n != 0 {
                true => Ok(Self::U32(n)),
                false => Err(format!("Attach ID '{}' must be >= 1", s).into()),
            }
        } else if let Ok(rng) = Range::<u32>::from_str(s) {
            match rng.start != 0 && rng.end != 0 {
                true => Ok(Self::Range(rng)),
                false => Err(format!("Attach ID '{}' must be >= 1", s).into()),
            }
        } else {
            Err(format!("Attach ID '{}' must be num or range (n-m) of num", s).into())
        }
    }
}
