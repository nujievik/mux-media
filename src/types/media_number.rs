use std::ffi::OsStr;

#[derive(Clone)]
pub struct MediaNumber {
    raw: String,
    len_bytes: usize,
    idx: Option<usize>,
    digits: Vec<String>,
    borders: Vec<(usize, usize)>,
}

impl From<&OsStr> for MediaNumber {
    fn from(value: &OsStr) -> Self {
        let bytes = get_bytes(value);
        let len_bytes = bytes.len();

        let (digits, borders) = get_ascii_digits(bytes);

        let raw = digits.get(0).cloned().unwrap_or(String::new());

        Self {
            raw,
            len_bytes,
            idx: None,
            digits,
            borders,
        }
    }
}

impl MediaNumber {
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub fn to_u64(&self) -> u64 {
        self.raw.parse::<u64>().unwrap_or(0)
    }

    pub fn upd(&mut self, value: &OsStr) {
        let bytes = get_bytes(value);
        let len_bytes = bytes.len();

        if self.len_bytes == len_bytes {
            match self.idx {
                Some(idx) => {
                    if let Some(&(start, end)) = self.borders.get(idx) {
                        let slice = &bytes[start..end];
                        if let Some(raw) = get_ascii_digit_from_slice(slice) {
                            self.raw = raw;
                            return;
                        }
                    }
                }
                None => {}
            }

            // fallback
            let (digits, borders) = get_ascii_digits(bytes);
            let (raw, idx) = self.get_raw_from_unmatched(&digits);

            self.raw = raw;
            self.idx = idx;
            self.digits = digits;
            self.borders = borders;
        } else {
            let (digits, borders) = get_ascii_digits(bytes);

            self.raw = digits.get(0).cloned().unwrap_or(String::new());
            self.len_bytes = len_bytes;
            self.idx = None;
            self.digits = digits;
            self.borders = borders;
        }
    }

    fn get_raw_from_unmatched(&self, digits: &[String]) -> (String, Option<usize>) {
        for (i, digit) in digits.iter().enumerate() {
            if self.digits.get(i) != Some(digit) {
                return (digit.clone(), Some(i));
            }
        }
        ("".to_string(), None)
    }
}

#[cfg(unix)]
fn get_bytes(os_str: &OsStr) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    os_str.as_bytes()
}

#[cfg(windows)]
fn get_bytes(os_str: &OsStr) -> Vec<u16> {
    use std::os::unix::ffi::OsStrExt;
    os_str.encode_wide().collect()
}

#[cfg(unix)]
fn get_ascii_digit_from_slice(slice: &[u8]) -> Option<String> {
    if slice.iter().all(|&c| c.is_ascii_digit()) {
        Some(unsafe { String::from_utf8_unchecked(slice.to_vec()) })
    } else {
        None
    }
}

#[cfg(windows)]
fn get_ascii_digit_from_slice(slice: &[u16]) -> Option<String> {
    if slice.iter().all(|&c| (0x30..=0x39).contains(&c)) {
        let s: String = slice
            .iter()
            .map(|&c| char::from_u32(c as u32).unwrap())
            .collect();
        Some(s)
    } else {
        None
    }
}

#[cfg(unix)]
fn get_ascii_digits(bytes: &[u8]) -> (Vec<String>, Vec<(usize, usize)>) {
    let mut digits = Vec::new();
    let mut borders = Vec::new();

    let mut current = Vec::new();
    let mut start = 0;
    let mut count = 0;

    for &b in bytes {
        if b.is_ascii_digit() {
            current.push(b);
            if start == 0 {
                start = count;
            }
        } else if !current.is_empty() {
            digits.push(unsafe { String::from_utf8_unchecked(current.clone()) });
            current.clear();
            borders.push((start, count + 1));
            start = 0;
        }
        count += 1;
    }

    if !current.is_empty() {
        digits.push(unsafe { String::from_utf8_unchecked(current) });
        borders.push((start, count));
    }

    (digits, borders)
}

#[cfg(windows)]
fn get_ascii_digits(utf16: Vec<u16>) -> (Vec<String>, Vec<(usize, usize)>) {
    let mut digits = Vec::new();
    let mut borders = Vec::new();

    let mut current = Vec::new();
    let mut start = 0;
    let mut count = 0;

    for &u in utf16 {
        if (u as u8).is_ascii_digit() && u <= 0x7F {
            current.push(u as u8);
            if start == 0 {
                start = count;
            }
        } else if !current.is_empty() {
            digits.push(unsafe { String::from_utf8_unchecked(current.clone()) });
            current.clear();
            borders.push((start, count + 1));
        }
        count += 1;
    }

    if !current.is_empty() {
        digits.push(unsafe { String::from_utf8_unchecked(current) });
        borders.push((start, count));
    }

    (digits, borders)
}
