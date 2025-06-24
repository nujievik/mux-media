use crate::{MuxError, Range};
use std::str::FromStr;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum AttachID {
    Num(u64),
    Range(Range<u64>),
}

impl AttachID {
    pub fn contains(&self, id: &Self) -> bool {
        match self {
            Self::Num(_) => self == id,
            Self::Range(rng) => match id {
                Self::Num(n) => rng.contains(*n),
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
            match n != 0 {
                true => Ok(Self::Num(n)),
                false => Err(format!("Attach ID '{}' must be >= 1", s).into()),
            }
        } else if let Ok(rng) = Range::<u64>::from_str(s) {
            match rng.start != 0 && rng.end != 0 {
                true => Ok(Self::Range(rng)),
                false => Err(format!("Attach ID '{}' must be >= 1", s).into()),
            }
        } else {
            Err(format!("Attach ID '{}' must be num or range (n-m) of num", s).into())
        }
    }
}
