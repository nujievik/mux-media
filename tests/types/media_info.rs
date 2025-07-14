use super::char_encoding;
use crate::common::*;
use mux_media::*;
use once_cell::sync::Lazy;
use smallvec::SmallVec;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};

static MUX_CONFIG: Lazy<MuxConfig> = Lazy::new(|| cfg::<_, &str>([]));

pub fn new() -> MediaInfo<'static> {
    MediaInfo::from(&*MUX_CONFIG)
}

fn new_path(s: &str) -> ArcPathBuf {
    ArcPathBuf::from(data_file(s))
}

fn new_stem(s: &str) -> Arc<OsString> {
    Arc::new(OsString::from(s))
}

#[test]
fn test_empty() {
    let mi = new();
    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_upd_cmn_stem() {
    ["x", "a", "bc"].iter().for_each(|s| {
        let mut mi = new();
        let stem = new_stem(s);
        mi.upd_cmn_stem(stem.clone());
        assert_eq!(0, mi.len());
        assert!(!mi.is_empty());
        assert!(mi.is_no_files());
        assert_eq!(&stem, mi.try_get_cmn::<MICmnStem>().unwrap());
    })
}

#[test]
fn test_try_insert() {
    let mut mi = new();
    let mut len = 0;

    ["srt.srt", "audio_x1.mka", "video_x8.mkv"]
        .iter()
        .for_each(|f| {
            len += 1;
            mi.try_insert_path(new_path(f)).unwrap();
            assert_eq!(len, mi.len());
            assert!(!mi.is_empty());
            assert!(!mi.is_no_files());
        });

    mi.clear();

    ["missing", "bad"].iter().for_each(|f| {
        assert!(mi.try_insert_path(new_path(f)).is_err());
    });

    assert_eq!(0, mi.len());
}

#[test]
fn test_clear() {
    let mut mi = new();
    mi.upd_cmn_stem(new_stem("x"));
    mi.try_insert_path(new_path("srt.srt")).unwrap();
    mi.clear();

    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_get_take_upd_cache() {
    let mut mi = new();
    assert!(mi.get_cache().of_files.is_empty());

    ["srt.srt", "audio_x1.mka"].iter().for_each(|f| {
        let file = new_path(f);
        mi.try_insert_path(file.clone()).unwrap();

        let cache = mi.get_cache();
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        mi.unmut_get::<MIMkvmergeI>(&file).unwrap();

        let cache = mi.take_cache();
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        assert!(mi.unmut_get::<MIMkvmergeI>(&file).is_none());

        mi.upd_cache(cache);
        assert_eq!(&file, mi.get_cache().of_files.keys().next().unwrap());
        mi.unmut_get::<MIMkvmergeI>(&file).unwrap();

        mi.clear()
    })
}

#[test]
fn test_try_insert_with_filter() {
    let paths = [new_path("srt.srt"), new_path("audio_x1.mka")];

    for (arg, len) in [("-D", 2), ("-SA", 0)] {
        let mc = cfg([arg]);
        let mut mi = MediaInfo::from(&mc);
        mi.try_insert_paths_with_filter(paths.clone(), true)
            .unwrap();
        assert_eq!(len, mi.len());
    }

    let mut mi = new();
    let bad_paths = [new_path("missing"), new_path("bad")];
    for exit_on_err in [true, false] {
        assert_eq!(
            exit_on_err,
            mi.try_insert_paths_with_filter(bad_paths.clone(), exit_on_err)
                .is_err()
        );
    }
    assert_eq!(0, mi.len());
}

#[test]
fn test_cmn_stem() {
    let mut mi = new();

    [("srt", "srt.srt"), ("audio_x1", "audio_x1.mka")]
        .into_iter()
        .for_each(|(stem, file)| {
            mi.try_insert_path(new_path(file)).unwrap();
            assert_eq!(&new_stem(stem), mi.try_get_cmn::<MICmnStem>().unwrap());
            mi.clear();
        });

    [
        s_sep("x1_set/x1_set.mkv"),
        s_sep("x1_set/audio/x1_set.[audio].mka"),
        s_sep("x1_set/subs/x1_set.[subs].mks"),
    ]
    .iter()
    .for_each(|f| mi.try_insert_path(new_path(f)).unwrap());

    assert_eq!(&new_stem("x1_set"), mi.try_get_cmn::<MICmnStem>().unwrap());
}

#[test]
fn test_sub_charset() {
    let mut mi = new();
    let empty = || char_encoding::empty();
    let new = |s: &str| char_encoding::new(s);

    [
        ("srt.srt", empty()),
        ("sub_x1.mks", empty()),
        ("cp1251.srt", new("windows-1251")),
    ]
    .into_iter()
    .for_each(|(file, enc)| {
        let file = data_file(file);
        assert_eq!(&SubCharset(enc), mi.try_get::<MISubCharset>(&file).unwrap());
    });

    assert!(
        mi.try_get::<MISubCharset>(data_file("audio_x1.mka").as_path())
            .is_err()
    );
}

#[test]
fn test_target_group() {
    let mut mi = new();

    [
        (
            TargetGroup::Audio,
            vec!["audio_x1.mka", "audio_x8.mka", "ogg.ogg"],
        ),
        (
            TargetGroup::Subs,
            vec!["srt.srt", "sub_x1.mks", "sub_x8.mks", "cp1251.srt"],
        ),
        (
            TargetGroup::Video,
            vec!["video_x1.mkv", "video_x8.mkv", "x1_set/x1_set.mkv"],
        ),
    ]
    .iter()
    .for_each(|(exp, files)| {
        files.iter().for_each(|f| {
            let f = data_file(f);
            assert_eq!(exp, mi.try_get::<MITargetGroup>(&f).unwrap());
        })
    })
}

#[test]
fn test_mkvinfo() {
    let mut mi = new();

    [
        "audio_x1.mka",
        "audio_x8.mka",
        "font_attachs_x16.mks",
        "font_x8_other_x8.mks",
        "sub_x1.mks",
        "video_x1.mkv",
    ]
    .iter()
    .for_each(|f| {
        let f = data_file(f);
        mi.try_get::<MIMkvinfo>(&f).unwrap();
    })
}

#[test]
fn test_mkvmerge_i() {
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
        mi.try_get::<MIMkvmergeI>(data_file(f).as_path()).unwrap();
    })
}

