use crate::common::*;
use encoding_rs::Encoding;
use mux_media::*;

fn not_compatible(s: &str) -> CharEncoding {
    CharEncoding::NotUtf8Compatible(Encoding::for_label_no_replacement(s.as_bytes()).unwrap())
}

#[test]
fn detect_text_file() {
    let file_enc_pairs = [
        ("srt.srt", CharEncoding::Utf8Compatible),
        ("cp1251.srt", not_compatible("cp1251")),
        ("sub_x1.mks", CharEncoding::NotRecognized),
        ("txt.txt", CharEncoding::NotRecognized),
    ];
    let mut buf = [0u8; 1024 * 128];

    for (file, enc) in file_enc_pairs {
        let file = data(file);
        assert_eq!(enc, CharEncoding::detect_text_file(&file, &mut buf));
    }
}
