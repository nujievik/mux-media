use crate::{Msg, Result, StreamType};
use std::{ffi::OsStr, fs, path::Path};

/// A target of [`ConfigTarget`](crate::ConfigTarget).
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Global,
    StreamType(StreamType),
    Path(Box<Path>),
}

impl Target {
    /// Tries parse [`Target`] from an os string.
    pub fn parse_os<OS: AsRef<OsStr>>(os: &OS) -> Result<Target>
    where
        OS: AsRef<OsStr> + ?Sized,
    {
        let os = os.as_ref();

        if let Some(t) = os.to_str().and_then(|s| get_from_str(s)) {
            return Ok(t);
        }

        let path = fs::canonicalize(os)?;

        return Ok(Self::Path(path.into()));

        fn get_from_str(s: &str) -> Option<Target> {
            let s = s.trim().to_ascii_lowercase();
            if matches!(s.as_str(), "g" | "global") {
                Some(Target::Global)
            } else if let Ok(ty) = s.parse::<StreamType>() {
                Some(Target::StreamType(ty))
            } else {
                None
            }
        }
    }

    /// Prints the list of supported targets to stdout.
    pub(crate) fn print_list_targets() {
        println!("{}", Msg::ListTargets.as_str_localized());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_os_global() {
        ["g", "global"].iter().for_each(|s| {
            assert_eq!(Target::Global, Target::parse_os(&s).unwrap());
        })
    }

    #[test]
    fn parse_os_stream() {
        [
            (StreamType::Video, vec!["d", "v", "video"]),
            (StreamType::Audio, vec!["a", "audio"]),
            (
                StreamType::Subtitles,
                vec!["s", "sub", "subs", "subtitle", "subtitles"],
            ),
            (StreamType::AttachedFont, vec!["f", "font", "fonts"]),
            (StreamType::AttachedPic, vec!["p", "pic", "pics"]),
            (StreamType::Other, vec!["other", "others"]),
        ]
        .into_iter()
        .for_each(|(ty, xs)| {
            for x in xs {
                assert_eq!(Target::StreamType(ty), Target::parse_os(&x).unwrap());
            }
        })
    }

    fn data(add: &str) -> PathBuf {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("data");
        for s in add.split('/') {
            p.push(s);
        }
        crate::ensure_long_path_prefix(p)
    }

    #[test]
    fn parse_os_path() {
        ["srt.srt", "ogg.ogg", "video_x1.mkv"].iter().for_each(|f| {
            let f = data(f);
            let exp = Target::Path(f.clone().into());
            assert_eq!(exp, Target::parse_os(&f).unwrap());
        })
    }
}
