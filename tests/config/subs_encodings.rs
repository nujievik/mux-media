use super::*;
use encoding_rs::Encoding;

fn enc(s: &str) -> &'static Encoding {
    Encoding::for_label_no_replacement(s.as_bytes()).unwrap()
}

#[test]
fn parse_default() {
    let xs = ConfigSubsEncodings::default();
    assert_eq!(&xs, &CONFIG.subs_encodings);
}

#[test]
fn parse_single_val() {
    for v in ["cp1251", "utf-8", "utf-16"] {
        let xs = ConfigSubsEncodings {
            single_val: Some(enc(v)),
            ..Default::default()
        };
        assert_eq!(xs, cfg(["--subs-encodings", v]).subs_encodings);
    }
}

#[test]
fn parse_idxs() {
    for (v, v2) in [
        ("cp1251", "cp1251"),
        ("cp1251", "utf-8"),
        ("utf-8", "utf-16"),
    ] {
        let xs = ConfigSubsEncodings {
            idxs: Some(
                [(0, enc(v)), (1, enc(v)), (8, enc(v2))]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };
        let arg = format!("0:{},1:{},8:{}", v, v, v2);

        assert_eq!(xs, cfg(["--subs-encodings", &arg]).subs_encodings);
    }
}

#[test]
fn parse_ranges() {
    for (v, v2) in [
        ("cp1251", "cp1251"),
        ("cp1251", "utf-8"),
        ("utf-8", "utf-16"),
    ] {
        let xs = ConfigSubsEncodings {
            ranges: Some(vec![
                (range::new("0-1"), enc(v)),
                (range::new("8-8"), enc(v2)),
            ]),
            ..Default::default()
        };
        let arg = format!("0-1:{},8-8:{}", v, v2);

        assert_eq!(xs, cfg(["--subs-encodings", &arg]).subs_encodings);
    }
}

#[test]
fn parse_langs() {
    for (v, v2) in [
        ("cp1251", "cp1251"),
        ("cp1251", "utf-8"),
        ("utf-8", "utf-16"),
    ] {
        let langs = [(lang!(Eng), enc(v)), (lang!(Und), enc(v2))];

        let xs = ConfigSubsEncodings {
            langs: Some(langs.into_iter().collect()),
            ..Default::default()
        };
        let arg = format!("eng:{},und:{}", v, v2);

        assert_eq!(xs, cfg(["--subs-encodings", &arg]).subs_encodings);
    }
}

#[test]
fn get_default() {
    let xs = &CONFIG.subs_encodings;

    for (i, lang) in iter_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_single_val() {
    let mut xs = ConfigSubsEncodings::default();

    for v in ["cp1251", "utf-8", "utf-16"] {
        let v = enc(v);
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
    let x = enc("cp1251");

    let mut xs = ConfigSubsEncodings::default();
    xs.idxs = Some([(0, x), (1, x), (8, x), (!0 - 1, x)].into_iter().collect());

    for (i, lang) in iter_i_lang() {
        assert_eq!(Some(x), xs.get(i, lang));
    }
    for (i, lang) in iter_alt_i_lang() {
        assert_eq!(None, xs.get(i, lang));
    }
}

#[test]
fn get_ranges() {
    let x = enc("cp1251");

    let mut xs = ConfigSubsEncodings::default();
    xs.ranges = Some(vec![
        (range::new("0-1"), x),
        (range::new("8-8"), x),
        (range::new(&format!("{}", usize::MAX - 1)), x),
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
    let x = enc("cp1251");

    let mut xs = ConfigSubsEncodings::default();
    xs.langs = Some(
        [(lang!(Eng), x), (lang!(Rus), x), (lang!(Und), x)]
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
    to_args, "config_subs_encodings";
    vec![],
    vec!["--subs-encodings", "windows-1251"],
    vec!["--subs-encodings", "1:windows-1251,2:UTF-8,8:windows-1251"],
    vec!["--subs-encodings", "1:UTF-8,2-4:UTF-16LE,eng:UTF-8"],
);
