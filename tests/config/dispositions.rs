use super::*;

#[test]
fn parse_default() {
    let xs = ConfigDispositions::default();
    let cfg = cfg::<_, &str>([]);
    assert_eq!(xs.clone(), cfg.defaults);
    assert_eq!(xs, cfg.forceds);
}

#[test]
fn parse_max() {
    for (&i, _) in iter_i_lang() {
        let xs = ConfigDispositions {
            max_in_auto: Some(i),
            ..Default::default()
        };
        let i = i.to_string();
        assert_eq!(xs, cfg(["--max-defaults", &i]).defaults);
        assert_eq!(xs, cfg(["--max-forceds", &i]).forceds);
    }
}

#[test]
fn parse_single_val() {
    for v in [true, false] {
        let xs = ConfigDispositions {
            single_val: Some(v),
            ..Default::default()
        };
        let v = v.to_string();
        assert_eq!(xs, cfg(["--defaults", &v]).defaults);
        assert_eq!(xs, cfg(["--forceds", &v]).forceds);
    }
}

#[test]
fn parse_idxs() {
    for (v, v2) in [(true, false), (false, true)] {
        let xs = ConfigDispositions {
            idxs: Some([(0, v), (1, v), (8, v2)].into()),
            ..Default::default()
        };
        let vs = format!("0:{},1:{},8:{}", v, v, v2);

        assert_eq!(xs, cfg(["--defaults", &vs]).defaults);
        assert_eq!(xs, cfg(["--forceds", &vs]).forceds);
    }
}

#[test]
fn parse_ranges() {
    for (v, v2) in [(true, false), (false, true)] {
        let xs = ConfigDispositions {
            ranges: Some(vec![(range::new("0-1"), v), (range::new("8-8"), v2)]),
            ..Default::default()
        };
        let vs = format!("0-1:{},8-8:{}", v, v2);

        assert_eq!(xs, cfg(["--defaults", &vs]).defaults);
        assert_eq!(xs, cfg(["--forceds", &vs]).forceds);
    }
}

#[test]
fn parse_langs() {
    for (v, v2) in [(true, false), (false, true)] {
        let xs = ConfigDispositions {
            langs: Some([(lang!(Eng), v), (lang!(Und), v2)].into()),
            ..Default::default()
        };
        let vs = format!("eng:{},und:{}", v, v2);

        assert_eq!(xs, cfg(["--defaults", &vs]).defaults);
        assert_eq!(xs, cfg(["--forceds", &vs]).forceds);
    }
}

#[test]
fn get_default() {
    let xs = ConfigDispositions::default();

    for (i, lang) in iter_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_single_val() {
    let mut xs = ConfigDispositions::default();

    for v in [true, false] {
        xs.single_val = Some(v);

        for (i, lang) in iter_i_lang() {
            assert_eq!(Some(v), xs.get(i, lang));
        }
        for (i, lang) in iter_alt_i_lang() {
            assert_eq!(Some(v), xs.get(i, lang));
        }
    }
}

#[test]
fn get_idxs() {
    let idxs = [
        (0, true),
        (1, true),
        (8, true),
        (!0 - 1, true),
        (5, false),
        (10, false),
        (11, false),
        (!0 - 2, false),
    ];
    let xs = ConfigDispositions {
        idxs: Some(idxs.into()),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(true), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(Some(false), xs.get(i, lang));
    }
}

#[test]
fn get_ranges() {
    let ranges = [
        (range::new("0-1"), true),
        (range::new("8-8"), true),
        (range::new(&format!("{}-", usize::MAX - 1)), true),
        (range::new("5-5"), false),
        (range::new("10-11"), false),
        (
            range::new(&format!("{}-{}", usize::MAX - 2, usize::MAX - 2)),
            false,
        ),
    ];
    let xs = ConfigDispositions {
        ranges: Some(ranges.into()),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(true), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(Some(false), xs.get(i, lang));
    }
}

#[test]
fn get_langs() {
    let langs = [
        (lang!(Eng), true),
        (lang!(Rus), true),
        (lang!(Und), true),
        (lang!(Abk), false),
        (lang!(Aar), false),
        (lang!(Afr), false),
    ];
    let xs = ConfigDispositions {
        langs: Some(langs.into()),
        ..Default::default()
    };

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(true), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(Some(false), xs.get(i, lang));
    }
}

#[test]
fn max_default() {
    let xs = ConfigDispositions::default();
    assert_eq!(1, xs.max(DispositionType::Default));
    assert_eq!(0, xs.max(DispositionType::Forced));
}

#[test]
fn max_user() {
    let mut xs = ConfigDispositions::default();
    for (&i, _) in iter_i_lang() {
        xs.max_in_auto = Some(i);
        for ty in [DispositionType::Default, DispositionType::Forced] {
            assert_eq!(i, xs.max(ty))
        }
    }
}

build_test_to_args!(
    to_args_defaults, "defaults";
    vec![],
    vec!["--max-defaults", "5"],
    vec!["--defaults", "true"],
    vec!["--defaults", "1:true,2:false,8:true"],
    vec!["--defaults", "false", "--max-defaults", "1"],
);

build_test_to_args!(
    to_args_forceds, "forceds";
    vec![],
    vec!["--max-forceds", "5"],
    vec!["--forceds", "true"],
    vec!["--forceds", "1:true,2:false,8:true"],
    vec!["--forceds", "false", "--max-forceds", "1"],
);
