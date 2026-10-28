use crate::common::*;
use mux_media::*;

#[test]
fn from_os_str_global() {
    ["g", "global"].iter().for_each(|s| {
        assert_eq!(Target::Global, Target::parse_os(&s).unwrap());
    })
}

#[test]
fn from_os_str_stream() {
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

#[test]
fn from_os_str_path() {
    ["srt.srt", "ogg.ogg", "video_x1.mkv"].iter().for_each(|f| {
        let f = data(f);
        let exp = Target::Path((&f).into());
        assert_eq!(exp, Target::parse_os(&f).unwrap());
    })
}
