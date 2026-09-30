use crate::{IsDefault, MuxError, Result, Time, ToTxtConfig, dashed};
use std::str::FromStr;
use subtitle_lines::vtt::VttTimeBuf;

/// A chapters configuration.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
#[non_exhaustive]
pub struct ConfigChapters {
    pub no_flag: bool,
    pub ranges: Option<Vec<ConfigChaptersTimeRange>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigChaptersTimeRange {
    pub(crate) title: Option<String>,
    pub(crate) start: Time,
    pub(crate) end: Time,
}

impl ToTxtConfig for ConfigChapters {
    fn append_args(&self, args: &mut Vec<String>) {
        if self.no_flag {
            args.push(dashed!(NoChapters).into());
            return;
        }

        if let Some(ranges) = self.ranges.as_ref() {
            let mut buf = VttTimeBuf::new();

            let mut arg = Vec::with_capacity(ranges.len() * 14);
            for r in ranges {
                if let Some(title) = r.title.as_ref() {
                    arg.extend_from_slice(title.as_bytes());
                    arg.push(b':')
                }
                arg.extend_from_slice(buf.format_time(r.start));
                arg.push(b'-');
                arg.extend_from_slice(buf.format_time(r.end));
                arg.push(b',');
            }
            let _ = arg.pop();
            let arg = unsafe { String::from_utf8_unchecked(arg) };

            args.push(to_args!(Chapters));
            args.push(arg);
        }
    }
}

impl FromStr for ConfigChapters {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Self> {
        let mut ranges: Vec<ConfigChaptersTimeRange> = Vec::new();

        for s_range in s.split(',') {
            let (s_start, s_end) = s_range
                .split_once('-')
                .ok_or_else(|| err!("must be start-end time range ({})", s_range))?;

            let (title, s_start) = if s_start.split(':').count() == 4 {
                s_start.split_once(':').unwrap()
            } else {
                ("", s_start)
            };

            let start = parse_time(s_start)?;
            let end = parse_time(s_end)?;

            if start >= end {
                return Err(err!("start ({}) must be lesser end ({})", s_start, s_end));
            }

            let title = if title.is_empty() {
                None
            } else {
                Some(title.into())
            };

            ranges.push(ConfigChaptersTimeRange { title, start, end });
        }

        Ok(ConfigChapters {
            no_flag: false,
            ranges: Some(ranges),
        })
    }
}

fn parse_time(s: &str) -> Result<Time> {
    let err = || err!("must be hh:mm:ss.mms time format");

    let (s_remainder, s_millis) = s.split_once('.').ok_or_else(err)?;
    let millis = u16::from_str(s_millis)?;

    let mut it = s_remainder.split(':');
    let mut next = || it.next().ok_or_else(err);

    let hours = u16::from_str(next()?)?;
    let mins = u8::from_str(next()?)?;
    let secs = u8::from_str(next()?)?;

    if it.next().is_some() {
        return Err(err());
    }

    let time = Time::new(hours, mins, secs, millis)?;

    Ok(time)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_time_str() {
        const TEST_SET: [(&str, Time); 3] = [
            ("00:00:00.000", Time::new_unchecked(0, 0, 0, 0)),
            ("1:2:3.4", Time::new_unchecked(1, 2, 3, 4)),
            ("12:23:45.678", Time::new_unchecked(12, 23, 45, 678)),
        ];

        for (s, t) in TEST_SET {
            assert_eq!(parse_time(s).unwrap(), t);
        }
    }

    #[test]
    fn parse_time_str_invalid() {
        const TEST_SET: &[&str] = &["00", "00:00", "00:00:00", "00:00:00,000"];

        for s in TEST_SET {
            assert!(parse_time(s).is_err());
        }
    }

    #[test]
    fn parse_range() {
        let v = ConfigChapters::from_str("0:0:0.0-12:23:45.678")
            .unwrap()
            .ranges
            .unwrap();
        assert!(v.len() == 1);
        let v = &v[0];

        assert_eq!(v.title, None);
        assert_eq!(v.start, Time::new_unchecked(0, 0, 0, 0));
        assert_eq!(v.end, Time::new_unchecked(12, 23, 45, 678));
    }

    #[test]
    fn parse_range_with_name() {
        let v = ConfigChapters::from_str("NAME:0:0:0.0-12:23:45.678")
            .unwrap()
            .ranges
            .unwrap();
        assert!(v.len() == 1);
        let v = &v[0];

        assert_eq!(v.title, Some(String::from("NAME")));
        assert_eq!(v.start, Time::new_unchecked(0, 0, 0, 0));
        assert_eq!(v.end, Time::new_unchecked(12, 23, 45, 678));
    }

    #[test]
    fn parse_ranges() {
        let v = ConfigChapters::from_str("0:0:0.0-12:23:45.678,12:23:45.678-23:45:55.000")
            .unwrap()
            .ranges
            .unwrap();
        assert!(v.len() == 2);
        let v0 = &v[0];
        let v1 = &v[1];

        assert_eq!(v0.title, None);
        assert_eq!(v0.start, Time::new_unchecked(0, 0, 0, 0));
        assert_eq!(v0.end, Time::new_unchecked(12, 23, 45, 678));

        assert_eq!(v1.title, None);
        assert_eq!(v1.start, Time::new_unchecked(12, 23, 45, 678));
        assert_eq!(v1.end, Time::new_unchecked(23, 45, 55, 0));
    }

    #[test]
    fn parse_invalid_range_start() {
        assert!(ConfigChapters::from_str("12:23:45.678-0:0:0.0").is_err());
    }

    #[test]
    fn parse_invalid_range_format() {
        const SET: &[&str] = &["0:0:0.0", "12:23:45.678-0:0:0.0", "0:0:0.0-12:23:45.678-"];

        for s in SET {
            assert!(ConfigChapters::from_str(s).is_err());
        }
    }
}
