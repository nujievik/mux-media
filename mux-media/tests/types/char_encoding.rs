use super::media_info;
use crate::common::*;
use mux_media::*;

pub fn empty() -> CharEncoding {
    CharEncoding::MkvmergeRecognizable
}

pub fn new(s: &str) -> CharEncoding {
    CharEncoding::MkvmergeNotRecognizable(s.into())
}

macro_rules! test_file {
    ($fn:ident, $file:expr, $exp:expr, $exp_args:expr) => {
        #[test]
        fn $fn() {
            let file = data($file);
            let path = file.as_path();
            let sc: SubCharset = path.try_into().unwrap();
            let mut mi = media_info::new();

            assert_eq!($exp, CharEncoding::from(path));
            assert_eq!(SubCharset($exp), sc);
            assert_eq!($exp_args, sc.to_mkvmerge_args(&mut mi, path));
        }
    };
}

test_file!(test_matroska, "sub_x1.mks", empty(), Vec::<String>::new());
test_file!(test_utf8, "srt.srt", empty(), Vec::<String>::new());

test_file!(
    test_cp1251,
    "cp1251.srt",
    new("windows-1251"),
    to_args(["--sub-charset", "0:windows-1251"])
);

#[test]
fn test_not_supported() {
    assert!(SubCharset::try_from(data("audio_x1.mka").as_path()).is_err());
}
