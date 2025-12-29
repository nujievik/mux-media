use crate::common::*;
use mux_media::*;

#[test]
fn from_os_str_global() {
    ["g", "global"].iter().for_each(|s| {
        assert_eq!(Target::Global, Target::new(s).unwrap());
    })
}

#[test]
fn from_os_str_stream() {
    [
        (StreamType::Video, vec!["d", "v", "video"]),
        (StreamType::Audio, vec!["a", "audio"]),
        (StreamType::Sub, vec!["s", "sub", "subs"]),
        (StreamType::Font, vec!["f", "font", "fonts"]),
        (StreamType::Attach, vec!["m", "attach", "attachs"]),
        (StreamType::Other, vec!["other", "others"]),
    ]
    .into_iter()
    .for_each(|(ty, xs)| {
        for x in xs {
            assert_eq!(Target::Stream(ty), Target::new(x).unwrap());
        }
    })
}

#[test]
fn from_os_str_path() {
    ["srt.srt", "ogg.ogg", "video_x1.mkv"].iter().for_each(|f| {
        let f = data(f);
        let exp = Target::Path((&f).into());
        assert_eq!(exp, Target::new(f).unwrap());
    })
}
