use crate::{char_encoding, common::*, input};
use mux_media::{markers::*, *};
use std::{
    mem,
    path::{Path, PathBuf},
    sync::LazyLock,
};

static MUX_CONFIG: LazyLock<MuxConfig> = LazyLock::new(|| cfg::<_, &str>([]));

pub fn new() -> MediaInfo<'static> {
    MediaInfo::from(&*MUX_CONFIG)
}

fn new_path(s: &str) -> ArcPathBuf {
    ArcPathBuf::from(data(s))
}

#[test]
fn test_empty() {
    let mi = new();
    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_set_cmn_stem() {
    ["x", "a", "bc"].iter().for_each(|s| {
        let mut mi = new();
        mi.set_cmn::<MICmnStem>(s.into());
        assert_eq!(0, mi.len());
        assert!(!mi.is_empty());
        assert!(mi.is_no_files());
        assert_eq!(s, mi.try_get_cmn::<MICmnStem>().unwrap());
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
            mi.try_insert(new_path(f)).unwrap();
            assert_eq!(len, mi.len());
            assert!(!mi.is_empty());
            assert!(!mi.is_no_files());
        });

    mi.clear();

    ["missing", "bad"].iter().for_each(|f| {
        assert!(mi.try_insert(new_path(f)).is_err());
    });

    assert_eq!(0, mi.len());
}

#[test]
fn test_clear() {
    let mut mi = new();
    mi.set_cmn::<MICmnStem>("x".into());
    mi.try_insert(new_path("srt.srt")).unwrap();
    mi.clear();

    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_get_take_set_cache() {
    let mut mi = new();
    assert!(mi.cache.of_files.is_empty());

    ["srt.srt", "audio_x1.mka"].iter().for_each(|f| {
        let file = new_path(f);
        mi.try_insert(file.clone()).unwrap();
        mi.init_ti::<MITIName>(&file, 0).unwrap();

        let cache = &mi.cache;
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        mi.immut_ti::<MITIName>(&file, 0).unwrap();

        let cache = mem::take(&mut mi.cache);
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        assert!(mi.immut_ti::<MITIName>(&file, 0).is_none());

        mi.cache = cache;
        assert_eq!(&file, mi.cache.of_files.keys().next().unwrap());
        mi.immut_ti::<MITIName>(&file, 0).unwrap();

        mi.clear()
    })
}

#[test]
fn test_try_insert_many() {
    let paths = [new_path("srt.srt"), new_path("audio_x1.mka")];

    for (arg, len) in [("-D", 2), ("-SA", 2)] {
        let mc = cfg([arg]);
        let mut mi = MediaInfo::from(&mc);
        mi.try_insert_many(paths.clone(), true).unwrap();
        assert_eq!(len, mi.len());
    }

    let mut mi = new();
    let bad_paths = [new_path("missing"), new_path("bad")];
    for exit_on_err in [true, false] {
        assert_eq!(
            exit_on_err,
            mi.try_insert_many(bad_paths.clone(), exit_on_err).is_err()
        );
    }
    assert_eq!(0, mi.len());
}

#[test]
fn test_cmn_external_fonts() {
    input::TEST_INPUT_FONTS.iter().for_each(|(dir, _)| {
        let dir = input::data_font(dir);
        let mut cfg = cfg([Path::new("-i"), &dir]);
        cfg.try_finalize_init().unwrap();
        let mut mi = MediaInfo::from(&cfg);

        let collected = mi.try_take_cmn::<MICmnExternalFonts>().unwrap();
        let expected = cfg.input.collect_fonts_with_filter_and_sort();
        assert_eq!(collected, expected.into());
    })
}

#[test]
fn test_cmn_stem() {
    let mut mi = new();

    [("srt", "srt.srt"), ("audio_x1", "audio_x1.mka")]
        .into_iter()
        .for_each(|(stem, file)| {
            mi.try_insert(new_path(file)).unwrap();
            assert_eq!(stem, mi.try_get_cmn::<MICmnStem>().unwrap());
            mi.clear();
        });

    [
        "x1_set/x1_set.mkv",
        "x1_set/audio/x1_set.[audio].mka",
        "x1_set/subs/x1_set.[subs].mks",
    ]
    .iter()
    .for_each(|f| mi.try_insert(new_path(f)).unwrap());

    assert_eq!("x1_set", mi.try_get_cmn::<MICmnStem>().unwrap());
}

#[test]
fn test_cmn_media_order() {
    let mut mi = new();
    mi.try_insert(data("audio_x1.mka")).unwrap();
    mi.try_init_cmn::<MICmnTrackOrder>().unwrap();
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
        let file = data(file);
        assert_eq!(&SubCharset(enc), mi.try_get::<MISubCharset>(&file).unwrap());
    });

    assert!(
        mi.try_get::<MISubCharset>(data("audio_x1.mka").as_path())
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
            let f = data(f);
            assert_eq!(exp, mi.try_get::<MITargetGroup>(&f).unwrap());
        })
    })
}

