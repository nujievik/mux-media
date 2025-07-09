use crate::common::*;
use mux_media::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[test]
fn test_normalize() {
    assert_eq!(new_dir(""), Input::try_normalize_dir(".").unwrap());
}

#[test]
fn test_default() {
    let input = Input::default();
    let dir = Path::new(".");

    assert_eq!(dir, input.get_dir());
    assert_eq!(dir, input.get_upmost());
}

fn new(args: &[&str]) -> Input {
    let mut input = cfg::<_, &&str>(args).get::<MCInput>().clone();
    input.try_finalize_init().unwrap();
    input
}

#[test]
fn test_collect_fonts() {
    let data_file = |s: &str| -> PathBuf {
        let s = format!("input/fonts/{}", s);
        data_file(&s)
    };

    [
        ("1", vec!["1/ttf.ttf"]),
        ("2", vec!["2/a b c.ttf", "2/other.ttf", "2/ttf.ttf"]),
        ("3", vec!["3/abc.tTf", "3/other.ttf", "3/ttf.TTF"]),
        (
            "4",
            vec![
                "4/ttf.ttf",
                "4/1/2/3/4/ttf.ttf",
                "4/1/2/3/4/5/6/7/8/9/10/ttf.ttf",
                "4/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/ttf.ttf",
                // ttf. in .../17/ttf.ttf not be collect because default
                // max_depth is 16
            ],
        ),
        ("5", vec!["5/otf.otf"]),
    ]
    .iter()
    .for_each(|(dir, files)| {
        let dir = data_file(dir);
        let s_dir = dir.to_str().unwrap();

        let input = new(&["-i", s_dir, "--up", "0"]);
        let files: HashSet<PathBuf> = files.iter().map(|f| data_file(f)).collect();

        let collected = input.collect_fonts();

        assert_eq!(
            files.len(),
            collected.len(),
            "Less or more fonts than expected collected from '{}'",
            dir.display()
        );

        collected.iter().for_each(|f| {
            assert!(
                files.contains(f),
                "Collected fonts from dir '{}' not contains '{}'",
                dir.display(),
                f.display()
            )
        });
    })
}

#[test]
fn test_iter_media_grouped_by_stem() {
    let data_file = |s: &str| -> PathBuf {
        let s = format!("input/{}", s);
        data_file(&s)
    };

    [
        ("1", vec![vec!["1/1.mka", "1/1.mks", "1/1.mkv", "1/1.srt"]]),
        (
            "2",
            vec![vec!["2/1.mkv", "2/1.srt"], vec!["2/2.mkv", "2/2.srt"]],
        ),
        ("3", vec![vec!["3/1.mkv", "3/1 suffix.srt"]]),
        ("4", vec![vec!["4/1.mkv", "4/1.SRT"]]),
        (
            "5",
            vec![vec![
                "5/prf.mkv",
                "5/1/2/3/prf.srt",
                "5/1/2/3/4/5/6/7/8/9/10/11/12/13/14/15/16/prf.srt",
            ]],
        ),
    ]
    .iter()
    .for_each(|(dir, files)| {
        let dir = data_file(dir);
        let s_dir = dir.to_str().unwrap();

        let input = new(&["-i", s_dir, "--up", "0"]);

        let files: Vec<Vec<PathBuf>> = files
            .iter()
            .map(|files| files.iter().map(|f| data_file(f)).collect())
            .collect();

        let mut cnt = 0;

        input.iter_media_grouped_by_stem().for_each(|media| {
            cnt += 1;
            assert!((cnt <= files.len()), "Exceeded expected num groups");

            assert!(
                files.iter().any(|f| f.len() == media.files.len()),
                "Mismatched len() collected files from '{}' by stem '{}'",
                dir.display(),
                AsRef::<Path>::as_ref(media.stem.as_ref()).display()
            );

            media.files.iter().for_each(|f| {
                assert!(
                    files.iter().any(|group| group.contains(f)),
                    "Collected media from dir '{}' by stem '{}' not contains '{}'",
                    dir.display(),
                    AsRef::<Path>::as_ref(media.stem.as_ref()).display(),
                    f.display()
                );
            })
        });

        assert!((cnt == files.len()), "Not found expected num groups");
    })
}
