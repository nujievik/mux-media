use crate::{
    EXTENSIONS, MISavedTracks, MediaInfo, MkvmergeArg, Msg, MuxError, ToMkvmergeArgs, TrackType,
    mkvmerge_arg, to_mkvmerge_args, unwrap_or_return_vec,
};
use std::io::{self, Read};
use std::path::Path;

const READ_LIMIT: usize = 32 * 1024; // 32 KiB
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

mkvmerge_arg!(SubCharset, "--sub-charset");

impl ToMkvmergeArgs for SubCharset {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
        let enc = match &self.0 {
            CharEncoding::MkvmergeNotRecognizable(s) if !s.is_empty() => s,
            _ => return Vec::new(),
        };
        unwrap_or_return_vec!(mi.get::<MISavedTracks>(path))[TrackType::Sub]
            .iter()
            .map(|num| [Self::MKVMERGE_ARG.to_string(), format!("{}:{}", num, enc)])
            .flatten()
            .collect()
    }

    to_mkvmerge_args!(@fn_os);
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
            Self::MkvmergeRecognizable
        } else {
            match [1, 2].into_iter().find_map(|factor| {
                let bytes = read_limited(path, factor * READ_LIMIT).ok()?;
                let detected = chardet::detect(&bytes);
                (detected.1 >= LIM_CONFIDENCE).then(|| detected.0)
            }) {
                Some(s) if Self::is_mkvmerge_recognizable(&s) => Self::MkvmergeRecognizable,
                Some(s) => Self::MkvmergeNotRecognizable(s),
                None => Self::NotRecognized,
            }
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

#[inline(always)]
fn read_limited(path: &Path, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}
