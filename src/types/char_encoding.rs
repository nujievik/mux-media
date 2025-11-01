use crate::{EXTENSIONS, Msg, MuxError, Result};
use std::{io::Read, path::Path};

/// A wrapper around [`CharEncoding`] with mkvmerge impl.
#[derive(Clone, Debug, PartialEq)]
pub struct SubCharset(pub CharEncoding);

/// A charaster encoding of file.
#[derive(Clone, Debug, PartialEq)]
pub enum CharEncoding {
    MkvmergeNotRecognizable(String),
    MkvmergeRecognizable,
    NotRecognized,
}

impl TryFrom<&Path> for SubCharset {
    type Error = MuxError;

    fn try_from(path: &Path) -> Result<Self> {
        path.extension()
            .map_or(false, |ext| {
                EXTENSIONS.subs.contains(ext.as_encoded_bytes())
            })
            .then(|| Self(path.into()))
            .ok_or_else(|| Msg::FileTypeNotSup.into())
    }
}

impl From<&Path> for CharEncoding {
    fn from(path: &Path) -> Self {
        if path.extension().map_or(false, |ext| {
            EXTENSIONS.matroska.contains(ext.as_encoded_bytes())
        }) {
            // All text in a Matroska(tm) file is encoded in UTF-8
            return Self::MkvmergeRecognizable;
        }

        return match detect_chardet(path) {
            Some(s) if is_mkvmerge_recognizable(&s) => Self::MkvmergeRecognizable,
            Some(s) => Self::MkvmergeNotRecognizable(s),
            None => Self::NotRecognized,
        };

        fn detect_chardet(path: &Path) -> Option<String> {
            const READ_LIMIT: usize = 128 * 1024; // 128 KiB
            const LIM_CONFIDENCE: f32 = 0.8;

            let mut file = std::fs::File::open(path).ok()?;
            let mut bytes = [0u8; READ_LIMIT];
            let bytes_read = file.read(&mut bytes).ok()?;

            match chardet::detect(&bytes[..bytes_read]) {
                det if det.1 >= LIM_CONFIDENCE => Some(det.0),
                _ => None,
            }
        }

        fn is_mkvmerge_recognizable(s: &str) -> bool {
            let s = s.trim().to_ascii_lowercase();
            s.starts_with("ascii") || s.starts_with("utf")
        }
    }
}
