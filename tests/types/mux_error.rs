use clap::error::ErrorKind;
use mux_media::*;

#[test]
fn test_default() {
    let err = MuxError::default();
    assert_eq!("", format!("{}", err));
    assert_eq!(1, err.code);
    assert_eq!(MuxErrorKind::Unknown, err.kind);
}

#[test]
fn test_new() {
    assert_eq!(MuxError::default(), MuxError::new());
}

#[test]
fn test_new_ok() {
    let err = MuxError::new_ok();
    assert_eq!("", format!("{}", err));
    assert_eq!(0, err.code);
    assert_eq!(MuxErrorKind::Ok, err.kind);
}

#[test]
fn test_new_message() {
    let err = MuxError::new().message("msg");
    assert_eq!("msg", format!("{}", err));
}

#[test]
fn test_new_code() {
    let err = MuxError::new().code(16);
    assert_eq!(16, err.code);
}

#[test]
fn test_new_kind() {
    let err = MuxError::new().kind(MuxErrorKind::InvalidValue);
    assert_eq!(MuxErrorKind::InvalidValue, err.kind);
}

#[test]
fn test_from_any_error() {
    use std::io;
    let io_error = io::Error::new(io::ErrorKind::Other, "IO error");
    let err = MuxError::from_any(io_error);
    assert_eq!("IO error", format!("{}", err));
    assert_eq!(1, err.code);
    assert_eq!(MuxErrorKind::Unknown, err.kind);
}

#[test]
fn test_use_stderr() {
    assert!(MuxError::new().use_stderr());
    assert!(MuxError::new().code(16).use_stderr());
    assert!(!MuxError::new_ok().use_stderr());
    assert!(!MuxError::new().code(0).use_stderr());
}

#[test]
fn test_print() {
    let err = MuxError::new().code(0).message("Test success message");
    err.print();
    let err = MuxError::new().code(1).message("Test error message");
    err.print();
}

#[test]
fn test_print_localized() {
    let mut err: MuxError = Msg::Using.into();
    err.print_localized();
    err.code = 0;
    err.print_localized();
}

#[test]
fn test_from_string() {
    let err: MuxError = String::from("msg").into();
    assert_eq!("msg", format!("{}", err));
    assert_eq!(1, err.code);
    assert_eq!(MuxErrorKind::Unknown, err.kind);
}

#[test]
fn test_from_str() {
    let err: MuxError = "msg".into();
    assert_eq!("msg", format!("{}", err));
    assert_eq!(1, err.code);
    assert_eq!(MuxErrorKind::Unknown, err.kind);
}

#[test]
fn test_from_clap_error() {
    let msg = "Test clap message. It's Ok if you see it with prefix 'error: Test clap message'\n";
    let clap_err = clap::Error::raw(ErrorKind::InvalidValue, msg);
    let err: MuxError = clap_err.into();

    // From<clap::Error> immediately prints a message, not moves
    assert_eq!("", format!("{}", err));
    // clap::Error return code 2 on error
    assert_eq!(2, err.code);
    assert_eq!(MuxErrorKind::Clap, err.kind);
}

#[test]
fn test_into_clap_error() {
    let mux_err = MuxError::new().message("Test clap message");
    let clap_err: clap::Error = mux_err.into();
    assert_eq!(clap_err.kind(), ErrorKind::InvalidValue);
    assert!(clap_err.to_string().contains("Test clap message"));
}
