use crate::{IsDefault, MuxError, Result, Time, ToTxtConfig, dashed};
use std::{ffi::OsString, str::FromStr};
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
    fn append_args(&self, args: &mut Vec<OsString>) {
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

            args.push(dashed!(Chapters).into());
            args.push(arg.into());
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
                .ok_or_else(|| err!("must be start-end time range"))?;

            let (title, s_start) = if s_start.split(':').count() == 4 {
                s_start.split_once(':').unwrap()
            } else {
                ("", s_start)
            };

            let start = parse_time(s_start)?;
            let end = parse_time(s_end)?;
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
