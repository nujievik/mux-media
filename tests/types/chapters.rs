use crate::*;
use mux_media::*;

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    let mut cs = Chapters::default();
    assert!(cs.is_default());
    cs.no_flag = true;
    assert!(!cs.is_default());
}

build_test_to_json_args!(
    test_to_json_args, chapters, "chapters";
    vec!["--no-chapters"],
);
