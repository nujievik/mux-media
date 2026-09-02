use clap::error::ErrorKind;
use mux_media::*;

#[test]
fn test_from_clap_error() {
    let msg = "Test clap message\n";
    let clap_err = clap::Error::raw(ErrorKind::InvalidValue, msg);
    let err: MuxError = clap_err.into();

    // clap::Error return code 2 on error
    assert_eq!(2, err.code());

    match err {
        MuxError::ConfigParse(_) => (),
        _ => panic!("Must be MuxError::ConfigParse"),
    }
}
