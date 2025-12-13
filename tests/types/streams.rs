use crate::{common::*, *};
use mux_media::*;
use std::collections::{HashMap, HashSet};

fn new(args: &[&str]) -> Streams {
    cfg(args).streams
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
fn parse_no_flag_aliases() {
    let xs = Streams {
        no_flag: true,
        ..Default::default()
    };

    for (cli, ty) in [
        ("-A", StreamType::Audio),
        ("-S", StreamType::Sub),
        ("-D", StreamType::Video),
        ("-F", StreamType::Font),
        ("-M", StreamType::Attach),
    ] {
        let t = Target::Stream(ty);
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };
        let ts = HashMap::from([(t, val)]);

        assert_eq!(ts, cfg([cli]).targets.unwrap());
    }
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
fn parse_idxs_aliases() {
    let xs = Streams {
        idxs: Some([0].into()),
        ..Default::default()
    };

    for (cli, ty) in [
        ("-a0", StreamType::Audio),
        ("-s0", StreamType::Sub),
        ("-d0", StreamType::Video),
        ("-f0", StreamType::Font),
        ("-m0", StreamType::Attach),
    ] {
        let t = Target::Stream(ty);
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };
        let ts = HashMap::from([(t, val)]);

        assert_eq!(ts, cfg([cli]).targets.unwrap());
    }
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
fn parse_ranges_aliases() {
    let xs = Streams {
        ranges: Some(vec![range::new("0-1")]),
        ..Default::default()
    };

    for (cli, ty) in [
        ("-a0-1", StreamType::Audio),
        ("-s0-1", StreamType::Sub),
        ("-d0-1", StreamType::Video),
        ("-f0-1", StreamType::Font),
        ("-m0-1", StreamType::Attach),
    ] {
        let t = Target::Stream(ty);
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };
        let ts = HashMap::from([(t, val)]);

        assert_eq!(ts, cfg([cli]).targets.unwrap());
    }
}

#[test]
fn parse_langs() {
    [
        ("eng", vec![lang!(Eng)]),
        ("eng,rus,und", vec![lang!(Eng), lang!(Rus), lang!(Und)]),
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
        ("!eng", vec![lang!(Eng)]),
        ("!eng,rus,und", vec![lang!(Eng), lang!(Rus), lang!(Und)]),
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
fn parse_langs_aliases() {
    let xs = Streams {
        langs: Some([lang!(Eng)].into()),
        ..Default::default()
    };

    for (cli, ty) in [
        ("-aeng", StreamType::Audio),
        ("-seng", StreamType::Sub),
        ("-deng", StreamType::Video),
        ("-feng", StreamType::Font),
        ("-meng", StreamType::Attach),
    ] {
        let t = Target::Stream(ty);
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };
        let ts = HashMap::from([(t, val)]);

        assert_eq!(ts, cfg([cli]).targets.unwrap());
    }
}

#[test]
fn parse_all() {
    let xs = Streams {
        no_flag: false,
        inverse: true,
        idxs: Some([1, 8].into()),
        ranges: Some(vec![range::new("2-4")]),
        langs: Some([lang!(Eng), lang!(Und)].into()),
    };
    assert_eq!(xs, new(&["--streams", "!1,eng,8,und,2-4"]));
}

#[test]
fn parse_target_switching() {
    let xs = Streams {
        no_flag: true,
        ..Default::default()
    };
    let args = [
        "--target",
        "audio",
        "--no-streams",
        "--target",
        "global",
        "--no-streams",
        "--target",
        "attach",
        "--no-streams",
        "--target",
        "sub",
        "--no-streams",
        "--target",
        "video",
        "--no-streams",
        "--target",
        "font",
        "--no-streams",
    ];

    let ts = [
        StreamType::Audio,
        StreamType::Attach,
        StreamType::Sub,
        StreamType::Video,
        StreamType::Font,
    ]
    .into_iter()
    .map(|ty| {
        let t = Target::Stream(ty);
        let val = ConfigTarget {
            streams: Some(xs.clone()),
            ..Default::default()
        };
        (t, val)
    })
    .collect::<HashMap<_, _>>();

    let cfg = cfg(args);
    assert_eq!(xs, cfg.streams);
    assert_eq!(ts, cfg.targets.unwrap());
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
        langs: Some([lang!(Eng), lang!(Rus), lang!(Und)].into()),
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
