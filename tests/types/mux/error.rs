use clap::error::ErrorKind;
use mux_media::MuxError;

#[test]
fn test_new() {
    let err = MuxError::new();
    assert_eq!(1, err.code);
    assert_eq!("", format!("{}", err));
}

#[test]
fn test_new_ok() {
    let err = MuxError::new_ok();
    assert_eq!(0, err.code);
    assert_eq!("", format!("{}", err));
}

#[test]
fn test_new_with_message() {
    let err = MuxError::new().message("msg");
    assert_eq!("msg", format!("{}", err));
}

#[test]
fn test_from_string() {
    let err: MuxError = String::from("msg").into();
    assert_eq!("msg", format!("{}", err));
    assert_eq!(1, err.code);
}

#[test]
fn test_from_str() {
    let err: MuxError = "msg".into();
    assert_eq!("msg", format!("{}", err));
    assert_eq!(1, err.code);
}

#[test]
fn test_from_any_error() {
    use std::io;
    let io_error = io::Error::new(io::ErrorKind::Other, "IO error");
    let err = MuxError::from_any(io_error);
    assert_eq!("IO error", format!("{}", err));
    assert_eq!(1, err.code);
}

#[test]
fn test_use_stderr() {
    assert!(MuxError::new().use_stderr());
    assert!(MuxError::default().use_stderr());
    assert!(!MuxError::new_ok().use_stderr());
    assert!(!MuxError::new().code(0).use_stderr());
}

#[test]
fn test_print_to_stdout() {
    let err = MuxError::new().code(0).message("Test success message");
    // We won't assert output stream here, but this ensures it doesn't panic
    err.print();
}

#[test]
fn test_print_to_stderr() {
    let err = MuxError::new().code(1).message("Test error message");
    // Again, just calling for coverage; eprintln will write to stderr
    err.print();
}

#[test]
fn test_from_clap_error() {
    let clap_err = clap::Error::raw(
        ErrorKind::InvalidValue,
        "Test clap message. It's Ok if you see it with prefix 'error: Test clap message'\n",
    );
    let mux_err: MuxError = clap_err.into();
    // clap::Error return code 2 on error
    assert_eq!(mux_err.code, 2);
}

#[test]
fn test_into_clap_error() {
    let mux_err = MuxError::new().message("Test clap message");
    let clap_err: clap::Error = mux_err.into();
    assert_eq!(clap_err.kind(), ErrorKind::InvalidValue);
    assert!(clap_err.to_string().contains("Test clap message"));
}
