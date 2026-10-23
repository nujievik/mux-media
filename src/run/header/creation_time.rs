use core::fmt::NumBuffer;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CreationTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,

    num_buf: NumBuffer<u16>,
    buf: [u8; 20],
}

impl CreationTime {
    pub fn now() -> Option<CreationTime> {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).ok()?;

        let duration_secs = duration.as_secs();
        let duration_mins = duration_secs / 60;
        let duration_hours = duration_mins / 60;

        let second = (duration_secs % 60) as u8;
        let minute = (duration_mins % 60) as u8;
        let hour = (duration_hours % 24) as u8;

        // 1970 is start UNIX_EPOCH
        let mut year: u16 = 1970;
        let mut days = duration_hours / 24;

        let is_leap = |year| (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

        loop {
            let is_leap = is_leap(year);
            let days_in_year = if is_leap { 366 } else { 365 };

            if days >= days_in_year {
                days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        let mut month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if is_leap(year) {
            month_days[1] = 29;
        }

        let mut month = 1;
        for &days_in_month in month_days.iter() {
            if days >= days_in_month {
                days -= days_in_month;
                month += 1;
            } else {
                break;
            }
        }

        let day = (days + 1) as u8; // days starts with 1

        Some(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            num_buf: NumBuffer::new(),
            buf: *b"1970-01-01T00:00:00Z",
        })
    }

    pub fn format_to_iso_8601(&mut self) -> &str {
        const BUF_YEAR_POSITION: usize = 0;
        const BUF_MONTH_POSITION: usize = 5;
        const BUF_DAY_POSITION: usize = 8;
        const BUF_HOUR_POSITION: usize = 11;
        const BUF_MINUTE_POSITION: usize = 14;
        const BUF_SECOND_POSITION: usize = 17;

        self.write_four_digits(BUF_YEAR_POSITION, self.year);
        self.write_two_digits(BUF_MONTH_POSITION, self.month as u16);
        self.write_two_digits(BUF_DAY_POSITION, self.day as u16);

        self.write_two_digits(BUF_HOUR_POSITION, self.hour as u16);
        self.write_two_digits(BUF_MINUTE_POSITION, self.minute as u16);
        self.write_two_digits(BUF_SECOND_POSITION, self.second as u16);

        // SAFETY: only ascii bytes
        unsafe { str::from_utf8_unchecked(&self.buf) }
    }

    fn write_two_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = bytes[0];
            }
            _ => self.buf[start_idx..start_idx + 2].copy_from_slice(&bytes[..2]),
        }
    }

    fn write_four_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = b'0';
                self.buf[start_idx + 2] = b'0';
                self.buf[start_idx + 3] = bytes[0];
            }
            2 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = b'0';
                self.buf[start_idx + 1..start_idx + 4].copy_from_slice(&bytes[..2]);
            }
            3 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1..start_idx + 4].copy_from_slice(&bytes[..3]);
            }
            _ => self.buf[start_idx..start_idx + 4].copy_from_slice(&bytes[..4]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_to_iso_8601() {
        let mut time = CreationTime::now().unwrap();
        let pairs = [
            ("1970-01-01T00:00:00Z", (1970, 1, 1, 0, 0, 0)),
            ("1992-04-08T11:22:33Z", (1992, 4, 8, 11, 22, 33)),
            ("2000-11-12T02:04:08Z", (2000, 11, 12, 2, 4, 8)),
            ("2026-10-23T10:38:00Z", (2026, 10, 23, 10, 38, 0)),
            ("2032-02-04T08:16:32Z", (2032, 2, 4, 8, 16, 32)),
            ("0001-01-01T00:00:00Z", (1, 1, 1, 0, 0, 0)),
        ];

        for (s, (year, month, day, hour, minute, second)) in pairs {
            time.year = year;
            time.month = month;
            time.day = day;
            time.hour = hour;
            time.minute = minute;
            time.second = second;

            assert_eq!(s, time.format_to_iso_8601());
        }
    }

    #[test]
    fn now() {
        let now = CreationTime::now().unwrap();
        assert!(now.year >= 2026);
        assert!(now.month <= 12);
        assert!(now.day <= 31);
        assert!(now.hour <= 24);
        assert!(now.minute <= 59);
        assert!(now.second <= 59);
    }

    #[test]
    fn now_format_to_iso_8601() {
        let mut now = CreationTime::now().unwrap();
        let now_year = now.year;
        let now_month = now.month;
        let now_day = now.day;
        let now_hour = now.hour;
        let now_minute = now.minute;
        let now_second = now.second;
        let bytes = now.format_to_iso_8601().as_bytes();

        assert_eq!(20, bytes.len());

        {
            let year = unsafe { str::from_utf8_unchecked(&bytes[..4]) };
            let year = year.parse::<u16>().unwrap();
            assert_eq!(now_year, year);
        }

        assert_eq!(b'-', bytes[4]);

        {
            let month = unsafe { str::from_utf8_unchecked(&bytes[5..7]) };
            let month = month.parse::<u8>().unwrap();
            assert_eq!(now_month, month);
        }

        assert_eq!(b'-', bytes[7]);

        {
            let day = unsafe { str::from_utf8_unchecked(&bytes[8..10]) };
            let day = day.parse::<u8>().unwrap();
            assert_eq!(now_day, day);
        }

        assert_eq!(b'T', bytes[10]);

        {
            let hour = unsafe { str::from_utf8_unchecked(&bytes[11..13]) };
            let hour = hour.parse::<u8>().unwrap();
            assert_eq!(now_hour, hour);
        }

        assert_eq!(b':', bytes[13]);

        {
            let minute = unsafe { str::from_utf8_unchecked(&bytes[14..16]) };
            let minute = minute.parse::<u8>().unwrap();
            assert_eq!(now_minute, minute);
        }

        assert_eq!(b':', bytes[16]);

        {
            let second = unsafe { str::from_utf8_unchecked(&bytes[17..19]) };
            let second = second.parse::<u8>().unwrap();
            assert_eq!(now_second, second);
        }

        assert_eq!(b'Z', bytes[19]);
    }
}
