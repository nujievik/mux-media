use crate::{common::*, *};
use mux_media::*;
use std::{ffi::OsString, path::PathBuf, sync::LazyLock};

pub fn empty() -> CharEncoding {
    CharEncoding::Utf8Compatible
}

pub fn new(s: &str) -> CharEncoding {
    CharEncoding::NotUtf8Compatible(s.into())
}

static FILE_ENC_PAIRS: LazyLock<[(PathBuf, CharEncoding); 3]> = LazyLock::new(|| {
    [
        (data("sub_x1.mks"), empty()),
        (data("srt.srt"), empty()),
        (data("cp1251.srt"), new("windows-1251")),
    ]
});

#[test]
fn test_new() {
    FILE_ENC_PAIRS.iter().for_each(|(f, enc)| {
        assert_eq!(enc, &CharEncoding::new(f));
    })
}

#[test]
fn to_ffmpeg_args() {
    FILE_ENC_PAIRS.iter().for_each(|(f, enc)| {
        let args = if let CharEncoding::NotUtf8Compatible(s) = enc {
            let ext = f.extension().and_then(|ext| ext.to_str()).unwrap();
            vec![
                p("-sub_charenc"),
                p(s),
                p("-i"),
                f,
                p("-map"),
                p("0:0"),
                p("-c:0"),
                p(ext),
            ]
        } else {
            vec![p("-i"), f, p("-map"), p("0:0"), p("-c:0"), p("copy")]
        };

        let mut mi = media_info::new();
        mi.try_insert(f).unwrap();
        assert_eq!(
            to_os_args(args),
            StreamsOrder::to_ffmpeg_args(&mut mi).unwrap()
        );
    })
}
