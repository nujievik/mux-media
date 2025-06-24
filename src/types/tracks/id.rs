use crate::{LangCode, MuxError, Range};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum TrackID {
    Num(u64),
    Lang(LangCode),
    Range(Range<u64>),
}

impl Default for TrackID {
    fn default() -> Self {
        Self::Num(0)
    }
}

impl TrackID {
    pub fn is_range(&self) -> bool {
        match self {
            TrackID::Range(_) => true,
            _ => false,
        }
    }

    pub fn contains(&self, id: &TrackID) -> bool {
        match self {
            Self::Num(_) => self == id,
            Self::Lang(_) => self == id,
            Self::Range(rng) => match id {
                Self::Num(n) => rng.contains(*n),
                Self::Lang(_) => false,
                Self::Range(id_rng) => rng.contains_range(id_rng),
            },
        }
    }
}

impl crate::ToMkvmergeArg for TrackID {
    fn to_mkvmerge_arg(&self) -> String {
        match self {
            Self::Num(n) => n.to_string(),
            Self::Lang(code) => code.to_string(),
            Self::Range(rng) => rng.to_mkvmerge_arg(),
        }
    }
}

impl std::str::FromStr for TrackID {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(n) = s.parse::<u64>() {
            Ok(Self::Num(n))
        } else if let Ok(rng) = s.parse::<Range<u64>>() {
            Ok(Self::Range(rng))
        } else {
            match LangCode::from_str(s) {
                Ok(code) => Ok(Self::Lang(code)),
                Err(_) => Err(format!(
                    "Invalid track ID '{}' (must be num, range (n-m) of num or lang code)",
                    s
                )
                .into()),
            }
        }
    }
}
