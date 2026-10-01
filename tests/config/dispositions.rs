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
            single_val: Some(Bool(v)),
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
            idxs: Some(
                [(0, Bool(v)), (1, Bool(v)), (8, Bool(v2))]
                    .into_iter()
                    .collect(),
            ),
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
            ranges: Some(vec![
                (range::new("0-1"), Bool(v)),
                (range::new("8-8"), Bool(v2)),
            ]),
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
        let langs = [(lang!(Eng), Bool(v)), (lang!(Und), Bool(v2))];

        let xs = ConfigDispositions {
            langs: Some(langs.into_iter().collect()),
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
        xs.single_val = Some(Bool(v));

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
        (0, Bool(true)),
        (1, Bool(true)),
        (8, Bool(true)),
        (!0 - 1, Bool(true)),
        (5, Bool(false)),
        (10, Bool(false)),
        (11, Bool(false)),
        (!0 - 2, Bool(false)),
    ];

    let xs = ConfigDispositions {
        idxs: Some(idxs.into_iter().collect()),
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
        (range::new("0-1"), Bool(true)),
        (range::new("8-8"), Bool(true)),
        (range::new(&format!("{}-", usize::MAX - 1)), Bool(true)),
        (range::new("5-5"), Bool(false)),
        (range::new("10-11"), Bool(false)),
        (
            range::new(&format!("{}-{}", usize::MAX - 2, usize::MAX - 2)),
            Bool(false),
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
        (lang!(Eng), Bool(true)),
        (lang!(Rus), Bool(true)),
        (lang!(Und), Bool(true)),
        (lang!(Abk), Bool(false)),
        (lang!(Aar), Bool(false)),
        (lang!(Afr), Bool(false)),
    ];
    let xs = ConfigDispositions {
        langs: Some(langs.into_iter().collect()),
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
