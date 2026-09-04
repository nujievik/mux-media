mod common;

#[path = "media_info/durations.rs"]
mod durations;

use common::*;
use mux_media::{media_info::*, *};
use std::sync::LazyLock;

static CONFIG: LazyLock<Config> = LazyLock::new(|| cfg::<_, &str>([]));

pub fn new() -> MediaInfo<'static> {
    MediaInfo::new(&*CONFIG, 0)
}

#[test]
fn test_empty() {
    let mi = new();
    assert_eq!(0, mi.cache.of_files.len());
}

#[test]
fn test_set_cmn_stem() {
    ["x", "a", "bc"].iter().for_each(|s| {
        let mut mi = new();
        mi.set_cmn(MarkMediaInfoStem, s.into());
        assert_eq!(s, mi.try_get_cmn(MarkMediaInfoStem).unwrap());
    })
}

#[test]
fn test_clear() {
    let mut mi = new();
    mi.set_cmn(MarkMediaInfoStem, "x".into());
    mi.try_insert(data("srt.srt")).unwrap();
    mi.clear();

    assert_eq!(0, mi.cache.of_files.len());
}

#[test]
fn test_try_insert() {
    let mut mi = new();
    let mut len = 0;

    ["srt.srt", "audio_x1.mka", "video_x8.mkv"]
        .iter()
        .for_each(|f| {
            len += 1;
            mi.try_insert(data(f)).unwrap();
            assert_eq!(len, mi.cache.of_files.len());
        });

    mi.clear();

    ["missing", "bad"].iter().for_each(|f| {
        mi.try_insert(data(f)).unwrap_err();
    });

    assert_eq!(0, mi.cache.of_files.len());
}

#[test]
fn test_try_insert_many() {
    let mut mi = new();
    let paths = [data("srt.srt"), data("audio_x1.mka")];

    mi.try_insert_many(paths).unwrap();
    assert_eq!(2, mi.cache.of_files.len());

    mi.clear();
    let bad_paths = [data("missing"), data("bad")];
    mi.try_insert_many(bad_paths).unwrap();
    assert_eq!(0, mi.cache.of_files.len());
}

#[test]
fn test_cmn_stem() {
    let mut mi = new();

    [("srt", "srt.srt"), ("audio_x1", "audio_x1.mka")]
        .into_iter()
        .for_each(|(stem, f)| {
            mi.try_insert(data(f)).unwrap();
            assert_eq!(stem, mi.try_get_cmn(MarkMediaInfoStem).unwrap());
            mi.clear();
        });

    [
        "x1_set/x1_set.mkv",
        "x1_set/audio/x1_set.[audio].mka",
        "x1_set/subs/x1_set.[subs].mks",
    ]
    .iter()
    .for_each(|f| mi.try_insert(data(f)).unwrap());

    assert_eq!("x1_set", mi.try_get_cmn(MarkMediaInfoStem).unwrap());
}

#[test]
fn test_cmn_streams_order() {
    let mut mi = new();
    mi.try_insert(data("audio_x1.mka")).unwrap();
    mi.try_init_cmn(MarkMediaInfoStreamsOrder).unwrap();
}

#[test]
fn test_sub_charset() {
    let mut mi = new();
    let empty = || CharEncoding::Utf8Compatible;
    let new = |s: &str| CharEncoding::NotUtf8Compatible(s.into());

    [
        ("srt.srt", empty()),
        ("sub_x1.mks", empty()),
        ("cp1251.srt", new("windows-1251")),
    ]
    .iter()
    .for_each(|(f, enc)| {
        assert_eq!(
            enc,
            mi.try_get(MarkMediaInfoSubCharEncoding, &data(f)).unwrap()
        );
    });

    mi.try_get(MarkMediaInfoSubCharEncoding, &data("audio_x1.mka"))
        .unwrap_err();
}

#[test]
fn test_streams() {
    let mut mi = new();

    [
        "audio_x1.mka",
        "font_x8_other_x8.mks",
        "sub_x1.mks",
        "video_x1.mkv",
        "ogg.ogg",
        "srt.srt",
    ]
    .iter()
    .for_each(|f| {
        mi.try_get(MarkMediaInfoStreams, data(f)).unwrap();
    })
}

static TEST_TARGETS: &[&[&str]] = &[
    &["audio_x1.mka", "audio_x8.mka", "ogg.ogg"],
    &["srt.srt", "sub_x1.mks", "sub_x8.mks", "cp1251.srt"],
    &["video_x1.mkv", "video_x8.mkv"],
];

#[test]
fn test_targets_empty_user_input() {
    let mut mi = new();
    let empty = Vec::<Target>::new();

    TEST_TARGETS.iter().flat_map(|fs| fs.iter()).for_each(|f| {
        assert_eq!(
            &empty,
            mi.try_get(MarkMediaInfoTargetPaths, &data(f)).unwrap()
        );
    });
}

fn build_targets(slice: &[Target]) -> Vec<Target> {
    slice.into_iter().map(|trg| trg.clone()).collect()
}

#[test]
fn test_targets_path_only() {
    TEST_TARGETS.iter().for_each(|files| {
        files.iter().for_each(|f| {
            let f = data(f);
            let cfg = cfg([p("--target"), &f, p("-C")]);
            let mut mi = MediaInfo::new(&cfg, 0);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&f))]);

            assert_eq!(&left, mi.try_get(MarkMediaInfoTargetPaths, &f).unwrap());
        })
    })
}

