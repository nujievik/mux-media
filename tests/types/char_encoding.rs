use crate::{common::*, *};
use mux_media::*;
use std::{path::PathBuf, sync::LazyLock};

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
