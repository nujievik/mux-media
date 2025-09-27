use mux_media::*;

#[test]
fn test_dashed_undashed() {
    assert_eq!("--input", CliArg::Input.dashed());
    assert_eq!("--input", dashed!(Input));
    assert_eq!("input", CliArg::Input.undashed());
    assert_eq!("input", undashed!(Input));
}
