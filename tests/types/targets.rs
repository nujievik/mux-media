/*
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
        (TargetGroup::Subs, "s"),
        (TargetGroup::Subs, "subs"),
        (TargetGroup::Subs, "subtitles"),
    ],
    ["missing"],
    @ok_compare
);
*/
