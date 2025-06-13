use mux_media::{MaxValue, Range, ToMkvmergeArg};
use std::str::FromStr;

const MAX: u32 = u32::MAX;

pub fn new(s: &str) -> Range<u32> {
    s.parse::<Range<u32>>().expect(&format!("Fail range from '{}'", s))
}

#[test]
fn test_max_value() {
    assert_eq!(MAX, <u32 as MaxValue>::MAX);
}

#[test]
fn test_empty_str() {
    let range = new("");
    assert_eq!(0, range.start);
    assert_eq!(u32::MAX, range.end);
}

crate::test_from_str!(
    Range<u32>, test_from_str,
    ["", "5", "0", " 10 ", "5,10", "5,", ",10", "5-10", "5-", "-10", "5..10"],
    ["a,10", "5,b", "5,10,15", "5-10-15", "5.10", "10,5"]
);

#[test]
fn test_expected_start_end() {
    let cases = [
        ("", (0, MAX)),
        (",", (0, MAX)),
        ("5", (5, MAX)),
        ("0", (0, MAX)),
        (" 10 ", (10, MAX)),
        ("5,10", (5, 10)),
        ("5,", (5, MAX)),
        (",10", (0, 10)),
        ("5-10", (5, 10)),
        ("5-", (5, MAX)),
        ("-10", (0, 10)),
        ("6-6", (6, 6)),
        ("3..7", (3, 7)),
    ];

    for (s, (min, max)) in cases {
        let range = new(s);
        assert_eq!(min, range.start);
        assert_eq!(max, range.end);
    }
}

#[test]
fn test_iter() {
    let cases = [
        ("5-10", (5, 10)),
        ("-10", (0, 10)),
        ("6-6", (6, 6)),
        ("3-7", (3, 7)),
        ("-128", (0, 128)),
        ("64-128", (64, 128)),
    ];

    for (s, (min, max)) in cases {
        let mut iter = new(s).iter();
        for x in min..=max {
            let i = iter.next().expect(&format!(
                "None iter.next() for '{}', on std iter '{}'",
                s, x
            ));
            assert_eq!(x, i);
        }
        assert!(
            iter.next().is_none(),
            "Some iter.next() for '{}', after end std iter",
            s
        );
    }
}

#[test]
fn test_to_mkvmerge_arg() {
    let cases = [
        ("5-10", (5, 10)),
        ("-10", (0, 10)),
        ("6-6", (6, 6)),
        ("3-7", (3, 7)),
        ("-128", (0, 128)),
        ("64-128", (64, 128)),
    ];

    for (s, (min, max)) in cases {
        let arg = new(s).to_mkvmerge_arg();
        let expected: String = (min..=max)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(arg, expected);
    }
}

#[test]
fn test_contains() {
    let cases = [
        ("", [0, 8, MAX]),
        ("5-", [5, 8, MAX]),
        ("10-", [10, 16, MAX]),
        ("5-10", [5, 8, 10]),
        ("-10", [0, 8, 10]),
        ("6-6", [6, 6, 6]),
        ("3-7", [3, 4, 7]),
    ];

    for (s, vals) in cases {
        let range = new(s);
        for x in vals {
            assert!(
                range.contains(x),
                "Not contains '{}' in range from '{}'",
                x,
                s
            );
        }
    }
}

#[test]
fn test_not_contains() {
    let cases = [
        ("5-", [0, 2, 4]),
        ("10-", [0, 8, 9]),
        ("5-10", [0, 4, MAX]),
        ("-10", [11, 16, MAX]),
        ("6-6", [0, 5, 7]),
        ("3-7", [0, 2, 8]),
    ];

    for (s, vals) in cases {
        let range = new(s);
        for x in vals {
            assert!(!range.contains(x), "Contains '{}' in range from '{}'", x, s);
        }
    }
}

#[test]
fn test_contains_range() {
    let cases = [
        ("", ["", "5-8", "16-"]),
        ("5-", ["5-", "5-8", "16-"]),
        ("10-", ["10-", "16-32", "16-"]),
        ("5-10", ["5-10", "8-8", "8-10"]),
        ("-10", ["-10", "5-8", "8-10"]),
        ("6-6", ["6-6", "6-6", "6-6"]),
        ("3-7", ["3-7", "4-5", "5-7"]),
    ];

    for (s, vals) in cases {
        let range = new(s);
        for x in vals {
            let rng = new(x);
            assert!(
                range.contains_range(rng),
                "Not contains range '{}' in range from '{}'",
                x,
                s
            );
        }
    }
}

#[test]
fn test_not_contains_range() {
    let cases = [
        ("5-", ["", "0-4", "4-"]),
        ("10-", ["", "5-8", "9-"]),
        ("5-10", ["", "0-4", "5-"]),
        ("-10", ["", "5-12", "10-"]),
        ("6-6", ["", "0-5", "6-"]),
        ("3-7", ["", "0-2", "7-"]),
    ];

    for (s, vals) in cases {
        let range = new(s);
        for x in vals {
            let rng = new(x);
            assert!(
                !range.contains_range(rng),
                "Not contains range {} in range from {}",
                x,
                s
            );
        }
    }
}

#[test]
fn test_expected_err_messages() {
    let cases = [
        ("x", "invalid digit"),
        ("1-x", "invalid digit"),
        ("1,,8", "Too many ',' delimiters in input"),
        (
            "8-1",
            "End of range (1) must be greater than or equal to start (8)",
        ),
    ];

    for (s, expected_msg) in &cases {
        match Range::<u32>::from_str(s) {
            Err(e) => assert!(
                e.to_string().contains(expected_msg),
                "Expected error contains '{}' for '{}', but got '{}'",
                expected_msg,
                s,
                e.to_string(),
            ),
            Ok(_) => panic!("Expected error for '{}', but got Ok", s),
        }
    }
}