#[test]
fn test_targets_parent_only() {
    let parent = data(""); //common for all files

    TEST_TARGETS.iter().for_each(|files| {
        files.iter().for_each(|f| {
            let f = data(f);
            let cfg = cfg([p("--target"), &parent, p("-C")]);
            let mut mi = MediaInfo::new(&cfg, 0);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&parent))]);

            assert_eq!(&left, mi.try_get(MarkMediaInfoTargetPaths, &f).unwrap());
        })
    })
}

#[test]
fn test_targets_all() {
    let parent = data(""); //common for all files

    TEST_TARGETS.iter().for_each(|files| {
        files.iter().for_each(|f| {
            let f = data(f);
            let args = [p("--target"), &f, p("-C"), p("--target"), &parent, p("-C")];

            let cfg = cfg(args);
            let mut mi = MediaInfo::new(&cfg, 0);

            let left = build_targets(&[
                Target::Path(ArcPathBuf::from(&f)),
                Target::Path(ArcPathBuf::from(&parent)),
            ]);

            assert_eq!(&left, mi.try_get(MarkMediaInfoTargetPaths, &f).unwrap());
        })
    })
}

#[test]
fn test_path_tail() {
    let mut mi = new();

    mi.set_cmn(MarkMediaInfoStem, "".into());

    [
        ("audio_x1", "audio_x1.mka"),
        ("sub_x1", "sub_x1.mks"),
        ("srt", "srt.srt"),
    ]
    .iter()
    .for_each(|(exp, f)| {
        assert_eq!(
            &exp.to_string(),
            mi.get(MarkMediaInfoPathTail, &data(f)).unwrap()
        );
    });

    mi.clear();
    mi.set_cmn(MarkMediaInfoStem, "s".into());

    [("ub_x1", "sub_x1.mks"), ("rt", "srt.srt")]
        .iter()
        .for_each(|(exp, f)| {
            assert_eq!(
                &exp.to_string(),
                mi.get(MarkMediaInfoPathTail, &data(f)).unwrap()
            );
        })
}

#[test]
fn test_relative_upmost() {
    // Upmost dir = data("")
    let mut cfg = cfg([p("-i"), &data("")]);
    cfg.try_finalize_init().unwrap();
    let mut mi = MediaInfo::new(&cfg, 0);

    [
        (String::new(), "audio_x1.mka"),
        (String::new(), "sub_x1.mks"),
        (String::new(), "srt.srt"),
        (s_sep("/x1_set"), "x1_set/x1_set.mkv"),
        (s_sep("/x1_set/subs"), "x1_set/subs/x1_set.[subs].mks"),
    ]
    .iter()
    .for_each(|(exp, f)| {
        let f = data(f);
        assert_eq!(exp, mi.try_get(MarkMediaInfoRelativeUpmost, &f).unwrap());
    })
}

#[test]
fn test_stream_title() {
    let mut mi = new();

    [
        ("a", "title/a_title.mks", ""),
        ("bc", "title/bc_title.mks", ""),
        ("abc", "title/begin.abc.mks", "begin"),
        ("tail", "title/begin.tail.mks", "begin"),
        ("abc", "title/begin  .abc ..mks", "begin"),
        ("abc", "title/other_begin.abc.mks", "other_begin"),
        ("a", "title/from_parent/a/begin.mks", "begin"),
        ("bc", "title/from_parent/bc/begin.mks", "begin"),
    ]
    .into_iter()
    .for_each(|(title, f, cmn_stem)| {
        let f = data(f);
        mi.clear();
        mi.try_insert(&f).unwrap();
        mi.set_cmn(MarkMediaInfoStem, cmn_stem.into());
        mi.try_finalize_init_streams().unwrap();

        let exp = String::from(title);
        assert_eq!(
            Some(&exp),
            mi.try_get(MarkMediaInfoStreams, &f).unwrap()[0]
                .title
                .as_deref()
        );
    })
}

#[test]
fn test_stream_lang() {
    let mut mi = new();

    [
        (lang!(Und), "audio_x1.mka", ""),
        (lang!(Und), "sub_x1.mks", "sub_x1"),
        (lang!(Eng), "lang/en_lang.mks", "en_lang"),
        (lang!(Rus), "lang/ru_lang.mks", "ru_lang"),
        (lang!(Eng), "lang/begin.en.srt", "begin"),
        (lang!(Rus), "lang/begin.ru.srt", "begin"),
    ]
    .into_iter()
    .for_each(|(lang, f, cmn_stem)| {
        let f = data(f);
        mi.clear();
        mi.try_insert(&f).unwrap();
        mi.set_cmn(MarkMediaInfoStem, cmn_stem.into());
        mi.try_finalize_init_streams().unwrap();

        assert_eq!(lang, *mi.try_get(MarkMediaInfoStreams, &f).unwrap()[0].lang);
    });

    let cfg = cfg([p("-i"), &data("")]);
    let mut mi = MediaInfo::new(&cfg, 0);

    [
        (lang!(Und), "srt.srt"),
        (lang!(Eng), "lang/eng subs/srt.srt"),
        (lang!(Rus), "lang/rus subs/srt.srt"),
    ]
    .into_iter()
    .for_each(|(lang, f)| {
        let f = data(f);
        mi.clear();
        mi.try_insert(&f).unwrap();
        mi.try_finalize_init_streams().unwrap();

        assert_eq!(lang, *mi.try_get(MarkMediaInfoStreams, &f).unwrap()[0].lang);
    });
}
