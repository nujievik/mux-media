use crate::{MuxError, Result, mux_err};
use std::{fmt, ops, str::FromStr};

const MAX_MINUS_ONE: u64 = !0 - 1;

/// A wrapper around [`Range<u64>`](ops::Range<u64>).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RangeU64(ops::Range<u64>);

crate::deref_singleton_tuple_struct!(RangeU64, ops::Range<u64>);

impl RangeU64 {
    pub fn contains_range(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }
}

impl fmt::Display for RangeU64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0.start, self.0.end - 1)
    }
}

impl Iterator for RangeU64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl TryFrom<(u64, u64)> for RangeU64 {
    type Error = MuxError;

    fn try_from(start_end: (u64, u64)) -> Result<Self> {
        let (start, mut end) = start_end;

        if end < start {
            return Err(mux_err!(
                "End of range ({}) must be greater than or equal to start ({})",
                end,
                start
            ));
        }

        if end == u64::MAX {
            return Err(mux_err!(
                "End of range ({}) must be lesser than MAX ({})",
                end,
                u64::MAX
            ));
        } else {
            end += 1;
        }

        Ok(Self(ops::Range { start, end }))
    }
}

impl FromStr for RangeU64 {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        let (start, end) = match detect_delimiter(s) {
            Some((delimiter, count)) if count > 1 => {
                return Err(mux_err!(
                    "Too many '{}' delimiters in input: '{}'",
                    delimiter,
                    s
                ));
            }
            Some((delimiter, _)) => parse_with_delimiter(s, delimiter)?,
            None => parse_single_or_empty(s)?,
        };

        return Self::try_from((start, end));

        fn detect_delimiter(s: &str) -> Option<(&str, usize)> {
            for delimiter in &["-", ",", "..="] {
                if s.contains(delimiter) {
                    return Some((delimiter, s.matches(delimiter).count()));
                }
            }
            None
        }

        fn parse_with_delimiter(s: &str, delimiter: &str) -> Result<(u64, u64)> {
            let mut ps = s.splitn(2, delimiter);
            let start = parse_part(ps.next().unwrap_or(""), 0)?;
            let end = parse_part(ps.next().unwrap_or(""), MAX_MINUS_ONE)?;
            Ok((start, end))
        }

        fn parse_single_or_empty(s: &str) -> Result<(u64, u64)> {
            let start = parse_part(s, 0)?;
            Ok((start, MAX_MINUS_ONE))
        }

        fn parse_part(part: &str, val_on_empty: u64) -> Result<u64> {
            if part.is_empty() {
                Ok(val_on_empty)
            } else {
                part.parse::<u64>()
                    .map_err(|e| mux_err!("invalid value '{}': {}", part, e))
            }
        }
    }
}
