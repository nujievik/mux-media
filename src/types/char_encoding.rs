use crate::EXTENSIONS;
use std::{fs::File, io::Read, path::Path};

/// A charaster encoding of file.
#[derive(Clone, Debug, PartialEq)]
pub enum CharEncoding {
    Utf8Compatible,
    NotUtf8Compatible(String),
    NotRecognized,
}

impl CharEncoding {
    pub fn new(file: impl AsRef<Path>) -> CharEncoding {
        let f = file.as_ref();

        if f.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            // All text in a Matroska(tm) file is encoded in UTF-8
            return Self::Utf8Compatible;
        }

        return match detect_chardet(f) {
            Some(s) if is_utf8_compatible(&s) => Self::Utf8Compatible,
            Some(s) => Self::NotUtf8Compatible(s),
            None => Self::NotRecognized,
        };

        fn detect_chardet(f: &Path) -> Option<String> {
            const READ_LIMIT: usize = 128 * 1024; // 128 KiB
            const LIM_CONFIDENCE: f32 = 0.8;

            let mut file = File::open(f).ok()?;
            let mut bytes = [0u8; READ_LIMIT];
            let bytes_read = file.read(&mut bytes).ok()?;

            match chardet::detect(&bytes[..bytes_read]) {
                det if det.1 >= LIM_CONFIDENCE => Some(det.0),
                _ => None,
            }
        }

        fn is_utf8_compatible(s: &str) -> bool {
            let s = s.trim();
            s.eq_ignore_ascii_case("ascii") || s.eq_ignore_ascii_case("utf-8")
        }
    }

    pub(crate) fn get_ffmpeg_sub_charenc(&self) -> Option<&str> {
        match self {
            Self::Utf8Compatible => None,
            Self::NotUtf8Compatible(s) => Some(&s),
            Self::NotRecognized => None,
        }
    }
}
