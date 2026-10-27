use std::ffi::OsStr;

/// Extracts a number from byte slices.
///
/// Defined number:
/// - if previous known: diff ascii-digit subsequence in previous and current byte slice.
/// - if previous unknown: first ascii-digit subsequence in current byte slice.
#[derive(Clone)]
pub struct MediaNumber {
    // ranges stores input (start, end) values, not digits_buf
    ranges: [(u8, u8); Self::MAX_NUMBER_NUMBERS],
    ranges_idx: u8,

    // ascii digit bytes only
    digits_buf: [u8; Self::MAX_ASCII_FILENAME_LEN],
}

impl MediaNumber {
    /// Max file name in most filesystems is 255 ascii bytes.
    const MAX_ASCII_FILENAME_LEN: usize = u8::MAX as usize;

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

    /// Updates the number from `new` bytes.
    ///
    /// # Cases
    /// - New bytes contain correct and different number at the old address:
    /// copies only this number bytes.
    ///
    /// - New bytes contain correct and different number at the any other old address:
    /// copies this number bytes and updates index number.
    ///
    /// - Otherwise, its eq constructs new [`MediaNumber`].
    pub fn upd(&mut self, new: &OsStr) {
        let new_bytes = new.as_encoded_bytes();

        let is_updated_only_number_and_idx = |current: &mut MediaNumber, ranges_idx| -> bool {
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
                            current.ranges_idx = ranges_idx as u8;
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

        *self = MediaNumber::from(new_bytes);
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes_idx(self.ranges_idx as usize)
    }

    fn as_bytes_idx(&self, idx: usize) -> &[u8] {
        let mut start = 0;
        let mut end = 0;

        for (r_start, r_end) in self.ranges.iter().take(idx + 1) {
            start = end;
            end += r_end - r_start;
        }

        &self.digits_buf[start as usize..end as usize]
    }

    fn as_mut_bytes_idx(&mut self, idx: usize) -> &mut [u8] {
        let mut start = 0;
        let mut end = 0;

        for (r_start, r_end) in self.ranges.iter().take(idx + 1) {
            start = end;
            end += r_end - r_start;
        }

        &mut self.digits_buf[start as usize..end as usize]
    }
}

impl From<&OsStr> for MediaNumber {
    /// Constructs a new [`Self`] from [`OsStr`].
    ///
    /// Sets to number string the first number found (if any); otherwise, an empty string.
    fn from(value: &OsStr) -> Self {
        value.as_encoded_bytes().into()
    }
}

impl From<&[u8]> for MediaNumber {
    /// Constructs a new [`Self`] from a byte slice.
    ///
    /// Sets to number string the first number found (if any); otherwise, an empty string.
    fn from(bytes: &[u8]) -> Self {
        let mut ranges = [(0u8, 0u8); Self::MAX_NUMBER_NUMBERS];
        let mut ranges_init_len = 0;

        let mut start = None;

        for (i, byte) in bytes.into_iter().enumerate() {
            match (byte.is_ascii_digit(), start) {
                (true, None) => start = Some(i as u8),
                (false, Some(st)) => {
                    if i > Self::MAX_ASCII_FILENAME_LEN {
                        break;
                    }

                    ranges[ranges_init_len] = (st, i as u8);
                    ranges_init_len += 1;

                    if ranges_init_len == Self::MAX_NUMBER_NUMBERS {
                        break;
                    }
                    start = None;
                }
                _ => continue,
            }
        }

        let mut digits_buf = [0u8; Self::MAX_ASCII_FILENAME_LEN];
        let mut i = 0;

        for &(start, end) in &ranges {
            let len = (end - start) as usize;
            let next_i = i + len;

            if next_i > Self::MAX_ASCII_FILENAME_LEN {
                break;
            }

            digits_buf[i..next_i].copy_from_slice(&bytes[start as usize..end as usize]);
            i = next_i;
        }

        MediaNumber {
            digits_buf,
            ranges,
            ranges_idx: 0,
        }
    }
}
