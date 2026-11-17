use super::FileNumber;
use std::ffi::OsStr;

fn new<'a>(s: &'a str) -> FileNumber<'a> {
    FileNumber::new(OsStr::new(s))
}

#[test]
fn get_number() {
    assert_eq!(123, new("x123y").get_number().unwrap());
    assert_eq!(54, new("54abc").get_number().unwrap());
}

#[test]
fn test_non_digit() {
    let mut num = new("abcdef");
    assert!(num.get_number().is_none());
}

#[test]
fn test_zero_digit() {
    let mut num = new("x023y");
    assert_eq!(23, num.get_number().unwrap());
}

#[test]
fn update() {
    let mut num = new("x023y");
    num.update(OsStr::new("a111b"));
    assert_eq!(111, num.get_number().unwrap());
    assert_ne!(num, new("a111b"));
}

#[test]
fn update_multi_digit() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_2_with_add_123_and_4";

    let mut num = new(file_1);
    assert_eq!(1920, num.get_number().unwrap());

    num.update(OsStr::new(file_2));
    assert_eq!(2, num.get_number().unwrap());

    assert_ne!(num, new(file_2));
}

#[test]
fn update_multi_digit_with_diff_len() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_2";

    let mut num = new(file_1);
    assert_eq!(1920, num.get_number().unwrap());

    num.update(OsStr::new(file_2));
    assert_eq!(2, num.get_number().unwrap());

    assert_ne!(num, new(file_2));
}

#[test]
fn digits_only() {
    let mut num = new("111");
    assert_eq!(111, num.get_number().unwrap());
    num.update(OsStr::new("222"));
    assert_eq!(222, num.get_number().unwrap());

    assert_ne!(num, new("222"));
}

#[test]
fn update_empty() {
    let mut num = new("");
    assert_eq!(None, num.get_number());

    num.update(OsStr::new("x123y"));
    assert_eq!(123, num.get_number().unwrap());
}

#[test]
fn update_diff_num_lens() {
    let mut num = new("x12y");
    assert_eq!(12, num.get_number().unwrap());
    num.update(OsStr::new("x333"));
    assert_eq!(333, num.get_number().unwrap());
}

#[test]
fn update_diff_num_lens_multi() {
    let file_1 = "1920x1080_file_1_with_add_123_and_4";
    let file_2 = "1920x1080_file_22_with_add_123_and_4";

    let mut num = new(file_1);
    assert_eq!(1920, num.get_number().unwrap());

    num.update(OsStr::new(file_2));
    assert_eq!(22, num.get_number().unwrap());
    assert_eq!(2, num.number_idx);
}

#[test]
fn update_shorter_number_at_the_end() {
    let mut num = new("1920x1080_file_10");
    assert_eq!(1920, num.get_number().unwrap());

    num.update(OsStr::new("1920x1080_file_2"));
    assert_eq!(2, num.get_number().unwrap());
    assert_eq!(2, num.number_idx);
}

#[test]
fn update_to_no_digits() {
    let mut num = new("file_123");
    assert_eq!(123, num.get_number().unwrap());

    num.update(OsStr::new("file_without_digits"));
    assert_eq!(None, num.get_number());
}

#[test]
fn overflow_usize_number() {
    let huge = format!("file_{}_end", "9".repeat(50));
    let mut num = new(&huge);
    assert_eq!(None, num.get_number());

    num.update(OsStr::new("file_42_end"));
    assert_eq!(42, num.get_number().unwrap());
}

#[test]
fn more_than_128_numbers_spill_to_heap() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    for i in 0..135 {
        s1.push_str(&format!("a{}_", i));
        if i == 132 {
            s2.push_str("a999_");
        } else {
            s2.push_str(&format!("a{}_", i));
        }
    }

    let mut num = new(&s1);
    assert_eq!(0, num.get_number().unwrap());

    num.update(OsStr::new(&s2));
    assert_eq!(999, num.get_number().unwrap());
    assert_eq!(132, num.number_idx);
}
