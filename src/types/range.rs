use crate::{MuxError, Result};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RangeUsizeInclusive {
    start: usize,
    end: usize,
}

impl RangeUsizeInclusive {
    /// Tries construct new [`RangeUsizeInclusive`] from `start` and `end`.
    ///
    /// # Errors
    /// Returns error **only if** `end` < `start`.
    #[inline]
    pub fn try_new(start: usize, end: usize) -> Result<RangeUsizeInclusive> {
        if end >= start {
            Ok(Self { start, end })
        } else {
            Err(err!("end of range must be >= start"))
        }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn contains(&self, num: &usize) -> bool {
        *num >= self.start && *num <= self.end
    }
}

impl FromStr for RangeUsizeInclusive {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<RangeUsizeInclusive> {
        let (start, end) = match s.split_once('-') {
            Some(xs) => xs,
            None => {
                let start = s.parse::<usize>()?;
                return Ok(RangeUsizeInclusive {
                    start,
                    end: usize::MAX,
                });
            }
        };

        let start = if start.is_empty() {
            0
        } else {
            start.parse::<usize>()?
        };

        let end = if end.is_empty() {
            usize::MAX
        } else {
            end.parse::<usize>()?
        };

        RangeUsizeInclusive::try_new(start, end)
    }
}
