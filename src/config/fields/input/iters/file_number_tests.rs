use super::FileNumber;
use std::ffi::OsStr;

fn new<'a>(s: &'a str) -> FileNumber<'a> {
    FileNumber::new(OsStr::new(s))
}

#[test]
fn test_get_usize() {
    assert_eq!(123, new("x123y").get_usize().unwrap());
    assert_eq!(54, new("54abc").get_usize().unwrap());
}

#[test]
fn test_non_digit() {
    let num = new("abcdef");
    assert!(num.get_usize().is_none());
}

#[test]
fn test_zero_digit() {
    let num = new("x023y");
    assert_eq!(23, num.get_usize().unwrap());
}

#[test]
fn update() {
    let mut num = new("x023y");
    num.update(OsStr::new("a111b"));
    assert_eq!(111, num.get_usize().unwrap());
}

#[test]
fn update_multi_digit() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_2_with_add_123_and_4";

    let mut num = new(file_1);
    assert_eq!(1920, num.get_usize().unwrap());

    num.update(OsStr::new(file_2));
    assert_eq!(2, num.get_usize().unwrap());
}

#[test]
fn update_multi_digit_with_diff_len() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_2";

    let mut num = new(file_1);
    assert_eq!(1920, num.get_usize().unwrap());

    num.update(OsStr::new(file_2));
    assert_eq!(2, num.get_usize().unwrap());
}

#[test]
fn digits_only() {
    let mut num = new("111");
    assert_eq!(111, num.get_usize().unwrap());
    num.update(OsStr::new("222"));
    assert_eq!(222, num.get_usize().unwrap());
}
