use super::*;

#[test]
fn parse_empty_args() {
    let xs = &EMPTY_ARGS.locales;
    assert!(xs.get_locale_streams().is_none());
    assert!(xs.get_locale_log().is_none());
}

#[test]
fn parse_locale() {
    for l in [LangCode::Eng, LangCode::Rus, LangCode::Jpn] {
        let xs = cfg(["--locale", l.as_ref()]).locales;
        assert_eq!(l, xs.locale());
        assert!(xs.get_locale_streams().is_none());
        assert!(xs.get_locale_log().is_none());
    }
}

#[test]
fn parse_locale_streams() {
    for l in [LangCode::Eng, LangCode::Rus, LangCode::Jpn] {
        let xs = cfg(["--locale-streams", l.as_ref()]).locales;
        assert_eq!(l, xs.get_locale_streams().unwrap());
        assert_eq!(l, xs.locale_streams());
        assert!(xs.get_locale_log().is_none());
    }
}

#[test]
fn parse_locale_log() {
    for l in [LangCode::Eng, LangCode::Rus] {
        let xs = cfg(["--locale-log", l.as_ref()]).locales;
        assert!(xs.get_locale_streams().is_none());
        assert_eq!(l, xs.get_locale_log().unwrap());
        assert_eq!(l, xs.locale_log());
    }
}

#[test]
fn empty_to_args() {
    let empty = &EMPTY_ARGS;
    let empty_to_args = empty.to_args().unwrap();
    assert_eq!(
        &empty_to_args[..2],
        &append_str_vecs([["--locale", empty.locales.locale().as_str()]])
    );
    assert!(!matches!(
        empty_to_args[2].as_str(),
        "--locale-streams" | "--locale-log"
    ));
}

#[test]
fn locale_to_args() {
    for l in [LangCode::Eng, LangCode::Rus, LangCode::Jpn] {
        let args = ["--locale", l.as_str()];
        let cfg = cfg(&args);
        let cfg_to_args = cfg.to_args().unwrap();

        assert_eq!(
            &cfg_to_args[..2],
            &append_str_vecs([["--locale", l.as_str()]])
        );
        assert!(!matches!(
            cfg_to_args[2].as_str(),
            "--locale-streams" | "--locale-log"
        ));
    }
}

#[test]
fn locale_streams_to_args() {
    for l in [LangCode::Eng, LangCode::Rus, LangCode::Jpn] {
        let args = ["--locale-streams", l.as_str()];
        let cfg = cfg(&args);
        let cfg_to_args = cfg.to_args().unwrap();

        assert_eq!("--locale", &cfg_to_args[0]);
        assert_eq!(
            &cfg_to_args[2..4],
            &append_str_vecs([["--locale-streams", l.as_str()]])
        );
        assert!(!matches!(cfg_to_args[4].as_str(), "--locale-log"));
    }
}

#[test]
fn locale_log_to_args() {
    for l in [LangCode::Eng, LangCode::Rus] {
        let args = ["--locale-log", l.as_str()];
        let cfg = cfg(&args);
        let cfg_to_args = cfg.to_args().unwrap();

        assert_eq!("--locale", &cfg_to_args[0]);
        assert_eq!(
            &cfg_to_args[2..4],
            &append_str_vecs([["--locale-log", l.as_str()]])
        );
    }
}
