use std::ffi::OsStr;

/// Extracts an ascii-digit number from file names.
///
/// Defines number:
/// - by default: as first ascii-digit subsequence.
/// - after [`FileNumber::update`]: as first different ascii-digit subsequence between two names
/// OR by default (if not different ascii-digit subsequence).
///
/// # Limits
/// Correct works with standard path length only (255 ascii-digits max). In other cases,
/// processes first 255 ascii-digits only.
#[derive(Clone)]
pub struct FileNumber {
    // ranges stores input (start, end) values, not digits_buf
    ranges: [(u16, u16); Self::MAX_NUMBER_NUMBERS],
    ranges_idx: u16,

    // ascii digit bytes only
    digits_buf: [u8; Self::MAX_ASCII_DIGITS],
}

impl FileNumber {
    /// Max digits count in most filesystems is 255 ascii bytes.
    const MAX_ASCII_DIGITS: usize = u8::MAX as usize;

    /// Max file name in most filesystems is 255 ascii bytes. Minimum digit range is 2 bytes
    /// (digit + sep). Therefore MAX_NUMBER_NUMBERS is 128.
    const MAX_NUMBER_NUMBERS: usize = 128;

    /// Returns the number string if present; otherwise, returns an empty string.
    pub fn as_str(&self) -> &str {
        // SAFETY: ascii digit bytes only
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Gets `usize` number if parsable.
    pub fn get_usize(&self) -> Option<usize> {
        self.as_str().parse::<usize>().ok()
    }

    /// Constructs a new [`FileNumber`] from a byte slice.
    ///
    /// Sets number as first ascii-digit subsequence (if present); otherwise, as empty string.
    pub fn from_bytes<B>(bytes: &B) -> FileNumber
    where
        B: AsRef<[u8]> + ?Sized,
    {
        FileNumber::from(bytes.as_ref())
    }

    /// Updates the number from `new` bytes.
    ///
    /// # Cases
    /// - New bytes contain correct and different number at the old address:
    /// copies only this number bytes.
    ///
    /// - New bytes contain correct and different number at the any other old address:
    /// copies this number bytes and updates index number.
    ///
    /// - Otherwise, its eq constructs new [`FileNumber`].
    pub fn update<B>(&mut self, new: &B)
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let new_bytes = new.as_ref();

        let is_updated_only_number_and_idx = |current: &mut FileNumber, ranges_idx| -> bool {
            let (start, end) = current.ranges[ranges_idx];
            let end = end as usize;

            if end <= new_bytes.len() {
                let maybe_new_number = &new_bytes[start as usize..end];
                let mut i = 0;

                for (old, n) in current
                    .as_bytes_idx(ranges_idx)
                    .iter()
                    .zip(maybe_new_number.iter())
                {
                    if n != old {
                        let remainder = &maybe_new_number[i..];

                        return if remainder.iter().all(|b| b.is_ascii_digit()) {
                            current.ranges_idx = ranges_idx as u16;
                            current.as_mut_bytes_idx(ranges_idx)[i..].copy_from_slice(remainder);
                            true
                        } else {
                            false
                        };
                    }
                    i += 1;
                }
            }

            false
        };

        let ranges_idx = self.ranges_idx as usize;

        if is_updated_only_number_and_idx(self, ranges_idx) {
            return;
        }

        for idx in 0..self.ranges.len() {
            if idx == ranges_idx {
                continue;
            }
            if self.ranges[idx].1 == 0 {
                break;
            }

            if is_updated_only_number_and_idx(self, idx) {
                return;
            }
        }

        *self = FileNumber::from(new_bytes);
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes_idx(self.ranges_idx as usize)
    }

    fn as_bytes_idx(&self, idx: usize) -> &[u8] {
        let (start, end) = self.digit_start_end_idx(idx);
        &self.digits_buf[start..end]
    }

    fn as_mut_bytes_idx(&mut self, idx: usize) -> &mut [u8] {
        let (start, end) = self.digit_start_end_idx(idx);
        &mut self.digits_buf[start..end]
    }

    fn digit_start_end_idx(&self, idx: usize) -> (usize, usize) {
        let mut start = 0;
        let mut end = 0;

        for (r_start, r_end) in self.ranges.iter().take(idx + 1) {
            start = end;
            end += (r_end - r_start) as usize;
        }

        (start, end)
    }
}

impl From<&OsStr> for FileNumber {
    fn from(value: &OsStr) -> Self {
        value.as_encoded_bytes().into()
    }
}

impl From<&[u8]> for FileNumber {
    fn from(bytes: &[u8]) -> Self {
        let mut ranges = [(0u16, 0u16); Self::MAX_NUMBER_NUMBERS];
        let mut i_ranges = 0;
        let mut count_digits = 0;

        let mut start = None;

        for (i, byte) in bytes.into_iter().enumerate() {
            match (byte.is_ascii_digit(), start) {
                (true, None) => start = Some(i),
                (false, Some(st)) => {
                    if i > u16::MAX as usize {
                        break;
                    }

                    count_digits += i - st;
                    if count_digits > Self::MAX_ASCII_DIGITS {
                        break;
                    }

                    ranges[i_ranges] = (st as u16, i as u16);
                    i_ranges += 1;
                    start = None;
                }
                _ => continue,
            }
        }

        let mut digits_buf = [0u8; Self::MAX_ASCII_DIGITS];
        let mut i = 0;

        ranges
            .iter()
            .take_while(|(_, end)| *end > 0)
            .for_each(|&(start, end)| {
                let (start, end) = (start as usize, end as usize);
                let len = end - start;
                let next_i = i + len;

                digits_buf[i..next_i].copy_from_slice(&bytes[start..end]);
                i = next_i;
            });

        FileNumber {
            digits_buf,
            ranges,
            ranges_idx: 0,
        }
    }
}
