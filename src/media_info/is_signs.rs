use super::*;
use crate::Stream;

impl MediaInfo<'_> {
    pub(crate) fn is_signs(&mut self, src: &Path, stream: &Stream) -> bool {
        if !stream.ty.is_sub() {
            return false;
        }

        parse(stream.title.as_ref().map(|v| &**v))
            || parse(self.get(MarkMediaInfoPathTail, src))
            || parse(self.get(MarkMediaInfoRelativeUpmost, src))
    }
}

macro_rules! matches_bytes {
    ($bytes:expr; $( $i:expr, $pattern:pat ),* $(,)?) => {{
        true $( && matches!($bytes[$i], $pattern) )*
    }};
}

fn parse(opt_s: Option<&String>) -> bool {
    let Some(s) = opt_s else { return false };
    let bytes = s.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        let b = bytes[i];

        // signs
        if matches!(b, b's' | b'S') {
            if i + 5 > len {
                return false;
            }

            if i + 5 == len || !is_alphabetic_byte(bytes[i + 5]) {
                if matches_bytes!(
                    bytes;
                    i + 1, b'i' | b'I',
                    i + 2, b'g' | b'G',
                    i + 3, b'n' | b'N',
                    i + 4, b's' | b'S'
                ) {
                    return true;
                }
            }
        }

        // russian надписи
        // н / Н
        if matches!(b, 0xD0) && i + 14 <= len {
            if matches!(bytes[i + 1], 0xBD | 0x9D)
                && (i + 14 == len || !is_alphabetic_byte(bytes[i + 14]))
            {
                if matches_bytes!(
                    &bytes[i..i+14];
                    // а / А
                    2, 0xD0, 3, 0xB0 | 0x90,
                    // д / Д
                    4, 0xD0, 5, 0xB4 | 0x94,
                    // п / П
                    6, 0xD0, 7, 0xBF | 0x9F,
                    // и / И
                    8, 0xD0, 9, 0xB8 | 0x98,
                    // и / И
                    12, 0xD0, 13, 0xB8 | 0x98
                ) && (
                    // с
                    (matches!(bytes[i + 10], 0xD1) && matches!(bytes[i + 11], 0x81))
                    ||
                    // С
                    (matches!(bytes[i + 10], 0xD0) && matches!(bytes[i + 11], 0xA1))
                ) {
                    return true;
                }
            }
        }

        i += 1;
        while i < len && is_alphabetic_byte(bytes[i]) {
            i += 1;
        }
        while i < len && !is_alphabetic_byte(bytes[i]) {
            i += 1;
        }
    }

    false
}

#[inline(always)]
fn is_alphabetic_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || matches!(b, 0xD0 | 0xD1)
}
