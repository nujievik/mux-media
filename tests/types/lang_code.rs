use mux_media::{IsDefault, LangCode, MuxError};

#[test]
fn test_is_default() {
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

fn try_slice(slice: &[&str]) -> Result<LangCode, MuxError> {
    let vec: Vec<String> = slice.into_iter().map(|s| s.to_string()).collect();
    LangCode::try_from(vec.as_slice())
}

fn slice(slice: &[&str]) -> LangCode {
    try_slice(slice).expect(&format!("Fail LangCode from slice '{:?}'", slice))
}

#[test]
fn test_from_slice_string() {
    assert!(LangCode::Eng == slice(&["trash", "missing", "eng"]));
    assert!(try_slice(&["trash", "missing"]).is_err());
}

#[test]
fn test_multiple_ok_slice() {
    assert!(LangCode::Eng == slice(&["und", "alb", "alu", "eng", "kud"]));
    assert!(LangCode::Rus == slice(&["und", "rus", "alu", "eng", "kud"]));
}
