use crate::{common::*, *};
use mux_media::*;

#[test]
fn parse_names_default() {
    let xs = NameMetadata::default();
    assert_eq!(xs, cfg::<_, &str>([]).names);
}

#[test]
fn parse_langs_default() {
    let xs = LangMetadata::default();
    assert_eq!(xs, cfg::<_, &str>([]).langs);
}

#[test]
fn parse_names_single_val() {
    let mut xs = NameMetadata::default();
    xs.0.single_val = Some("x".into());
    assert_eq!(xs, cfg(["--names", "x"]).names);
}

#[test]
fn parse_langs_single_val() {
    let mut xs = LangMetadata::default();
    xs.0.single_val = Some(LangCode::Eng);
    assert_eq!(xs, cfg(["--langs", "eng"]).langs);
}

#[test]
fn parse_names_idxs() {
    let mut xs = NameMetadata::default();
    xs.0.idxs = Some([(0, "a".into()), (8, "b".into())].into());

    assert_eq!(xs, cfg(["--names", "0:a,8:b"]).names);
}

#[test]
fn parse_langs_idxs() {
    let mut xs = LangMetadata::default();
    xs.0.idxs = Some([(0, LangCode::Eng), (8, LangCode::Rus)].into());

    assert_eq!(xs, cfg(["--langs", "0:eng,8:rus"]).langs);
}

#[test]
fn parse_names_ranges() {
    let mut xs = NameMetadata::default();
    xs.0.ranges = Some(
        [
            (range::new("0-1"), "a".into()),
            (range::new("8-8"), "b".into()),
        ]
        .into(),
    );

    assert_eq!(xs, cfg(["--names", "0-1:a,8-8:b"]).names);
}

#[test]
fn parse_langs_ranges() {
    let mut xs = LangMetadata::default();
    xs.0.ranges = Some(
        [
            (range::new("0-1"), LangCode::Eng),
            (range::new("8-8"), LangCode::Rus),
        ]
        .into(),
    );

    assert_eq!(xs, cfg(["--langs", "0-1:eng,8-8:rus"]).langs);
}

#[test]
fn parse_names_langs() {
    let mut xs = NameMetadata::default();
    xs.0.langs = Some([(LangCode::Eng, "a".into()), (LangCode::Rus, "b".into())].into());

    assert_eq!(xs, cfg(["--names", "eng:a,rus:b"]).names);
}

#[test]
fn parse_langs_langs() {
    let mut xs = LangMetadata::default();
    xs.0.langs = Some(
        [
            (LangCode::Eng, LangCode::Rus),
            (LangCode::Rus, LangCode::Eng),
        ]
        .into(),
    );

    assert_eq!(xs, cfg(["--langs", "eng:rus,rus:eng"]).langs);
}

#[test]
fn get_default() {
    let names = NameMetadata::default();
    let langs = LangMetadata::default();

    for (i, lang) in iter_i_lang() {
        assert_eq!(None, names.get(i, lang));
        assert_eq!(None, langs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, names.get(i, lang));
        assert_eq!(None, langs.get(i, lang));
    }
}

#[test]
fn get_single_val() {
    let x = String::from("x");
    let mut xs = NameMetadata::default();
    xs.0.single_val = Some(x.clone());

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(&x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(Some(&x), xs.get(i, lang));
    }
}

#[test]
fn get_idxs() {
    let x = String::from("x");
    let mut xs = NameMetadata::default();
    xs.0.idxs = Some(
        [
            (0, x.clone()),
            (1, x.clone()),
            (8, x.clone()),
            (!0 - 1, x.clone()),
        ]
        .into(),
    );

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(&x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_ranges() {
    let x = String::from("x");
    let mut xs = NameMetadata::default();
    xs.0.ranges = Some(vec![
        (range::new("0-1"), x.clone()),
        (range::new("8-8"), x.clone()),
        (range::new(&format!("{}", usize::MAX - 1)), x.clone()),
    ]);

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(&x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_langs() {
    let x = String::from("x");
    let mut xs = NameMetadata::default();
    xs.0.langs = Some(
        [
            (LangCode::Eng, x.clone()),
            (LangCode::Rus, x.clone()),
            (LangCode::Und, x.clone()),
        ]
        .into(),
    );

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(&x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

build_test_to_json_args!(
    to_json_args_names, names, "names";
    vec![],
    vec!["--names", "x"],
    vec!["--names", "1:a,2:b,8:c"],
    vec!["--names", "1-2:a,8-8:c"],
    vec!["--names", "eng:a,rus:b,und:c"],
    vec!["--names", "1:a,2-8:b,eng:c"],
);

build_test_to_json_args!(
    to_json_args_langs, langs, "langs";
    vec![],
    vec!["--langs", "eng"],
    vec!["--langs", "1:eng,2:rus,8:und"],
    vec!["--langs", "1-2:eng,8-8:rus"],
    vec!["--langs", "eng:und,rus:eng,und:rus"],
    vec!["--langs", "1:eng,2-8:rus,eng:und"],
);

#[test]
fn to_ffmpeg_args_names_solo_default() {
    for f in ["srt.srt", "audio_x1.mka"] {
        let mut mi = media_info::new();
        mi.try_insert(data(f)).unwrap();
        mi.try_finalize_init_streams().unwrap();

        let exp = to_os_args(["-metadata:s:0", "title=test_data"]);
        assert_eq!(exp, NameMetadata::to_ffmpeg_args(&mut mi).unwrap());
    }
}

#[test]
fn to_ffmpeg_args_names_solo_single_val() {
    let cfg = cfg(["--names", "x"]);

    for f in ["srt.srt", "audio_x1.mka"] {
        let mut mi = MediaInfo::new(&cfg, 0);
        mi.try_insert(data(f)).unwrap();
        mi.try_finalize_init_streams().unwrap();

        let exp = to_os_args(["-metadata:s:0", "title=x"]);
        assert_eq!(exp, NameMetadata::to_ffmpeg_args(&mut mi).unwrap());
    }
}

#[test]
fn to_ffmpeg_args_names_set_single_val() {
    let cfg = cfg(["--names", "x"]);
    let mut mi = MediaInfo::new(&cfg, 0);
    for f in ["video_x1.mkv", "ogg.ogg", "srt.srt"] {
        mi.try_insert(data(f)).unwrap();
    }
    mi.try_finalize_init_streams().unwrap();

    let exp = to_os_args([
        "-metadata:s:0",
        "title=x",
        "-metadata:s:1",
        "title=x",
        "-metadata:s:2",
        "title=x",
    ]);
    assert_eq!(exp, NameMetadata::to_ffmpeg_args(&mut mi).unwrap());
}
