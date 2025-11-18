use mux_media::MediaNumber;
use std::ffi::OsStr;

fn new(s: &str) -> MediaNumber {
    MediaNumber::from(OsStr::new(s))
}

#[test]
fn test_as_str() {
    assert_eq!("123", new("x123y").as_str());
    assert_eq!("54", new("54abc").as_str());
}

#[test]
fn test_to_usize() {
    assert_eq!(123, new("x123y").to_usize());
    assert_eq!(54, new("54abc").to_usize());
}

#[test]
fn test_non_digit() {
    let num = new("abcdef");
    assert_eq!("", num.as_str());
    assert_eq!(0, num.to_usize());
}

#[test]
fn test_zero_digit() {
    let num = new("x023y");
    assert_eq!("023", num.as_str());
    assert_eq!(23, num.to_usize());
}

#[test]
fn test_multi_digit() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_2_with_add_123_and_4";

    let mut num = new(file_1);
    num.upd(OsStr::new(file_2));

    assert_eq!("2", num.as_str());
    assert_eq!(2, num.to_usize());
}