static TEST_TARGETS: &[(TargetGroup, &[&str])] = &[
    (
        TargetGroup::Audio,
        &["audio_x1.mka", "audio_x8.mka", "ogg.ogg"],
    ),
    (
        TargetGroup::Subs,
        &["srt.srt", "sub_x1.mks", "sub_x8.mks", "cp1251.srt"],
    ),
    (TargetGroup::Video, &["video_x1.mkv", "video_x8.mkv"]),
];

#[test]
fn test_targets_empty_user_input() {
    let mut mi = new();
    let empty = SmallVec::<[Target; 3]>::new();

    TEST_TARGETS
        .iter()
        .map(|(_, files)| files.iter())
        .flatten()
        .for_each(|f| {
            let f = data_file(f);
            assert_eq!(&empty, mi.try_get::<MITargets>(&f).unwrap());
        });
}

fn build_targets(slice: &[Target]) -> SmallVec<[Target; 3]> {
    slice.into_iter().map(|trg| trg.clone()).collect()
}

#[test]
fn test_targets_group_only() {
    TEST_TARGETS.iter().for_each(|(group, files)| {
        let g_args = vec!["--target", group.as_ref(), "-B"];
        files.iter().for_each(|f| {
            let f = data_file(f);
            let mc = cfg(&g_args);
            let mut mi = MediaInfo::from(&mc);

            let left = build_targets(&[Target::Group(*group)]);
            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_targets_path_only() {
    TEST_TARGETS.iter().for_each(|(_, files)| {
        files.iter().for_each(|f| {
            let f = data_file(f);
            let mc = cfg([Path::new("--target"), &f, Path::new("-B")]);
            let mut mi = MediaInfo::from(&mc);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&f))]);

            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_targets_parent_only() {
    let parent = data_dir(); //common for all files

    TEST_TARGETS.iter().for_each(|(_, files)| {
        files.iter().for_each(|f| {
            let f = data_file(f);
            let mc = cfg([Path::new("--target"), &parent, Path::new("-B")]);
            let mut mi = MediaInfo::from(&mc);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&parent))]);

            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_targets_all() {
    let parent = data_dir(); //common for all files

    TEST_TARGETS.iter().for_each(|(group, files)| {
        files.iter().for_each(|f| {
            let f = data_file(f);
            let args = [
                Path::new("--target"),
                group.as_path(),
                Path::new("-B"),
                Path::new("--target"),
                &f,
                Path::new("-B"),
                Path::new("--target"),
                &parent,
                Path::new("-B"),
            ];

            let mc = cfg(args);
            let mut mi = MediaInfo::from(&mc);

            let left = build_targets(&[
                Target::Path(ArcPathBuf::from(&f)),
                Target::Path(ArcPathBuf::from(&parent)),
                Target::Group(*group),
            ]);

            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_tracks_info() {
    let mut mi = new();

    [
        ("audio_x1.mka", 1),
        ("audio_x8.mka", 8),
        ("sub_x8.mks", 8),
        ("video_x1.mkv", 1),
        ("srt.srt", 1),
    ]
    .into_iter()
    .for_each(|(f, len)| {
        assert_eq!(
            len,
            mi.get::<MITracksInfo>(data_file(f).as_path())
                .unwrap()
                .len()
        );
    })
}

#[test]
fn test_attachs_info() {
    let mut mi = new();

    [
        ("audio_x1.mka", 0),
        ("font_attachs_x16.mks", 16),
        ("font_x8_other_x8.mks", 16),
        ("other_attachs_x16.mks", 16),
        ("other_x8_font_x8.mks", 16),
        ("srt.srt", 0),
    ]
    .into_iter()
    .for_each(|(f, len)| {
        assert_eq!(
            len,
            mi.get::<MIAttachsInfo>(data_file(f).as_path())
                .unwrap()
                .len()
        );
    })
}

#[test]
fn test_path_tail() {
    let mut mi = new();

    mi.upd_cmn_stem(new_stem(""));

    [
        ("audio_x1", "audio_x1.mka"),
        ("sub_x1", "sub_x1.mks"),
        ("srt", "srt.srt"),
    ]
    .iter()
    .for_each(|(exp, f)| {
        let f = data_file(f);
        assert_eq!(&exp.to_string(), mi.get::<MIPathTail>(&f).unwrap());
    });

    mi.clear();
    mi.upd_cmn_stem(new_stem("s"));

    [("ub_x1", "sub_x1.mks"), ("rt", "srt.srt")]
        .iter()
        .for_each(|(exp, f)| {
            let f = data_file(f);
            assert_eq!(&exp.to_string(), mi.get::<MIPathTail>(&f).unwrap());
        })
}

#[test]
fn test_relative_upmost() {
    // Upmost dir = data_dir()
    let mut mc = cfg::<_, PathBuf>([PathBuf::from("-i"), data_dir()]);
    mc.try_finalize_init().unwrap();
    let mut mi = MediaInfo::from(&mc);

    [
        (String::new(), s_sep("audio_x1.mka")),
        (String::new(), s_sep("sub_x1.mks")),
        (String::new(), s_sep("srt.srt")),
        (s_sep("/x1_set"), s_sep("x1_set/x1_set.mkv")),
        (
            s_sep("/x1_set/subs"),
            s_sep("x1_set/subs/x1_set.[subs].mks"),
        ),
    ]
    .iter()
    .for_each(|(exp, f)| {
        let f = data_file(f);
        assert_eq!(exp, mi.get::<MIRelativeUpmost>(&f).unwrap());
    })
}

#[test]
fn test_saved_tracks() {
    [
        (1, 0, 0, 0, "audio_x1.mka", vec![]),
        (8, 0, 0, 0, "audio_x8.mka", vec![]),
        (0, 8, 0, 0, "sub_x8.mks", vec![]),
        (0, 0, 8, 0, "video_x8.mkv", vec![]),
        (0, 0, 0, 0, "audio_x8.mka", vec!["-ASD"]),
        (0, 0, 0, 0, "sub_x8.mks", vec!["-ASD"]),
        (0, 0, 0, 0, "video_x8.mkv", vec!["-ASD"]),
        (3, 0, 0, 0, "audio_x8.mka", vec!["-a", "2-4"]),
    ]
    .into_iter()
    .for_each(|(a, s, d, b, file, args)| {
        let mc = cfg(args);
        let mut mi = MediaInfo::from(&mc);
        let saved = mi.try_get::<MISavedTracks>(&data_file(file)).unwrap();
        assert_eq!(a, saved[TrackType::Audio].len());
        assert_eq!(s, saved[TrackType::Sub].len());
        assert_eq!(d, saved[TrackType::Video].len());
        assert_eq!(b, saved[TrackType::Button].len());
    })
}

#[test]
fn test_ti_name() {
    let mut mi = new();

    [
        ("a", s_sep("name/a_name.mks"), ""),
        ("bc", s_sep("name/bc_name.mks"), ""),
        ("abc", s_sep("name/begin.abc.mks"), "begin"),
        ("tail", s_sep("name/begin.tail.mks"), "begin"),
        ("abc", s_sep("name/begin  .abc ..mks"), "begin"),
        ("abc", s_sep("name/other_begin.abc.mks"), "other_begin"),
        ("a", s_sep("name/from_parent/a/begin.mks"), "begin"),
        ("bc", s_sep("name/from_parent/bc/begin.mks"), "begin"),
    ]
    .iter()
    .for_each(|(name, file, cmn_stem)| {
        mi.upd_cmn_stem(new_stem(cmn_stem));
        assert_eq!(
            name,
            mi.try_get_ti::<MITIName>(&data_file(file), 0).unwrap()
        );
    })
}

#[test]
fn test_ti_lang() {
    let mut mi = new();

    [
        (LangCode::Und, s_sep("audio_x1.mka"), ""),
        (LangCode::Und, s_sep("sub_x1.mks"), "sub_x1"),
        (LangCode::Eng, s_sep("lang/en_lang.mks"), "en_lang"),
        (LangCode::Rus, s_sep("lang/ru_lang.mks"), "ru_lang"),
        (LangCode::Eng, s_sep("lang/begin.en.srt"), "begin"),
        (LangCode::Rus, s_sep("lang/begin.ru.srt"), "begin"),
    ]
    .into_iter()
    .for_each(|(lang, file, cmn_stem)| {
        mi.upd_cmn_stem(new_stem(cmn_stem));
        assert_eq!(
            lang,
            *mi.try_get_ti::<MITILang>(&data_file(file), 0).unwrap()
        );
    });

    let mc = cfg::<_, PathBuf>([PathBuf::from("-i"), data_dir()]);
    let mut mi = MediaInfo::from(&mc);

    [
        (LangCode::Und, s_sep("srt.srt")),
        (LangCode::Eng, s_sep("lang/eng subs/srt.srt")),
        (LangCode::Rus, s_sep("lang/rus subs/srt.srt")),
    ]
    .into_iter()
    .for_each(|(lang, file)| {
        assert_eq!(
            lang,
            *mi.try_get_ti::<MITILang>(&data_file(file), 0).unwrap()
        );
    });
}

#[test]
fn test_ti_track_ids() {
    let mut mi = new();

    [
        (0, LangCode::Und, s_sep("audio_x1.mka")),
        (0, LangCode::Eng, s_sep("lang/en_lang.mks")),
        (0, LangCode::Rus, s_sep("lang/ru_lang.mks")),
    ]
    .into_iter()
    .for_each(|(num, lang, file)| {
        assert_eq!(
            &[TrackID::Num(num), TrackID::Lang(lang)],
            mi.try_get_ti::<MITITrackIDs>(&data_file(file), num)
                .unwrap()
        );
    })
}
