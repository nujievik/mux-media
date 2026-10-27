#[allow(dead_code)]
mod common;

use crate::common::*;
use mux_media::*;

#[test]
fn test_config_parse() {
    let arg_lang_pairs = [("rus", LangCode::Rus), ("eng", LangCode::Eng)];

    for (arg, lang) in &arg_lang_pairs {
        assert_eq!(cfg(["--locale", arg]).locales.locale(), *lang);
        assert_eq!(Msg::lang(), *lang);
    }

    for (arg, lang) in &arg_lang_pairs {
        assert_eq!(cfg(["--locale-log", arg]).locales.locale(), *lang);
        assert_eq!(Msg::lang(), *lang);
    }

    [("jpn", LangCode::Jpn), ("und", LangCode::Und)]
        .into_iter()
        .for_each(|(arg, lang)| {
            assert_eq!(cfg(["--locale", arg]).locales.locale(), lang);
            // Unsupported langs do not sets in Msg
            assert_eq!(Msg::lang(), LangCode::Eng);
        });
}
