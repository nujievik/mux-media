use crate::common::*;
use mux_media::*;
use once_cell::sync::Lazy;
use std::ffi::OsString;

static MUX_CONFIG: Lazy<MuxConfig> = Lazy::new(|| cfg::<_, &str>([]));

fn new() -> MediaInfo<'static> {
    MediaInfo::from(&*MUX_CONFIG)
}

#[test]
fn test_empty() {
    let mi = new();
    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_upd_cmn_stem() {
    let mut mi = new();
    mi.upd_cmn_stem(OsString::from("x"));
    assert_eq!(0, mi.len());
    assert!(!mi.is_empty());
    assert!(mi.is_no_files());
    assert_eq!("x", mi.try_get_cmn::<MICmnStem>().unwrap());
}

#[test]
fn test_try_insert() {
    let mut mi = new();
    let mut len = 0;
    for x in ["srt.srt", "audio_x1.mka", "video_x8.mkv"] {
        len += 1;
        mi.try_insert(data_file(x)).unwrap();
        assert_eq!(len, mi.len());
        assert!(!mi.is_empty());
        assert!(!mi.is_no_files());
    }

    mi.clear();
    for x in ["missing", "bad"] {
        assert!(mi.try_insert(x).is_err());
    }
    assert_eq!(0, mi.len());
}

#[test]
fn test_clear() {
    let mut mi = new();
    mi.upd_cmn_stem(OsString::from("x"));
    mi.try_insert(data_file("srt.srt")).unwrap();
    mi.clear();

    assert_eq!(0, mi.len());
    assert!(mi.is_empty());
    assert!(mi.is_no_files());
}

#[test]
fn test_get_take_upd_cache() {
    let mut mi = new();
    assert!(mi.get_cache().of_files.is_empty());

    for x in ["srt.srt", "audio_x1.mka"] {
        let file = data_file(x);
        mi.try_insert(&file).unwrap();

        let cache = mi.get_cache();
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        mi.unmut_get::<MIMkvmergeI>(&file).unwrap();

        let cache = mi.take_cache();
        assert_eq!(&file, cache.of_files.keys().next().unwrap());
        assert!(mi.unmut_get::<MIMkvmergeI>(&file).is_none());

        mi.upd_cache(cache);
        assert_eq!(&file, mi.get_cache().of_files.keys().next().unwrap());
        mi.unmut_get::<MIMkvmergeI>(&file).unwrap();

        mi.clear()
    }
}

#[test]
fn test_try_insert_with_filter() {
    let paths = [data_file("srt.srt"), data_file("audio_x1.mka")];

    for (arg, len) in [("-D", 2), ("-SA", 0)] {
        let mc = cfg([arg]);
        let mut mi = MediaInfo::from(&mc);
        mi.try_insert_paths_with_filter(&paths, true).unwrap();
        assert_eq!(len, mi.len());
    }

    let mut mi = new();
    let bad_paths = [data_file("missing"), data_file("bad")];
    for exit_on_err in [true, false] {
        assert_eq!(
            exit_on_err,
            mi.try_insert_paths_with_filter(&bad_paths, exit_on_err)
                .is_err()
        );
    }
    assert_eq!(0, mi.len());
}

#[test]
fn test_cmn_stem() {
    let mut mi = new();

    [("srt", "srt.srt"), ("audio_x1", "audio_x1.mka")]
        .into_iter()
        .for_each(|(stem, file)| {
            mi.try_insert(data_file(file)).unwrap();
            assert_eq!(stem, mi.try_get_cmn::<MICmnStem>().unwrap());
            mi.clear();
        });

    [
        "x1_set/x1_set.mkv",
        "x1_set/audio/x1_set.[audio].mka",
        "x1_set/subs/x1_set.[subs].mks",
    ]
    .into_iter()
    .for_each(|x| mi.try_insert(data_file(x)).unwrap());
    assert_eq!("x1_set", mi.try_get_cmn::<MICmnStem>().unwrap());
}