#[test]
fn test_matroska() {
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
        let f = data(f);
        mi.try_get::<MIMatroska>(&f).unwrap();
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
        mi.try_get::<MIMkvmergeI>(data(f).as_path()).unwrap();
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
    let empty = Vec::<Target>::new();

    TEST_TARGETS
        .iter()
        .map(|(_, files)| files.iter())
        .flatten()
        .for_each(|f| {
            let f = data(f);
            assert_eq!(&empty, mi.try_get::<MITargets>(&f).unwrap());
        });
}

fn build_targets(slice: &[Target]) -> Vec<Target> {
    slice.into_iter().map(|trg| trg.clone()).collect()
}

#[test]
fn test_targets_group_only() {
    TEST_TARGETS.iter().for_each(|(group, files)| {
        let g_args = vec!["--target", group.as_ref(), "-C"];
        files.iter().for_each(|f| {
            let f = data(f);
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
            let f = data(f);
            let mc = cfg([Path::new("--target"), &f, Path::new("-C")]);
            let mut mi = MediaInfo::from(&mc);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&f))]);

            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_targets_parent_only() {
    let parent = data(""); //common for all files

    TEST_TARGETS.iter().for_each(|(_, files)| {
        files.iter().for_each(|f| {
            let f = data(f);
            let mc = cfg([Path::new("--target"), &parent, Path::new("-C")]);
            let mut mi = MediaInfo::from(&mc);
            let left = build_targets(&[Target::Path(ArcPathBuf::from(&parent))]);

            assert_eq!(&left, mi.try_get::<MITargets>(&f).unwrap());
        })
    })
}

#[test]
fn test_targets_all() {
    let parent = data(""); //common for all files

    TEST_TARGETS.iter().for_each(|(group, files)| {
        files.iter().for_each(|f| {
            let f = data(f);
            let args = [
                Path::new("--target"),
                group.as_path(),
                Path::new("-C"),
                Path::new("--target"),
                &f,
                Path::new("-C"),
                Path::new("--target"),
                &parent,
                Path::new("-C"),
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
fn test_audio_duration() {
    let mut mi = new();

    let d = mi.try_get::<MIAudioDuration>(&data("audio_x1.mka"));
    assert_eq!(*d.unwrap(), Duration::new(0, 989_742_774));

    mi.try_take::<MIAudioDuration>(&data("video_x1.mkv"))
        .unwrap_err();
}

#[test]
fn test_video_duration() {
    let mut mi = new();

    let d = mi.try_take::<MIVideoDuration>(&data("video_x1.mkv"));
    assert_eq!(d.unwrap(), Duration::new(0, 960_000_000));

    mi.try_take::<MIVideoDuration>(&data("audio_x1.mka"))
        .unwrap_err();
}

#[test]
fn test_playable_duration() {
    let mut mi = new();

    assert_eq(&mut mi, "video_x1.mkv", 0, 960_000_000);
    assert_eq(&mut mi, "audio_x1.mka", 0, 989_742_774);
    assert_eq(&mut mi, "vid_0.96s_and_srt_5s.mkv", 0, 960_000_000);
    assert_eq(&mut mi, "vid_0.96s_and_aud_0.99s.mkv", 0, 990_000_000);

    fn assert_eq(mi: &mut MediaInfo, f: &str, secs: u64, nanos: u32) {
        let d = mi.try_get::<MIPlayableDuration>(&data(f)).copied();
        assert_eq!(d.unwrap(), Duration::new(secs, nanos));
    }
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
            mi.get::<MITracksInfo>(data(f).as_path()).unwrap().len()
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
            mi.get::<MIAttachsInfo>(data(f).as_path()).unwrap().len()
        );
    })
}

#[test]
fn test_path_tail() {
    let mut mi = new();

    mi.set_cmn::<MICmnStem>("".into());

    [
        ("audio_x1", "audio_x1.mka"),
        ("sub_x1", "sub_x1.mks"),
        ("srt", "srt.srt"),
    ]
    .iter()
    .for_each(|(exp, f)| {
        let f = data(f);
        assert_eq!(&exp.to_string(), mi.get::<MIPathTail>(&f).unwrap());
    });

    mi.clear();
    mi.set_cmn::<MICmnStem>("s".into());

    [("ub_x1", "sub_x1.mks"), ("rt", "srt.srt")]
        .iter()
        .for_each(|(exp, f)| {
            let f = data(f);
            assert_eq!(&exp.to_string(), mi.get::<MIPathTail>(&f).unwrap());
        })
}

#[test]
fn test_relative_upmost() {
    // Upmost dir = data("")
    let mut mc = cfg::<_, PathBuf>([PathBuf::from("-i"), data("")]);
    mc.try_finalize_init().unwrap();
    let mut mi = MediaInfo::from(&mc);

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
        assert_eq!(exp, mi.get::<MIRelativeUpmost>(&f).unwrap());
    })
}

#[test]
fn test_saved_tracks() {
    [
        (1, 0, 0, "audio_x1.mka", vec![]),
        (8, 0, 0, "audio_x8.mka", vec![]),
        (0, 8, 0, "sub_x8.mks", vec![]),
        (0, 0, 8, "video_x8.mkv", vec![]),
        (0, 0, 0, "audio_x8.mka", vec!["-A"]),
        (0, 0, 0, "sub_x8.mks", vec!["-S"]),
        (0, 0, 0, "video_x8.mkv", vec!["-D"]),
        (3, 0, 0, "audio_x8.mka", vec!["-a", "2-4"]),
    ]
    .into_iter()
    .for_each(|(a, s, d, file, args)| {
        let mc = cfg(args);
        let mut mi = MediaInfo::from(&mc);
        let saved = mi.try_get::<MISavedTracks>(&data(file)).unwrap();
        assert_eq!(a, saved[TrackType::Audio].len());
        assert_eq!(s, saved[TrackType::Sub].len());
        assert_eq!(d, saved[TrackType::Video].len());
    })
}

#[test]
fn test_ti_name() {
    let mut mi = new();

    [
        ("a", "name/a_name.mks", ""),
        ("bc", "name/bc_name.mks", ""),
        ("abc", "name/begin.abc.mks", "begin"),
        ("tail", "name/begin.tail.mks", "begin"),
        ("abc", "name/begin  .abc ..mks", "begin"),
        ("abc", "name/other_begin.abc.mks", "other_begin"),
        ("a", "name/from_parent/a/begin.mks", "begin"),
        ("bc", "name/from_parent/bc/begin.mks", "begin"),
    ]
    .iter()
    .for_each(|(name, file, cmn_stem)| {
        mi.set_cmn::<MICmnStem>(cmn_stem.into());
        assert_eq!(
            name,
            mi.try_get_ti::<MITIName>(&data(file), 0).unwrap().deref()
        );
    })
}

#[test]
fn test_ti_lang() {
    let mut mi = new();

    [
        (LangCode::Und, "audio_x1.mka", ""),
        (LangCode::Und, "sub_x1.mks", "sub_x1"),
        (LangCode::Eng, "lang/en_lang.mks", "en_lang"),
        (LangCode::Rus, "lang/ru_lang.mks", "ru_lang"),
        (LangCode::Eng, "lang/begin.en.srt", "begin"),
        (LangCode::Rus, "lang/begin.ru.srt", "begin"),
    ]
    .into_iter()
    .for_each(|(lang, file, cmn_stem)| {
        mi.set_cmn::<MICmnStem>(cmn_stem.into());
        assert_eq!(
            lang,
            *mi.try_get_ti::<MITILang>(&data(file), 0).unwrap().deref()
        );
    });

    let mc = cfg::<_, PathBuf>([PathBuf::from("-i"), data("")]);
    let mut mi = MediaInfo::from(&mc);

    [
        (LangCode::Und, "srt.srt"),
        (LangCode::Eng, "lang/eng subs/srt.srt"),
        (LangCode::Rus, "lang/rus subs/srt.srt"),
    ]
    .into_iter()
    .for_each(|(lang, file)| {
        assert_eq!(
            lang,
            *mi.try_get_ti::<MITILang>(&data(file), 0).unwrap().deref()
        );
    });
}

#[test]
fn test_ti_track_ids() {
    let mut mi = new();

    [
        (0, LangCode::Und, "audio_x1.mka"),
        (0, LangCode::Eng, "lang/en_lang.mks"),
        (0, LangCode::Rus, "lang/ru_lang.mks"),
    ]
    .into_iter()
    .for_each(|(num, lang, file)| {
        assert_eq!(
            &[TrackID::Num(num), TrackID::Lang(lang)],
            mi.try_get_ti::<MITITrackIDs>(&data(file), num).unwrap()
        );
    })
}
