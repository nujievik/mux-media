use crate::{common, *};
use mux_media::{markers::*, *};
use std::collections::HashSet;

fn new(args: &[&str]) -> Streams {
    common::cfg_field(CfgStreams, args)
}

#[test]
fn parse_no_flag() {
    let xs = Streams {
        no_flag: true,
        ..Default::default()
    };
    assert_eq!(xs, new(&["--no-streams"]));
}

#[test]
fn parse_idxs() {
    [
        ("0", vec![0]),
        ("1,2,8", vec![1, 2, 8]),
        (&format!("{}", usize::MAX - 1), vec![usize::MAX - 1]),
    ]
    .into_iter()
    .for_each(|(arg, idxs)| {
        let xs = Streams {
            idxs: Some(HashSet::from_iter(idxs)),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_idxs_inverse() {
    [
        ("!0", vec![0]),
        ("!1,2,8", vec![1, 2, 8]),
        (&format!("!{}", usize::MAX - 1), vec![usize::MAX - 1]),
    ]
    .into_iter()
    .for_each(|(arg, idxs)| {
        let xs = Streams {
            inverse: true,
            idxs: Some(HashSet::from_iter(idxs)),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_ranges() {
    let max = usize::MAX - 1;

    [
        ("1-1", vec!["1-1"]),
        ("1-2,8-8", vec!["1-2", "8-8"]),
        (
            &format!("{}-{}", max, max),
            vec![&format!("{}-{}", max, max)],
        ),
    ]
    .into_iter()
    .for_each(|(arg, idxs)| {
        let ranges = idxs.iter().map(|s| range::new(s)).collect();
        let xs = Streams {
            ranges: Some(ranges),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_ranges_inverse() {
    let max = usize::MAX - 1;

    [
        ("!1-1", vec!["1-1"]),
        ("!1-2,8-8", vec!["1-2", "8-8"]),
        (
            &format!("!{}-{}", max, max),
            vec![&format!("{}-{}", max, max)],
        ),
    ]
    .into_iter()
    .for_each(|(arg, idxs)| {
        let ranges = idxs.iter().map(|s| range::new(s)).collect();
        let xs = Streams {
            inverse: true,
            ranges: Some(ranges),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_langs() {
    [
        ("eng", vec![LangCode::Eng]),
        (
            "eng,rus,und",
            vec![LangCode::Eng, LangCode::Rus, LangCode::Und],
        ),
    ]
    .into_iter()
    .for_each(|(arg, langs)| {
        let xs = Streams {
            langs: Some(HashSet::from_iter(langs)),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_langs_inverse() {
    [
        ("!eng", vec![LangCode::Eng]),
        (
            "!eng,rus,und",
            vec![LangCode::Eng, LangCode::Rus, LangCode::Und],
        ),
    ]
    .into_iter()
    .for_each(|(arg, langs)| {
        let xs = Streams {
            inverse: true,
            langs: Some(HashSet::from_iter(langs)),
            ..Default::default()
        };
        assert_eq!(xs, new(&["--streams", arg]));
    })
}

#[test]
fn parse_all() {
    let xs = Streams {
        no_flag: false,
        inverse: true,
        idxs: Some([1, 8].into()),
        ranges: Some(vec![range::new("2-4")]),
        langs: Some([LangCode::Eng, LangCode::Und].into()),
    };
    assert_eq!(xs, new(&["--streams", "!1,eng,8,und,2-4"]));
}

fn iter_i_lang() -> impl Iterator<Item = (&'static usize, &'static LangCode)> {
    static I: [usize; 4] = [0, 1, 8, usize::MAX - 1];
    static LANGS: [LangCode; 3] = [LangCode::Eng, LangCode::Rus, LangCode::Und];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

fn iter_alt_i_lang() -> impl Iterator<Item = (&'static usize, &'static LangCode)> {
    static I: [usize; 4] = [5, 10, 11, usize::MAX - 2];
    static LANGS: [LangCode; 3] = [LangCode::Abk, LangCode::Aar, LangCode::Afr];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

#[test]
fn is_save_default() {
    let xs = Streams::default();
    for (i, lang) in iter_i_lang() {
        assert!(xs.is_save(i, lang));
    }
}

#[test]
fn is_save_no_flag() {
    let xs = Streams {
        no_flag: true,
        ..Default::default()
    };
    for (i, lang) in iter_i_lang() {
        assert!(!xs.is_save(i, lang));
    }
}

#[test]
fn is_save_idxs() {
    let mut xs = Streams {
        idxs: Some([0, 1, 8, usize::MAX - 1].into()),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert!(xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    xs.inverse = true;

    for (i, lang) in iter_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(xs.is_save(i, lang));
    }
}

#[test]
fn is_save_ranges() {
    let rng_0_1 = range::new("0-1");
    let rng_8 = range::new("8-8");
    let rng_max = range::new(&format!("{}-", usize::MAX - 1));
    let mut xs = Streams {
        ranges: Some(vec![rng_0_1, rng_8, rng_max]),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert!(xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    xs.inverse = true;

    for (i, lang) in iter_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(xs.is_save(i, lang));
    }
}

#[test]
fn is_save_langs() {
    let mut xs = Streams {
        langs: Some([LangCode::Eng, LangCode::Rus, LangCode::Und].into()),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert!(xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    xs.inverse = true;

    for (i, lang) in iter_i_lang() {
        assert!(!xs.is_save(i, lang));
    }

    for (i, lang) in iter_alt_i_lang() {
        assert!(xs.is_save(i, lang));
    }
}

build_test_to_json_args!(
    to_json_args, streams, "streams";
    vec!["--no-streams"],
    vec!["--streams", "0"],
    vec!["--streams", "1,2,8"],
    vec!["--streams", "2-8,8-16"],
    vec!["--streams", "eng,rus,und"],
    vec!["--streams", "!1,2,8-16,eng"],
);
