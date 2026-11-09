use encoding_rs::Encoding;
use std::{fs::File, io::Read, path::Path};

/// A charaster encoding of file.
#[derive(Clone, Debug, PartialEq)]
pub enum CharEncoding {
    Utf8Compatible,
    NotUtf8Compatible(&'static Encoding),
    NotRecognized,
}

impl CharEncoding {
    pub fn detect_text_file<P>(file: &P, buf: &mut [u8]) -> CharEncoding
    where
        P: AsRef<Path> + ?Sized,
    {
        detect_file_charenc(file.as_ref(), buf).map_or(Self::NotRecognized, |s| {
            if is_utf8_compatible(&s) {
                return Self::Utf8Compatible;
            }

            match Encoding::for_label_no_replacement(s.as_bytes()) {
                Some(enc) => Self::NotUtf8Compatible(enc),
                None => Self::NotRecognized,
            }
        })
    }
}

fn is_utf8_compatible(s: &str) -> bool {
    s.eq_ignore_ascii_case("ascii") || s.eq_ignore_ascii_case("utf-8")
}

fn detect_file_charenc(file: &Path, buf: &mut [u8]) -> Option<String> {
    const LIM_CONFIDENCE: f32 = 0.8;

    let mut file = File::open(file).ok()?;
    let bytes_read = file.read(buf).ok()?;

    match chardet::detect(&buf[..bytes_read]) {
        det if det.1 >= LIM_CONFIDENCE => Some(det.0),
        _ => None,
    }
}
