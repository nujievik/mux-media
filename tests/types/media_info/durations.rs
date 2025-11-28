use super::new;
use crate::common::*;
use mux_media::{markers::*, *};

#[test]
fn test_audio_duration() {
    let mut mi = new();
    let d = mi.try_get(MIAudioDuration, &data("audio_x1.mka"));
    assert_eq!(*d.unwrap(), Duration::new(0, 1_012_960_950));
    mi.try_get(MIAudioDuration, &data("video_x1.mkv"))
        .unwrap_err();
}

#[test]
fn test_video_duration() {
    let mut mi = new();
    let d = mi.try_take(MIVideoDuration, &data("video_x1.mkv"));
    assert_eq!(d.unwrap(), Duration::new(0, 1_000_000_000));
    mi.try_get(MIVideoDuration, &data("audio_x1.mka"))
        .unwrap_err();
}

#[test]
fn test_playable_duration() {
    let mut mi = new();
    assert_eq(&mut mi, "video_x1.mkv", 0, 1_000_000_000);
    assert_eq(&mut mi, "audio_x1.mka", 0, 1_012_960_950);
    assert_eq(&mut mi, "vid_1s_and_srt_5s.mkv", 0, 1_000_000_000);
    assert_eq(&mut mi, "vid_1s_and_aud_1.013s.mkv", 0, 1_013_000_000);

    fn assert_eq(mi: &mut MediaInfo, f: &str, secs: u64, nanos: u32) {
        let d = mi.try_get(MIPlayableDuration, &data(f)).copied();
        assert_eq!(d.unwrap(), Duration::new(secs, nanos));
    }
}
