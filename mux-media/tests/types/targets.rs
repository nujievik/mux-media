use mux_media::{TargetGroup, TrackType};

#[test]
fn test_from_track_type() {
    let cases = [
        (TargetGroup::Audio, TrackType::Audio),
        (TargetGroup::Subs, TrackType::Sub),
        (TargetGroup::Video, TrackType::Video),
    ];

    for (group, tt) in cases {
        assert_eq!(group, TargetGroup::try_from(tt).unwrap());
    }
}

crate::test_from_str!(
    TargetGroup, test_from_str,
    [
        (TargetGroup::Audio, "a"),
        (TargetGroup::Audio, "audio"),
        (TargetGroup::Video, "v"),
        (TargetGroup::Video, "video"),
        (TargetGroup::Signs, "signs"),
        (TargetGroup::Subs, "s"),
        (TargetGroup::Subs, "subs"),
        (TargetGroup::Subs, "subtitles"),
    ],
    ["missing"],
    @ok_compare
);

#[test]
fn test_try_signs() {
    let cases = [
        (true, "signs"),
        (true, "надписи"),
        (false, "missing"),
        (false, "s"),
    ];

    for (is_ok, s) in cases {
        let res = TargetGroup::try_signs(s);
        assert_eq!(is_ok, res.is_ok());
        if is_ok {
            assert!(TargetGroup::Signs == res.unwrap());
        }
    }
}

#[test]
fn try_signs_many() {
    let cases = [
        (true, ["signs".to_string(), "trash".to_string()]),
        (true, ["trash".to_string(), "signs".to_string()]),
        (false, ["trash".to_string(), "trash".to_string()]),
    ];

    for (is_ok, array) in cases {
        let res = TargetGroup::try_signs_many(array.as_slice());
        assert_eq!(is_ok, res.is_ok());
        if is_ok {
            assert!(TargetGroup::Signs == res.unwrap());
        }
    }
}
