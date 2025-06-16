mod iter;
mod max_value;

use crate::{MaxValue, MuxError, ToMkvmergeArg};
use std::fmt;
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T>
where
    T: PartialOrd + Copy,
{
    pub fn contains(&self, value: T) -> bool {
        value >= self.start && value <= self.end
    }

    pub fn contains_range(&self, rng: &Self) -> bool {
        rng.start >= self.start && rng.end <= self.end
    }
}

impl<T> ToMkvmergeArg for Range<T>
where
    T: ToString + Copy + PartialOrd + From<u8> + Add<Output = T>,
{
    fn to_mkvmerge_arg(&self) -> String {
        self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl<T> FromStr for Range<T>
where
    T: Copy + Default + PartialOrd + ToString + fmt::Debug + Add + FromStr + MaxValue,
    T::Err: fmt::Display,
{
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (start, end) = match detect_delimiter(s) {
            Some((delimiter, count)) if count > 1 => {
                return Err(MuxError::from(format!(
                    "Too many '{}' delimiters in input: '{}'",
                    delimiter, s
                )));
            }
            Some((delimiter, _)) => parse_with_delimiter::<T>(s, delimiter)?,
            None => parse_single_or_empty::<T>(s)?,
        };

        if end < start {
            return Err(MuxError::from(format!(
                "End of range ({:?}) must be greater than or equal to start ({:?})",
                end, start
            )));
        }

        Ok(Self { start, end })
    }
}

fn detect_delimiter(s: &str) -> Option<(&str, usize)> {
    for delimiter in &["-", "..", ","] {
        if s.contains(delimiter) {
            return Some((delimiter, s.matches(delimiter).count()));
        }
    }
    None
}

fn parse_with_delimiter<T>(s: &str, delimiter: &str) -> Result<(T, T), MuxError>
where
    T: FromStr + Default + MaxValue,
    T::Err: fmt::Display,
{
    let parts: Vec<&str> = s.splitn(2, delimiter).collect();

    let start_str = parts.get(0).copied().unwrap_or("").trim();
    let end_str = parts.get(1).copied().unwrap_or("").trim();

    let start = parse_part::<T>(start_str, T::default())?;
    let end = parse_part::<T>(end_str, T::MAX)?;

    Ok((start, end))
}

fn parse_single_or_empty<T>(s: &str) -> Result<(T, T), MuxError>
where
    T: FromStr + Default + MaxValue,
    T::Err: fmt::Display,
{
    if s.is_empty() {
        Ok((T::default(), T::MAX))
    } else {
        let start = parse_part::<T>(s, T::MAX)?;
        Ok((start, T::MAX))
    }
}

fn parse_part<T>(part: &str, dflt: T) -> Result<T, MuxError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    if part.is_empty() {
        Ok(dflt)
    } else {
        part.parse::<T>()
            .map_err(|e| MuxError::from(format!("invalid value '{}': {}", part, e)))
    }
}
