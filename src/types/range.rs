use crate::{MuxError, Result};
use std::{fmt, ops, str::FromStr};

const MAX_MINUS_ONE: usize = !0 - 1;

/// A wrapper around [`Range<usize>`](ops::Range<usize>).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RangeUsize(ops::Range<usize>);

deref_singleton_tuple_struct!(RangeUsize, ops::Range<usize>);

impl fmt::Display for RangeUsize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0.start, self.0.end - 1)
    }
}

impl Iterator for RangeUsize {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl TryFrom<(usize, usize)> for RangeUsize {
    type Error = MuxError;

    fn try_from(start_end: (usize, usize)) -> Result<Self> {
        let (start, mut end) = start_end;

        if end < start {
            return Err(err!(
                "End of range ({}) must be greater than or equal to start ({})",
                end,
                start
            ));
        }

        if end == usize::MAX {
            return Err(err!(
                "End of range ({}) must be lesser than MAX ({})",
                end,
                usize::MAX
            ));
        } else {
            end += 1;
        }

        Ok(Self(ops::Range { start, end }))
    }
}

impl FromStr for RangeUsize {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();

        let (start, end) = match detect_delimiter(s) {
            Some((delimiter, count)) if count > 1 => {
                return Err(err!(
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

        fn parse_with_delimiter(s: &str, delimiter: &str) -> Result<(usize, usize)> {
            let mut ps = s.splitn(2, delimiter);
            let start = parse_part(ps.next().unwrap_or(""), 0)?;
            let end = parse_part(ps.next().unwrap_or(""), MAX_MINUS_ONE)?;
            Ok((start, end))
        }

        fn parse_single_or_empty(s: &str) -> Result<(usize, usize)> {
            let start = parse_part(s, 0)?;
            Ok((start, MAX_MINUS_ONE))
        }

        fn parse_part(part: &str, val_on_empty: usize) -> Result<usize> {
            if part.is_empty() {
                Ok(val_on_empty)
            } else {
                part.parse::<usize>()
                    .map_err(|e| err!("invalid value '{}': {}", part, e))
            }
        }
    }
}
