use super::*;

#[test]
fn test_audio_duration() {
    let mut mi = new();
    let d = mi.try_get(MarkMediaInfoAudioDuration, &data("audio_x1.mka"));
    assert_eq!(*d.unwrap(), Time::from_millis(1013));
    mi.try_get(MarkMediaInfoAudioDuration, &data("video_x1.mkv"))
        .unwrap_err();
}

#[test]
fn test_video_duration() {
    let mut mi = new();
    let d = mi.try_take(MarkMediaInfoVideoDuration, &data("video_x1.mkv"));
    assert_eq!(d.unwrap(), Time::from_millis(1000));
    mi.try_get(MarkMediaInfoVideoDuration, &data("audio_x1.mka"))
        .unwrap_err();
}

#[test]
fn test_playable_duration() {
    let set = [
        ("video_x1.mkv", 1000),
        ("audio_x1.mka", 1013),
        ("vid_1s_and_srt_5s.mkv", 1000),
        ("vid_1s_and_aud_1.013s.mkv", 1013),
    ];

    let mut mi = new();

    for (file, millis) in set {
        let d = *mi
            .try_get(MarkMediaInfoPlayableDuration, &data(file))
            .unwrap();
        assert_eq!(d, Time::from_millis(millis));
    }
}
