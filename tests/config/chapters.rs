use super::*;

#[test]
fn test_is_default() {
    use is_default::IsDefault;
    let mut cs = ConfigChapters::default();
    assert!(cs.is_default());
    cs.no_flag = true;
    assert!(!cs.is_default());
}

build_test_to_args!(
    test_to_args, "chapters";
    vec!["--no-chapters"],
    vec!["--chapters", "NAME:00:00:00.000-00:05:00.000,other:00:05:00.000-00:10:00.000"],
);
