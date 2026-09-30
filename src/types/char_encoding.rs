use crate::Extension;
use std::{fs::File, io::Read, path::Path};

/// A charaster encoding of file.
#[derive(Clone, Debug, PartialEq)]
pub enum CharEncoding {
    Utf8Compatible,
    NotUtf8Compatible(String),
    NotRecognized,
}

impl CharEncoding {
    /// Detect charset reading file bytes into given buffer.
    pub fn detect<P>(file: &P, buf: &mut [u8]) -> CharEncoding
    where
        P: AsRef<Path> + ?Sized,
    {
        let f = file.as_ref();

        if Extension::new_from_path(f).is_some_and(|ext| ext.is_matroska()) {
            // All text in a Matroska(tm) file is encoded in UTF-8
            return Self::Utf8Compatible;
        }

        match detect_file_charenc(f, buf) {
            Some(s) if is_utf8_compatible(&s) => Self::Utf8Compatible,
            Some(s) => Self::NotUtf8Compatible(s),
            None => Self::NotRecognized,
        }
    }

    #[deprecated]
    pub fn new(file: impl AsRef<Path>) -> CharEncoding {
        let f = file.as_ref();

        if f.extension().map_or(false, |ext| {
            Extension::new_and_is_matroska(ext.as_encoded_bytes())
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
            let mut bytes = [0u8; READ_LIMIT];
            detect_file_charenc(f, &mut bytes)
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

fn is_utf8_compatible(s: &str) -> bool {
    let s = s.trim();
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
