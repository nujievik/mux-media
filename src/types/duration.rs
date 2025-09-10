use crate::{IsDefault, MuxError, mux_err};
use std::{fmt, ops::Add, str::FromStr, time};

/// A wrapper around [`std::time::Duration`].
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, IsDefault)]
pub struct Duration(pub time::Duration);

crate::deref_singleton_tuple_struct!(Duration, time::Duration);

impl Duration {
    pub const fn new(secs: u64, nanos: u32) -> Duration {
        Self(time::Duration::new(secs, nanos))
    }

    pub(crate) fn from_secs_f64(secs: f64) -> Duration {
        Self(time::Duration::from_secs_f64(secs))
    }
}

impl From<time::Duration> for Duration {
    fn from(d: time::Duration) -> Duration {
        Self(d)
    }
}
impl From<Duration> for time::Duration {
    fn from(d: Duration) -> time::Duration {
        d.0
    }
}

impl From<::time::Time> for Duration {
    fn from(t: ::time::Time) -> Duration {
        let (h, m, s, nanos) = t.as_hms_nano();
        let secs = h as u64 * 3600 + m as u64 * 60 + s as u64;
        Duration::new(secs, nanos)
    }
}

impl From<Duration> for ::time::Time {
    fn from(d: Duration) -> ::time::Time {
        ::time::Time::MIDNIGHT + time::Duration::from(d)
    }
}

impl FromStr for Duration {
    type Err = MuxError;

    /// Attempts construct [`Duration`] from str of view HH:MM:SS.nnnnnnnnn
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.matches(':').count() != 2 {
            return Err(mux_err!("Invalid time str ({}), must be HH:MM:SS[.nn]", s));
        }

        let mut parts = s.split(':');
        let hours = parts.next().unwrap().parse::<u64>()?;
        let minutes = parts.next().unwrap().parse::<u64>()?;

        let mut sec_parts = parts.next().unwrap().splitn(2, '.');

        let seconds = sec_parts.next().unwrap().parse::<u64>()?;

        let nanos = if let Some(ns) = sec_parts.next() {
            let mut ns = ns.to_string();
            if ns.len() > 9 {
                ns.truncate(9);
            } else {
                while ns.len() < 9 {
                    ns.push('0');
                }
            }
            ns.parse::<u32>()?
        } else {
            0
        };

        let total = time::Duration::new(hours * 3600 + minutes * 60 + seconds, nanos);
        Ok(Self(total))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(*self + *other)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hours, minutes, seconds) = to_hours_minutes_seconds(self);
        let nanos = self.subsec_nanos();
        write!(f, "{:02}:{:02}:{:02}.{:09}", hours, minutes, seconds, nanos)
    }
}

fn to_hours_minutes_seconds(d: &Duration) -> (u64, u8, u8) {
    let secs = d.as_secs();
    let h = secs / 3600;
    let m = ((secs % 3600) / 60) as u8;
    let s = (secs % 60) as u8;
    (h, m, s)
}
