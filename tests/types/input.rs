use crate::common::*;
use mux_media::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

fn new(args: &[&Path]) -> Input {
    let mut input = cfg::<_, &&Path>(args).input;
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
        let input = new(&[p("-i"), &dir]);

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
            let input = new(&[p("-i"), &dir, p("--skip"), p(skip)]);

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

pub static TEST_INPUT_FONTS: &[(&str, &[&str])] = &[
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

pub fn data_font(s: &str) -> PathBuf {
    let s = format!("input/fonts/{}", s);
    data(s)
}

fn collect_fonts_and_assert_eq(input: &Input, expected: &[&str]) {
    let expected: HashSet<_> = expected.iter().map(|f| data_font(f)).collect();
    let collected = HashSet::from_iter(input.collect_fonts());
    assert_eq!(expected, collected);
}

#[test]
fn test_collect_fonts() {
    TEST_INPUT_FONTS.iter().for_each(|(dir, files)| {
        let dir = data_font(dir);
        let input = new(&[p("-i"), &dir]);
        collect_fonts_and_assert_eq(&input, files);
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
            let input = new(&[p("-i"), &dir, p("--skip"), p(skip)]);

            let expected: Vec<_> = files
                .iter()
                .filter(|f| !not_contains.iter().any(|ext| f.contains(ext)))
                .map(|s| *s)
                .collect();

            collect_fonts_and_assert_eq(&input, &expected);
        })
    })
}

fn body_sort(dir: &str, order: &[&str]) {
    let dir = data_font(dir);
    let input = new(&[p("-i"), &dir]);
    let fonts = input.collect_fonts_with_filter_and_sort();
    fonts.iter().enumerate().for_each(|(i, f)| {
        assert_eq!(order[i], f.file_stem().unwrap());
    })
}

#[test]
fn test_fonts_sort_depth_independent() {
    body_sort("sort/depth_independent/", &["first", "second", "third"]);
}

#[test]
fn test_fonts_sort_case_insensitive() {
    body_sort("sort/case_insensitive/", &["first", "Second", "THIRD"]);
}

#[test]
fn test_depth() {
    let dir = data_font("5/");

    [(1, "0"), (2, "4"), (3, "10"), (4, "16"), (5, "17")]
        .into_iter()
        .for_each(|(expected_len, depth)| {
            let input = new(&[p("-i"), &dir, p("--depth"), p(depth)]);
            assert_eq!(expected_len, input.collect_fonts().len());
        })
}

#[test]
fn test_solo() {
    let dir = data_media("solo/");
    let input = new(&[p("-i"), &dir]);
    assert_eq!(None, input.iter_media_grouped_by_stem().next());

    let input = new(&[p("-i"), &dir, p("--solo")]);
    let exp = vec![dir.join("srt.srt")];
    assert_eq!(
        exp,
        input.iter_media_grouped_by_stem().next().unwrap().files
    );
}
