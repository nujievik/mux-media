use clap::error::ErrorKind;
use mux_media::MuxError;

#[test]
fn test_new_error_default_code() {
    let err = MuxError::new();
    assert_eq!(err.code, 1);
    assert!(err.message.is_none());
    assert_eq!(format!("{}", err), "");
}

#[test]
fn test_ok_error() {
    let err = MuxError::ok();
    assert_eq!(err.code, 0);
    assert!(err.message.is_none());
    assert_eq!(format!("{}", err), "");
}

#[test]
fn test_error_with_message() {
    let err = MuxError::new().message("Something went wrong");
    assert_eq!(err.message.as_deref(), Some("Something went wrong"));
    assert_eq!(format!("{}", err), "Something went wrong");
}

#[test]
fn test_from_string() {
    let err: MuxError = String::from("Error from string").into();
    assert_eq!(err.message.as_deref(), Some("Error from string"));
    assert_eq!(err.code, 1);
}

#[test]
fn test_from_str() {
    let err: MuxError = "Error from str".into();
    assert_eq!(err.message.as_deref(), Some("Error from str"));
    assert_eq!(err.code, 1);
}

#[test]
fn test_from_any_error() {
    use std::io;
    let io_error = io::Error::new(io::ErrorKind::Other, "IO error");
    let err = MuxError::from_any(io_error);
    assert_eq!(err.message.unwrap(), "IO error");
    assert_eq!(err.code, 1);
}

#[test]
fn test_use_stderr_true() {
    let err = MuxError::new().code(1);
    assert!(err.use_stderr());
}

#[test]
fn test_use_stderr_false() {
    let err = MuxError::new().code(0);
    assert!(!err.use_stderr());
}

#[test]
fn test_print_to_stdout() {
    let err = MuxError::new().code(0).message("Success message");
    // We won't assert output stream here, but this ensures it doesn't panic
    err.print();
}

#[test]
fn test_print_to_stderr() {
    let err = MuxError::new().code(1).message("Error message");
    // Again, just calling for coverage; eprintln will write to stderr
    err.print();
}

#[test]
fn test_from_clap_error() {
    let clap_err = clap::Error::raw(ErrorKind::InvalidValue, "invalid value");
    let app_err: MuxError = clap_err.into();
    // clap::Error return code 2 on error
    assert_eq!(app_err.code, 2);
}

#[test]
fn test_into_clap_error() {
    let app_err = MuxError::new().message("conversion failed");
    let clap_err: clap::Error = app_err.into();
    assert_eq!(clap_err.kind(), ErrorKind::InvalidValue);
    assert!(clap_err.to_string().contains("conversion failed"));
}
