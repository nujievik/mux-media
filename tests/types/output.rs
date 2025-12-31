use crate::common::*;
use mux_media::*;
use std::path::{Path, PathBuf};

#[test]
fn parse_empty_args() {
    let dir = new_dir("muxed/");
    let assert = |out: &Output| {
        assert!(!out.need_num());
        assert_eq!("", &out.name_begin);
        assert_eq!("", &out.name_tail);
        assert_eq!("mkv", &out.ext);

        ["1", "2", "abc", "n.am.e."].iter().for_each(|name| {
            let builded = out.build_out(name);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}.mkv", name)), builded);
        })
    };

    assert(&Output::try_from_path("").unwrap());
    assert(&cfg::<_, &str>([]).output);
}

fn body_parse_dir_only(dir: &Path, out: &Output) {
    assert!(!out.need_num());
    assert_eq!("", &out.name_begin);
    assert_eq!("", &out.name_tail);
    assert_eq!("mkv", &out.ext);

    ["1", "2", "abc", "n.am.e."].iter().for_each(|x| {
        let builded = out.build_out(x);
        assert_eq!(dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", x)), builded);
    })
}

#[test]
fn parse_dir_only() {
    ["", "output/dir_only/", "output/dir_only other/"]
        .iter()
        .for_each(|dir| {
            let dir = temp(dir);
            let o = cfg([p("-o"), &dir]).output;
            body_parse_dir_only(&dir, &o);
        })
}

#[test]
fn from_input() {
    ["", "x1_set"].iter().for_each(|dir| {
        let dir = data(dir);
        let i = cfg([p("-i"), &dir]).input;
        let o = Output::try_from(&i).unwrap();
        body_parse_dir_only(&dir.join("muxed"), &o);
    })
}

#[test]
fn parse_name_begin_only() {
    let dir = new_dir("muxed");

    ["name", "other", "a b c"].iter().for_each(|x| {
        let out = cfg(["-o", x]).output;
        assert!(out.need_num());
        assert_eq!(x, &out.name_begin);
        assert_eq!("", &out.name_tail);
        assert_eq!("mkv", &out.ext);

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", x, middle)), builded);
        })
    })
}

#[test]
fn parse_name_tail_only() {
    let dir = new_dir("muxed");

    [",name", ",other", ",a b c", ",ab,c"].iter().for_each(|x| {
        let out = cfg(["-o", x]).output;
        let x = x.strip_prefix(',').unwrap();

        assert!(out.need_num());
        assert_eq!("", &out.name_begin);
        assert_eq!(x, &out.name_tail);
        assert_eq!("mkv", &out.ext);

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", middle, x)), builded);
        })
    })
}

#[test]
fn parse_ext_only() {
    let dir = new_dir("muxed");

    [",.mp4", ",.webm", ",.other"].iter().for_each(|x| {
        let out = cfg(["-o", x]).output;
        let x = x.strip_prefix(",.").unwrap();

        assert!(!out.need_num());
        assert_eq!("", &out.name_begin);
        assert_eq!("", &out.name_tail);
        assert_eq!(x, &out.ext);

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
fn parse_begin_and_tail() {
    let dir = new_dir("muxed");

    ["ab,c", "ot,her", "s tart,en d"].iter().for_each(|x| {
        let out = cfg(["-o", x]).output;
        let (begin, tail) = to_begin_tail(x);

        assert!(out.need_num());
        assert_eq!(begin.as_str(), &out.name_begin);
        assert_eq!(tail.as_str(), &out.name_tail);
        assert_eq!("mkv", &out.ext);

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
fn parse_begin_and_tail_and_ext() {
    let dir = new_dir("muxed");

    ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
        .iter()
        .for_each(|x| {
            let out = cfg(["-o", x]).output;
            let (begin, tail, ext) = to_begin_tail_ext(x);

            assert!(out.need_num());
            assert_eq!(begin.as_str(), &out.name_begin);
            assert_eq!(tail.as_str(), &out.name_tail);
            assert_eq!(ext.as_str(), &out.ext);

            ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
                let builded = out.build_out(middle);
                assert_eq!(&dir, builded.parent().unwrap());

                let name = format!("{}{}{}.{}", begin, middle, tail, ext);
                assert_eq!(dir.join(name), builded);
            })
        })
}

#[test]
fn parse_full() {
    ["output/dir/", "output/ot her/"].iter().for_each(|dir| {
        let dir = temp(dir);

        ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
            .iter()
            .for_each(|x| {
                let (begin, tail, ext) = to_begin_tail_ext(x);
                let out = cfg([p("-o"), &dir.join(x)]).output;

                assert!(out.need_num());
                assert_eq!(begin.as_str(), &out.name_begin);
                assert_eq!(tail.as_str(), &out.name_tail);
                assert_eq!(ext.as_str(), &out.ext);

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
fn remove_created_dirs() {
    let new_dir = |s: &str| -> PathBuf {
        let s = format!("output/remove/{}", s);
        temp(s)
    };

    ["1/", "2/", "abc/", "a b/", "a/b/c/"]
        .iter()
        .for_each(|subdir| {
            let dir = new_dir(subdir);
            let _ = std::fs::remove_dir_all(&dir);
            assert!(!dir.exists());

            let mut out = cfg([p("-o"), &dir]).output;
            let _ = out.try_finalize_init();
            assert!(dir.exists());
            out.remove_created_dirs();
            assert!(!dir.exists());
        })
}

#[test]
fn test_to_json_args() {
    let d = temp("to_json_args/output");
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
        let _ = std::fs::create_dir_all(&d).unwrap();

        let add_args1 = vec!["--locale", "eng", "--input", s_dir];
        let add_args2 = vec!["--save-config"];

        let json = d.clone().join("mux-media.json");

        let mc_args = append_str_vecs([add_args1.clone(), add_args2.clone(), right]);
        let mc = cfg(mc_args);

        let right = mc.output.to_json_args();
        assert_eq!(to_args(&left), right);

        let left = append_str_vecs([add_args1, left, add_args2]);
        mc.try_save_config().unwrap();
        let right = read_json_args(&json);

        assert_eq!(left, right, "from json err");
    });
}
