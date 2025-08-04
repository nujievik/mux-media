use crate::common::*;
use mux_media::{markers::*, *};
use std::path::{Path, PathBuf};

fn body_test_empty(out: &Output, dir: PathBuf) {
    assert_eq!(false, out.need_num());
    assert_eq!(Path::new(""), out.temp_dir());

    ["1", "2", "abc", "n.am.e."].iter().for_each(|name| {
        let builded = out.build_out(name);
        assert_eq!(&dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", name)), builded);
    })
}

#[test]
fn test_empty() {
    let dir = new_dir("muxed/");

    let out = Output::try_from_path("").unwrap();
    body_test_empty(&out, dir.clone());

    let out = from_cfg::<MCOutput>(vec![]);
    body_test_empty(&out, dir);
}

pub fn new(args: &[&str]) -> Output {
    let mut out = cfg::<_, &&str>(args).field::<MCOutput>().clone();
    out.try_finalize_init().unwrap();
    out
}

fn body_test_dir_only(dir: &Path, out: &Output) {
    assert_eq!(false, out.need_num());
    assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

    ["1", "2", "abc", "n.am.e."].iter().for_each(|x| {
        let builded = out.build_out(x);
        assert_eq!(dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", x)), builded);
    })
}

#[test]
fn test_dir_only() {
    ["output", "output/", "output/dir/", "output/ot her/"]
        .iter()
        .for_each(|dir| {
            let dir = data(dir);
            let out = new(&["-o", &dir.to_str().unwrap()]);
            body_test_dir_only(&dir, &out);
        })
}

#[test]
fn test_from_input() {
    ["output", "output/", "output/dir/", "output/ot her/"]
        .iter()
        .for_each(|dir| {
            let dir = data(dir);
            let in_dir = dir.to_str().unwrap();
            let _ = new(&["-o", in_dir]);

            let input = from_cfg::<MCInput>(vec!["-i", in_dir]);
            let mut out = Output::try_from(&input).unwrap();
            out.try_finalize_init().unwrap();

            body_test_dir_only(&dir.join("muxed"), &out);
        })
}

#[test]
fn test_remove_created_dirs() {
    let new_dir = |s: &str| -> PathBuf {
        let s = format!("output/remove/{}", s);
        data(s)
    };

    ["1/", "2/", "abc/", "a b/", "a/b/c/"]
        .iter()
        .for_each(|subdir| {
            let dir = new_dir(subdir);
            let _ = std::fs::remove_dir_all(&dir);
            let out = new(&["-o", &dir.to_str().unwrap()]);

            assert!(dir.exists());
            out.remove_created_dirs();
            assert!(!dir.exists());
        })
}

#[test]
fn test_name_begin_only() {
    let dir = new_dir("muxed");

    ["name", "other", "a b c"].iter().for_each(|x| {
        let out = new(&["-o", x]);

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", x, middle)), builded);
        })
    })
}

#[test]
fn test_name_tail_only() {
    let dir = new_dir("muxed");

    [",name", ",other", ",a b c", ",ab,c"].iter().for_each(|x| {
        let out = new(&["-o", x]);
        let x = x.strip_prefix(',').unwrap();

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", middle, x)), builded);
        })
    })
}

#[test]
fn test_ext_only() {
    let dir = new_dir("muxed");

    [",.mp4", ",.webm", ",.other"].iter().for_each(|x| {
        let out = new(&["-o", x]);
        let x = x.strip_prefix(",.").unwrap();

        assert_eq!(false, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}.{}", middle, x)), builded);
        })
    })
}

fn to_begin_tail(arg: &str) -> (String, String) {
    let mut it = arg.splitn(2, ',');
    (it.next().unwrap().into(), it.next().unwrap().into())
}

#[test]
fn test_name_begin_tail() {
    let dir = new_dir("muxed");

    ["ab,c", "ot,her", "s tart,en d"].iter().for_each(|x| {
        let out = new(&["-o", x]);
        let (begin, tail) = to_begin_tail(x);

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            let name = format!("{}{}{}.mkv", begin, middle, tail);
            assert_eq!(dir.join(name), builded);
        })
    })
}

fn to_begin_tail_ext(arg: &str) -> (String, String, String) {
    let (begin, x) = to_begin_tail(arg);

    let mut it = x.rsplitn(2, '.');
    let ext = it.next().unwrap().to_string();
    let tail = it.next().unwrap().to_string();

    (begin, tail, ext)
}

#[test]
fn test_name_ext() {
    let dir = new_dir("muxed");

    ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
        .iter()
        .for_each(|x| {
            let out = new(&["-o", x]);
            let (begin, tail, ext) = to_begin_tail_ext(x);

            assert_eq!(true, out.need_num());
            assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

            ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
                let builded = out.build_out(middle);
                assert_eq!(&dir, builded.parent().unwrap());

                let name = format!("{}{}{}.{}", begin, middle, tail, ext);
                assert_eq!(dir.join(name), builded);
            })
        })
}

#[test]
fn test_full() {
    ["output", "output/", "output/dir/", "output/ot her/"]
        .iter()
        .for_each(|dir| {
            let dir = data(dir);

            ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
                .iter()
                .for_each(|x| {
                    let x = dir.join(x).to_str().unwrap().to_string();

                    let out = new(&["-o", &x]);
                    let (begin, tail, ext) = to_begin_tail_ext(&x);

                    assert_eq!(true, out.need_num());
                    assert_eq!(&dir.join(".temp-mux-media"), out.temp_dir());

                    ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
                        let builded = out.build_out(middle);
                        assert_eq!(&dir, builded.parent().unwrap());

                        let name = format!("{}{}{}.{}", begin, middle, tail, ext);
                        assert_eq!(dir.join(name), builded);
                    })
                })
        })
}

#[test]
fn test_to_json_args() {
    let d = data("output/to_json_args/output");
    let s_dir = d.to_str().unwrap();

    [
        ("muxed/name,.mkv", vec!["-o", "name"]),
        ("muxed/name,.mkv", vec!["--output", "name"]),
        ("dir/,.mkv", vec!["--output", "dir/"]),
        ("muxed/a,b.mp4", vec!["--output", "a,b.mp4"]),
        (
            "other/a b,c de .webm",
            vec!["--output", "other/a b,c de .webm"],
        ),
    ]
    .into_iter()
    .for_each(|(arg, right)| {
        let arg = new_dir(arg).to_str().unwrap().to_string();
        let left = vec!["--output", &arg];

        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();

        let add_args1 = vec!["--locale", "eng", "--input", s_dir];
        let add_args2 = vec!["--json"];

        let json = d.clone().join(MuxConfig::JSON_NAME);

        let mc_args = append_str_vecs([add_args1.clone(), add_args2.clone(), right]);
        let mc = cfg(mc_args);

        let right = mc.field::<MCOutput>().to_json_args();
        assert_eq!(to_args(&left), right);

        let left = append_str_vecs([add_args1, left, add_args2]);
        mc.write_args_to_json_or_log();
        let right = read_json_args(&json);

        assert_eq!(left, right, "from json err");
    });

    let _ = std::fs::remove_dir_all(&d);
}
