use mux_media::RangeUsize;

const MAX: usize = usize::MAX;

pub fn new(s: &str) -> RangeUsize {
    s.parse::<RangeUsize>()
        .expect(&format!("Fail range from '{}'", s))
}

#[test]
fn test_empty_str() {
    let range = new("");
    assert_eq!(0, range.start);
    assert_eq!(MAX, range.end);
}

crate::test_from_str!(
    RangeUsize,
    test_from_str,
    [
        "", "5", "0", " 10 ", "5,10", "5,", ",10", "5-10", "5-", "-10", "5..=10"
    ],
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
        ("5,10", (5, 11)),
        ("5,", (5, MAX)),
        (",10", (0, 11)),
        ("5-10", (5, 11)),
        ("5-", (5, MAX)),
        ("-10", (0, 11)),
        ("6-6", (6, 7)),
        ("3..=7", (3, 8)),
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
        let mut iter = new(s).into_iter();
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
fn test_to_string() {
    [
        ("5-10", (5, 10)),
        ("-10", (0, 10)),
        ("6-6", (6, 6)),
        ("3-7", (3, 7)),
        ("-128", (0, 128)),
        ("64-128", (64, 128)),
    ]
    .iter()
    .for_each(|(s, (start, end))| {
        let exp = format!("{}-{}", start, end);
        let rng = s.parse::<RangeUsize>().unwrap();
        assert_eq!(exp, rng.to_string());
    })
}

#[test]
fn test_contains() {
    [
        ("", [0, 8, MAX - 1]),
        ("5-", [5, 8, MAX - 1]),
        ("10-", [10, 16, MAX - 1]),
        ("5-10", [5, 8, 10]),
        ("-10", [0, 8, 10]),
        ("6-6", [6, 6, 6]),
        ("3-7", [3, 4, 7]),
    ]
    .iter()
    .for_each(|(s, vals)| {
        let range = new(s);
        vals.iter().for_each(|v| {
            assert!(
                range.contains(v),
                "Not contains '{}' in range from '{}'",
                v,
                s
            );
        })
    })
}

#[test]
fn test_not_contains() {
    [
        ("5-", [0, 2, 4]),
        ("10-", [0, 8, 9]),
        ("5-10", [0, 4, MAX]),
        ("-10", [11, 16, MAX]),
        ("6-6", [0, 5, 7]),
        ("3-7", [0, 2, 8]),
    ]
    .iter()
    .for_each(|(s, vals)| {
        let range = new(s);
        vals.iter().for_each(|v| {
            assert!(!range.contains(v), "Contains '{}' in range from '{}'", v, s);
        })
    })
}

#[test]
fn test_expected_err_messages() {
    [
        ("x", "invalid digit"),
        ("1-x", "invalid digit"),
        ("1,,8", "Too many ',' delimiters in input"),
        (
            "8-1",
            "End of range (1) must be greater than or equal to start (8)",
        ),
    ]
    .iter()
    .for_each(|(s, expected_msg)| match s.parse::<RangeUsize>() {
        Err(e) => assert!(
            e.to_string().contains(expected_msg),
            "Expected error contains '{}' for '{}', but got '{}'",
            expected_msg,
            s,
            e.to_string(),
        ),
        Ok(_) => panic!("Expected error for '{}', but got Ok", s),
    })
}
