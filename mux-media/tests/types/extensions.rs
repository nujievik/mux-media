use mux_media::EXTENSIONS;
use phf::Set;
use std::collections::HashSet;

fn case_permutations(s: &str) -> Vec<String> {
    fn helper(chars: &[char], current: String, result: &mut Vec<String>) {
        if chars.is_empty() {
            result.push(current);
        } else {
            let mut lower = current.clone();
            lower.push(chars[0].to_ascii_lowercase());
            helper(&chars[1..], lower, result);

            let mut upper = current;
            upper.push(chars[0].to_ascii_uppercase());
            helper(&chars[1..], upper, result);
        }
    }

    let mut result = Vec::new();
    helper(&s.chars().collect::<Vec<_>>(), String::new(), &mut result);
    result
}

fn assert_all_permutations_present(set: &'static Set<&'static [u8]>, ext: &str) {
    for ext in case_permutations(ext) {
        assert!(
            set.contains(ext.as_bytes()),
            "Set should contain variant: {}",
            ext
        );
    }
}

fn assert_all_permutations_absent(set: &'static Set<&'static [u8]>, ext: &str) {
    for ext in case_permutations(ext) {
        assert!(
            !set.contains(ext.as_bytes()),
            "Set should NOT contain variant: {}",
            ext
        );
    }
}

macro_rules! test_any_exts_contains {
    ($fn:ident, $ext:ident, $ext_list:expr) => {
        #[test]
        fn $fn() {
            $ext_list
                .iter()
                .for_each(|ext| assert_all_permutations_present(EXTENSIONS.$ext, ext))
        }
    };
}

test_any_exts_contains!(test_avi_contains, avi, ["avi"]);
test_any_exts_contains!(test_fonts_contains, fonts, ["otf", "ttf"]);
test_any_exts_contains!(
    test_matroska_contains,
    matroska,
    ["mka", "mks", "mkv", "webm"]
);
test_any_exts_contains!(
    test_media_contains,
    media,
    [
        "264", "265", "3gp", "aac", "ac3", "ass", "avi", "avc", "av1", "caf", "dts", "dtshd",
        "eac3", "ec3", "f4v", "flac", "flv", "h264", "h265", "hevc", "ivf", "m2ts", "m2v", "m4a",
        "m4v", "mka", "mks", "mlp", "mov", "mp2", "mp3", "mp4", "mpa", "mpg", "mpv", "mpeg", "ogg",
        "ogm", "ogv", "obu", "opus", "ra", "srt", "ssa", "sub", "sup", "thd", "truehd", "tta",
        "ts", "vc1", "wav", "weba", "webm", "webma", "wma", "wmv", "x264", "x265",
    ]
);
test_any_exts_contains!(test_mp4_contains, mp4, ["mp4"]);
test_any_exts_contains!(
    test_subs_contains,
    subs,
    ["ass", "mks", "srt", "ssa", "sub", "sup"]
);
test_any_exts_contains!(test_webm_contains, webm, ["webm"]);

#[test]
fn test_media_absent() {
    for ext in EXTENSIONS.fonts.iter() {
        assert!(!EXTENSIONS.media.contains(ext));
    }
}

#[test]
fn test_extensions_absent() {
    let fake_exts = [
        "fake", "none", "xyz", "audiox", "v1deo", "123", "supper", "trackin", "ext", "subtitle",
    ];

    for set in [
        EXTENSIONS.avi,
        EXTENSIONS.fonts,
        EXTENSIONS.matroska,
        EXTENSIONS.media,
        EXTENSIONS.mp4,
        EXTENSIONS.subs,
        EXTENSIONS.webm,
    ] {
        fake_exts.iter().for_each(|ext| {
            assert_all_permutations_absent(set, ext);
        })
    }
}

fn generate_fake_exts(existing: HashSet<String>, count: usize) -> Vec<String> {
    use rand::{Rng, seq::IteratorRandom};
    use std::collections::HashSet;

    let mut rng = rand::rng();
    let charset = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut fake_exts = HashSet::new();
    let mut attempts = 0;

    while fake_exts.len() < count && attempts < 10000 {
        attempts += 1;

        let len = rng.random_range(3..6);
        let candidate: String = (0..len)
            .map(|_| *charset.iter().choose(&mut rng).unwrap() as char)
            .collect();

        if !existing.contains(candidate.as_str()) {
            fake_exts.insert(candidate);
        }
    }

    fake_exts.into_iter().collect()
}

#[test]
fn test_extensions_auto_absent() {
    let all_known: HashSet<String> = EXTENSIONS
        .media
        .iter()
        .chain(EXTENSIONS.fonts.iter())
        .filter_map(|bytes| Some(String::from_utf8_lossy(bytes).to_string()))
        .collect();

    let fake_exts = generate_fake_exts(all_known, 1000);

    for set in [
        EXTENSIONS.avi,
        EXTENSIONS.fonts,
        EXTENSIONS.matroska,
        EXTENSIONS.media,
        EXTENSIONS.mp4,
        EXTENSIONS.subs,
        EXTENSIONS.webm,
    ] {
        for ext in &fake_exts {
            assert_all_permutations_absent(set, &ext);
        }
    }
}
