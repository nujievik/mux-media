use mux_media::*;

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    assert!(LangCode::default().is_default());
    assert!(!LangCode::Jpn.is_default());
}

crate::test_from_str!(
    LangCode, test_from_str,
    [
        (LangCode::Eng, "eng"),
        (LangCode::Eng, "en"),
        (LangCode::Rus, "rus"),
        (LangCode::Rus, "ru"),
        (LangCode::Jpn, "jpn"),
        (LangCode::Jpn, "ja"),
        (LangCode::Eng, "ENG"),
        (LangCode::Eng, "Complex eng"),
    ],
    ["missing", "trash", "9325124"],
    @ok_compare
);

#[test]
fn test_to_string() {
    assert_eq!("eng", &LangCode::Eng.to_string());
    assert_eq!("rus", &LangCode::Rus.to_string());
    assert_eq!("jpn", &LangCode::Jpn.to_string());
}
