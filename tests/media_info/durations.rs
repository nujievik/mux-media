use super::*;

#[test]
fn audio_durations() {
    let mut mi = new();
    let xs = mi
        .try_get(MarkMediaInfoVideoAndAudioDurations, &data("audio_x1.mka"))
        .unwrap();

    const EXPECTED: Time = Time::from_millis(1013);
    assert_eq!(EXPECTED, xs.get_audio().unwrap());
    assert_eq!(EXPECTED, xs.max());
    assert_eq!(None, xs.get_video());
}

#[test]
fn video_durations() {
    let mut mi = new();
    let xs = mi
        .try_get(MarkMediaInfoVideoAndAudioDurations, &data("video_x1.mkv"))
        .unwrap();

    const EXPECTED: Time = Time::from_millis(1000);
    assert_eq!(EXPECTED, xs.get_video().unwrap());
    assert_eq!(EXPECTED, xs.max());
    assert_eq!(None, xs.get_audio());
}

#[test]
fn max_durations() {
    let set = [
        ("video_x1.mkv", 1000),
        ("audio_x1.mka", 1013),
        ("vid_1s_and_srt_5s.mkv", 1000),
        ("vid_1s_and_aud_1.013s.mkv", 1013),
    ];

    let mut mi = new();

    for (file, millis) in set {
        let file = data(file);
        let xs = mi
            .try_take(MarkMediaInfoVideoAndAudioDurations, &file)
            .unwrap();

        assert_eq!(xs.max(), Time::from_millis(millis));
        // eq directly
        assert_eq!(
            xs,
            VideoAndAudioDurations::try_from_media_file(&mut mi, &file).unwrap()
        );
    }
}
