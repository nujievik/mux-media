use crate::common::*;
use mux_media::markers::*;
use mux_media::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[test]
fn test_normalize() {
    assert_eq!(new_dir(""), Input::try_normalize_dir(".").unwrap());
}

fn new(args: &[&str]) -> Input {
    let mut input = cfg::<_, &&str>(args).field::<MCInput>().clone();
    input.try_finalize_init().unwrap();
    input
}

static TEST_INPUT_MEDIA: &[(&str, &[&[&str]])] = &[
    ("1", &[&["1/1.mka", "1/1.mks", "1/1.mkv", "1/1.srt"]]),
    ("2", &[&["2/1.mkv", "2/1.srt"], &["2/2.mkv", "2/2.srt"]]),
    ("3", &[&["3/1.mkv", "3/1 suffix.srt"]]),
    ("4", &[&["4/1.mkv", "4/1.SRT"]]),
    (
        "5",
        &[&[
            "5/prf.mkv",
            "5/1/2/3/prf.srt",
            "5/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/prf.srt",
        ]],
    ),
];

fn data_media(s: &str) -> PathBuf {
    let s = format!("input/{}", s);
    data(s)
}

fn iter_media_and_assert(input: &Input, dir: &PathBuf, expected: &Vec<Vec<PathBuf>>) {
    let mut cnt = 0;

    input.iter_media_grouped_by_stem().for_each(|media| {
        cnt += 1;
        assert!((cnt <= expected.len()), "Exceeded expected num groups");

        assert!(
            expected.iter().any(|f| f.len() == media.files.len()),
            "Mismatched len() collected expected from '{}' by stem '{}'",
            dir.display(),
            Path::new(&media.stem).display()
        );

        media.files.iter().for_each(|f| {
            assert!(
                expected.iter().any(|group| group.contains(f)),
                "Collected media from dir '{}' by stem '{}' not contains '{}'",
                dir.display(),
                Path::new(&media.stem).display(),
                f.display()
            );
        })
    });

    assert!((cnt == expected.len()), "Not found expected num groups");
}

#[test]
fn test_iter_media_grouped_by_stem() {
    TEST_INPUT_MEDIA.iter().for_each(|(dir, files)| {
        let dir = data_media(dir);
        let s_dir = dir.to_str().unwrap();

        let input = new(&["-i", s_dir]);

        let expected: Vec<Vec<PathBuf>> = files
            .iter()
            .map(|files| files.iter().map(|f| data_media(f)).collect())
            .collect();

        iter_media_and_assert(&input, &dir, &expected);
    })
}

#[test]
fn test_skip_media() {
    let skip_patterns: &[(&str, &[&str])] = &[("*.mka", &[".mka"]), ("*.mks", &[".mks"])];

    skip_patterns.iter().for_each(|(skip, not_contains)| {
        TEST_INPUT_MEDIA.iter().for_each(|(dir, files)| {
            let dir = data_media(dir);
            let s_dir = dir.to_str().unwrap();

            let input = new(&["-i", s_dir, "--skip", skip]);

            let expected: Vec<Vec<PathBuf>> = files
                .iter()
                .map(|files| {
                    files
                        .iter()
                        .filter(|f| !not_contains.iter().any(|ext| f.contains(ext)))
                        .map(|f| data_media(f))
                        .collect()
                })
                .collect();

            iter_media_and_assert(&input, &dir, &expected);
        })
    })
}

static TEST_INPUT_FONTS: &[(&str, &[&str])] = &[
    ("1", &["1/ttf.ttf"]),
    ("2", &["2/a b c.ttf", "2/other.ttf", "2/ttf.ttf"]),
    ("3", &["3/abc.tTf", "3/other.ttf", "3/ttf.TTF"]),
    ("4", &["4/otf.otf"]),
    (
        "5",
        &[
            "5/ttf.ttf",
            "5/1/2/3/4/ttf.ttf",
            "5/1/2/3/4/5/6/7/8/9/10/ttf.ttf",
            "5/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/ttf.ttf",
            // ttf. in .../17/ttf.ttf not be collect by default because default
            // max_depth is 16
        ],
    ),
];

fn data_font(s: &str) -> PathBuf {
    let s = format!("input/fonts/{}", s);
    data(s)
}

fn collect_fonts_and_assert_eq(input: &Input, dir: &PathBuf, expected: &HashSet<PathBuf>) {
    let collected = input.collect_fonts();

    assert_eq!(
        expected.len(),
        collected.len(),
        "Less or more fonts than expected collected from '{}'",
        dir.display()
    );

    collected.iter().for_each(|f| {
        assert!(
            expected.contains(f),
            "Collected fonts from dir '{}' not contains '{}'",
            dir.display(),
            f.display()
        )
    });
}

#[test]
fn test_collect_fonts() {
    TEST_INPUT_FONTS.iter().for_each(|(dir, files)| {
        let dir = data_font(dir);
        let s_dir = dir.to_str().unwrap();

        let input = new(&["-i", s_dir]);
        let expected: HashSet<PathBuf> = files.iter().map(|f| data_font(f)).collect();

        collect_fonts_and_assert_eq(&input, &dir, &expected);
    })
}

#[test]
fn test_skip_fonts() {
    let skip_patterns: &[(&str, &[&str])] = &[
        ("*.ttf", &[".ttf"]),
        ("*.TTF", &[".TTF"]),
        ("*.otf", &[".otf"]),
        ("*.tTf,*.TTF", &[".tTf", ".TTF"]),
    ];

    skip_patterns.iter().for_each(|(skip, not_contains)| {
        TEST_INPUT_FONTS.iter().for_each(|(dir, files)| {
            let dir = data_font(dir);
            let s_dir = dir.to_str().unwrap();

            let input = new(&["-i", s_dir, "--skip", skip]);

            let expected: HashSet<PathBuf> = files
                .iter()
                .filter(|f| !not_contains.iter().any(|ext| f.contains(ext)))
                .map(|f| data_font(f))
                .collect();

            collect_fonts_and_assert_eq(&input, &dir, &expected);
        })
    })
}

#[test]
fn test_depth() {
    let dir = data_font("5/");
    let s_dir = dir.to_str().unwrap();

    [(1, "0"), (2, "4"), (3, "10"), (4, "16"), (5, "17")]
        .into_iter()
        .for_each(|(expected_len, depth)| {
            let input = new(&["-i", s_dir, "--depth", depth]);
            assert_eq!(expected_len, input.collect_fonts().len());
        })
}
