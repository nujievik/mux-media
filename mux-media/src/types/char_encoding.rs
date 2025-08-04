use crate::{
    EXTENSIONS, MediaInfo, Msg, MuxConfigArg, MuxError, ParseableArg, ToMkvmergeArgs, TrackType,
    markers::MISavedTracks,
};
use std::{ffi::OsString, io::Read, path::Path};

const READ_LIMIT: usize = 64 * 1024; // 64 KiB
const LIM_CONFIDENCE: f32 = 0.8;

/// A wrapper for [`CharEncoding`] with mkvmerge impls.
#[derive(Clone, Debug, PartialEq)]
pub struct SubCharset(pub CharEncoding);

/// A charaster encoding of file.
#[derive(Clone, Debug, PartialEq)]
pub enum CharEncoding {
    MkvmergeNotRecognizable(String),
    MkvmergeRecognizable,
    NotRecognized,
}

impl ToMkvmergeArgs for SubCharset {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
    ) -> Result<(), MuxError> {
        let enc = match &self.0 {
            CharEncoding::MkvmergeNotRecognizable(s) if !s.is_empty() => s,
            _ => return Ok(()),
        };

        mi.try_get::<MISavedTracks>(media)?[TrackType::Sub]
            .iter()
            .for_each(|track| {
                args.push(MuxConfigArg::SubCharset.dashed().into());
                args.push(format!("{}:{}", track, enc).into());
            });

        Ok(())
    }
}

impl TryFrom<&Path> for SubCharset {
    type Error = MuxError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
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

        let detect_chardet = || {
            let mut file = std::fs::File::open(path)?;
            let mut bytes = [0u8; READ_LIMIT];
            let bytes_read = file.read(&mut bytes)?;

            match chardet::detect(&bytes[..bytes_read]) {
                det if det.1 >= LIM_CONFIDENCE => Ok(det.0),
                _ => Err(MuxError::from("Not enough confidence")),
            }
        };

        match detect_chardet() {
            Ok(s) if Self::is_mkvmerge_recognizable(&s) => Self::MkvmergeRecognizable,
            Ok(s) => Self::MkvmergeNotRecognizable(s),
            Err(_) => Self::NotRecognized,
        }
    }
}

impl CharEncoding {
    #[inline(always)]
    fn is_mkvmerge_recognizable(s: &str) -> bool {
        let s = s.trim().to_ascii_lowercase();
        s.starts_with("ascii") || s.starts_with("utf")
    }
}
