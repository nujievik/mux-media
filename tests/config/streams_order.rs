use super::*;

#[test]
fn parse_default() {
    let xs = ConfigStreamsOrder::default();
    let cfg = cfg::<_, &str>([]);
    assert_eq!(xs.clone(), cfg.order);
}

#[test]
fn parse_single_val() {
    for v in [0, 1, 8, 255] {
        let xs = ConfigStreamsOrder {
            single_val: Some(v),
            ..Default::default()
        };
        let arg = v.to_string();
        assert_eq!(xs, cfg(["--order", &arg]).order);
    }
}

#[test]
fn parse_idxs() {
    for (v, v2) in [(0, 1), (1, 0), (8, 255)] {
        let xs = ConfigStreamsOrder {
            idxs: Some([(0, v), (1, v), (8, v2)].into_iter().collect()),
            ..Default::default()
        };
        let arg = format!("0:{},1:{},8:{}", v, v, v2);

        assert_eq!(xs, cfg(["--order", &arg]).order);
    }
}

#[test]
fn parse_ranges() {
    for (v, v2) in [(0, 1), (1, 0), (8, 255)] {
        let xs = ConfigStreamsOrder {
            ranges: Some(vec![(range::new("0-1"), v), (range::new("8-8"), v2)]),
            ..Default::default()
        };
        let arg = format!("0-1:{},8-8:{}", v, v2);

        assert_eq!(xs, cfg(["--order", &arg]).order);
    }
}

#[test]
fn parse_langs() {
    for (v, v2) in [(0, 1), (1, 0), (8, 255)] {
        let langs = [(lang!(Eng), v), (lang!(Und), v2)];

        let xs = ConfigStreamsOrder {
            langs: Some(langs.into_iter().collect()),
            ..Default::default()
        };
        let arg = format!("eng:{},und:{}", v, v2);

        assert_eq!(xs, cfg(["--order", &arg]).order);
    }
}

#[test]
fn get_default() {
    let xs = ConfigStreamsOrder::default();

    for (i, lang) in iter_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_single_val() {
    let mut xs = ConfigStreamsOrder::default();

    for v in [0, 1, 8, 255] {
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
    let x = 128u8;

    let mut xs = ConfigStreamsOrder::default();
    xs.idxs = Some(
        [
            (0, x.clone()),
            (1, x.clone()),
            (8, x.clone()),
            (!0 - 1, x.clone()),
        ]
        .into_iter()
        .collect(),
    );

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_ranges() {
    let x = 128u8;

    let mut xs = ConfigStreamsOrder::default();
    xs.ranges = Some(vec![
        (range::new("0-1"), x.clone()),
        (range::new("8-8"), x.clone()),
        (range::new(&format!("{}", usize::MAX - 1)), x.clone()),
    ]);

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_langs() {
    let x = 128u8;

    let mut xs = ConfigStreamsOrder::default();
    xs.langs = Some(
        [
            (lang!(Eng), x.clone()),
            (lang!(Rus), x.clone()),
            (lang!(Und), x.clone()),
        ]
        .into_iter()
        .collect(),
    );

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

build_test_to_args!(
    to_args, "config_streams_order";
    vec![],
    vec!["--order", "128"],
    vec!["--order", "1:0,2:1,8:4"],
    vec!["--order", "1:0,2-4:1,eng:4"],
);
