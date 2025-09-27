#[allow(dead_code)]
mod common;

use crate::common::*;
use mux_media::*;

#[test]
fn test_mux_config_parse() {
    [("rus", LangCode::Rus), ("eng", LangCode::Eng)]
        .into_iter()
        .for_each(|(arg, lang)| {
            assert_eq!(cfg(["--locale", arg]).locale, lang);
            assert_eq!(Msg::lang(), lang);
        });

    [("jpn", LangCode::Jpn), ("und", LangCode::Und)]
        .into_iter()
        .for_each(|(arg, lang)| {
            assert_eq!(cfg(["--locale", arg]).locale, lang);
            // Unsupported langs do not sets in Msg
            assert_eq!(Msg::lang(), LangCode::Eng);
        });
}
