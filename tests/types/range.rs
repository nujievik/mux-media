use mux_media::RangeUsizeInclusive;

const MAX: usize = usize::MAX;

pub fn new(s: &str) -> RangeUsizeInclusive {
    s.parse::<RangeUsizeInclusive>()
        .expect(&format!("Fail range from '{}'", s))
}

#[test]
fn test_full() {
    let range = new("-");
    assert_eq!(0, range.start());
    assert_eq!(MAX, range.end());
}

crate::test_from_str!(
    from_str, RangeUsizeInclusive;
    [
        "-", "5", "0", "5-10", "5-", "-10",
    ];
    ["a-10", "5-b", "5-10-15", "5-10-15", "10-5"]
);

#[test]
fn test_expected_start_end() {
    let cases = [
        ("-", (0, MAX)),
        ("5", (5, MAX)),
        ("0", (0, MAX)),
        ("10", (10, MAX)),
        ("5-10", (5, 10)),
        ("-10", (0, 10)),
        ("5-", (5, MAX)),
        ("6-6", (6, 6)),
        ("3-7", (3, 7)),
    ];

    for (s, (min, max)) in cases {
        let range = new(s);
        assert_eq!(min, range.start());
        assert_eq!(max, range.end());
    }
}

#[test]
fn test_contains() {
    [
        ("-", [0, 8, MAX]),
        ("5-", [5, 8, MAX]),
        ("10-", [10, 16, MAX]),
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
