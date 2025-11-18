use std::{ffi::OsStr, mem::take};

/// Stores a numbers extracted from [`OsStr`].
///
/// Internally tracks byte length and parsed numeric substrings to support
/// accurate and efficient updates and reuse when possible.
#[derive(Clone)]
pub struct MediaNumber {
    len_raw_bytes: usize,
    s_nums_borders: Vec<(String, usize, usize)>,
    idx: Option<usize>,
    s: String,
}

impl MediaNumber {
    /// Returns the number string if present; otherwise, returns an empty string.
    pub fn as_str(&self) -> &str {
        &self.s
    }

    /// Returns the number as `u64` if parsable; otherwise, returns `0`.
    pub fn to_usize(&self) -> usize {
        self.s.parse::<usize>().unwrap_or(0)
    }

    /// Updates the internal value based on the new [`OsStr`].
    ///
    /// - If the new value has a different byte length, recreates [`Self`] from scratch.
    /// - If lengths match and the stored index is valid, only updates the number string.
    /// - Otherwise, updates all fields, sets number string and index based on mismatched content.
    pub fn upd(&mut self, value: &OsStr) {
        let bytes = value.as_encoded_bytes();
        let len_raw_bytes = bytes.len();

        if self.len_raw_bytes != len_raw_bytes {
            *self = Self::from(bytes);
            return;
        }

        if let Some(s) = self
            .idx
            .and_then(|idx| self.s_nums_borders.get(idx))
            .and_then(|&(_, start, end)| get_s_num(&bytes[start..end]))
        {
            self.s = s;
            return;
        }

        let s_nums_borders = s_nums_borders_from_bytes(bytes);
        let (s, idx) = self.get_s_idx_from_mismatch_num(&s_nums_borders);

        self.s_nums_borders = s_nums_borders;
        self.idx = idx;
        self.s = s;
    }

    #[inline(always)]
    fn get_s_idx_from_mismatch_num(
        &self,
        s_nums_borsers: &[(String, usize, usize)],
    ) -> (String, Option<usize>) {
        s_nums_borsers
            .into_iter()
            .enumerate()
            .find(|(i, vals)| Some(*vals) != self.s_nums_borders.get(*i))
            .map(|(i, vals)| (vals.0.to_string(), Some(i)))
            .unwrap_or(("".to_string(), None))
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
        let s_nums_borders = s_nums_borders_from_bytes(bytes);

        let s = s_nums_borders
            .get(0)
            .map(|(s, _, _)| s.to_string())
            .unwrap_or_default();

        Self {
            len_raw_bytes: bytes.len(),
            s_nums_borders,
            idx: None,
            s,
        }
    }
}

#[inline(always)]
fn s_nums_borders_from_bytes(bytes: &[u8]) -> Vec<(String, usize, usize)> {
    let mut s_nums_borders = Vec::<(String, usize, usize)>::new();

    let mut current = Vec::<u8>::new();
    let mut start = 0;

    bytes.into_iter().enumerate().for_each(|(i, byte)| {
        if byte.is_ascii_digit() {
            current.push(*byte);
            if start == 0 {
                start = i;
            }
        } else if !current.is_empty() {
            // SAFETY: only `is_ascii_digit()` bytes
            let s = unsafe { String::from_utf8_unchecked(take(&mut current)) };
            s_nums_borders.push((s, start, i));
            start = 0;
        }
    });

    s_nums_borders
}

#[inline(always)]
fn get_s_num(bytes: &[u8]) -> Option<String> {
    bytes
        .into_iter()
        .all(|byte| byte.is_ascii_digit())
        .then(|| {
            // SAFETY: only `is_ascii_digit()` bytes
            unsafe { String::from_utf8_unchecked(bytes.to_vec()) }
        })
}
