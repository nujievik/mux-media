use crate::common::*;
use mux_media::*;
use std::{
    env,
    path::{Path, PathBuf},
};

fn new(args: &[&str]) -> Output {
    cfg::<_, &&str>(args).get::<MCOutput>().clone()
}

fn new_fin(args: &[&str]) -> Output {
    let mut out = new(args);
    out.try_finalize_init().unwrap();
    out
}

fn dir(s: &str) -> PathBuf {
    let mut dir = env::current_dir().unwrap();
    dir.push(s);
    dir
}

fn build_test_empty_args(out: &Output, dir: PathBuf) {
    assert_eq!(false, out.need_num());
    assert_eq!(Path::new(""), out.get_temp_dir());

    ["1", "2", "abc", "n.am.e."].iter().for_each(|name| {
        let builded = out.build_out(name);
        assert_eq!(&dir, builded.parent().unwrap());
        assert_eq!(dir.join(format!("{}.mkv", name)), builded);
    })
}

#[test]
fn test_default() {
    let out = Output::default();
    build_test_empty_args(&out, PathBuf::from("./muxed/"));
}

#[test]
fn test_empty() {
    let out = new(&[]);
    build_test_empty_args(&out, dir("muxed/"));
}

#[test]
fn test_dir_only() {
    ["output", "output/", "output/dir/", "output/ot her/"]
        .iter()
        .for_each(|dir| {
            let dir = data_file(dir);
            let out = new_fin(&["-o", &dir.to_str().unwrap()]);

            assert_eq!(false, out.need_num());
            assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

            ["1", "2", "abc", "n.am.e."].iter().for_each(|x| {
                let builded = out.build_out(x);
                assert_eq!(&dir, builded.parent().unwrap());
                assert_eq!(dir.join(format!("{}.mkv", x)), builded);
            })
        })
}

#[test]
fn test_name_begin_only() {
    let dir = dir("");

    ["name", "other", "a b c"].iter().for_each(|x| {
        let out = new_fin(&["-o", x]);

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", x, middle)), builded);
        })
    })
}

#[test]
fn test_name_tail_only() {
    let dir = dir("");

    [",name", ",other", ",a b c", ",ab,c"].iter().for_each(|x| {
        let out = new_fin(&["-o", x]);
        let x = x.strip_prefix(',').unwrap();

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

        ["1", "2", "abc", "n.am.e."].iter().for_each(|middle| {
            let builded = out.build_out(middle);
            assert_eq!(&dir, builded.parent().unwrap());
            assert_eq!(dir.join(format!("{}{}.mkv", middle, x)), builded);
        })
    })
}

#[test]
fn test_ext_only() {
    let dir = dir("");

    [",.mp4", ",.webm", ",.other"].iter().for_each(|x| {
        let out = new_fin(&["-o", x]);
        let x = x.strip_prefix(",.").unwrap();

        assert_eq!(false, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

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
    let dir = dir("");

    ["ab,c", "ot,her", "s tart,en d"].iter().for_each(|x| {
        let out = new_fin(&["-o", x]);
        let (begin, tail) = to_begin_tail(x);

        assert_eq!(true, out.need_num());
        assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

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
    let dir = dir("");

    ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
        .iter()
        .for_each(|x| {
            let out = new_fin(&["-o", x]);
            let (begin, tail, ext) = to_begin_tail_ext(x);

            assert_eq!(true, out.need_num());
            assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

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
            let dir = data_file(dir);

            ["ab,c .mp4", "ot,her.webm", "s tart,en d.other"]
                .iter()
                .for_each(|x| {
                    let x = dir.join(x).to_str().unwrap().to_string();

                    let out = new_fin(&["-o", &x]);
                    let (begin, tail, ext) = to_begin_tail_ext(&x);

                    assert_eq!(true, out.need_num());
                    assert_eq!(&dir.join(".temp-mux-media"), out.get_temp_dir());

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
    let d = data_file("output/to_json_args/output");
    let s_dir = d.to_str().unwrap();

    [
        ("name", vec!["-o", "name"]),
        ("name", vec!["--output", "name"]),
        ("dir/", vec!["--output", "dir/"]),
        ("ab.mp4", vec!["--output", "a,b.mp4"]),
        (
            "other/a bc de .webm",
            vec!["--output", "other/a b,c de .webm"],
        ),
    ]
    .into_iter()
    .for_each(|(arg, right)| {
        let arg = dir(arg);
        let arg = arg.to_str().unwrap();

        let left = vec!["--output", arg];

        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();

        let add1 = vec!["--input", s_dir];
        let add2 = vec!["--locale", "eng"];
        let json = d.clone().join(MuxConfig::JSON_NAME);

        let mc_args = append_str_vecs([add1.clone(), add2.clone(), right]);
        let mc = cfg(mc_args);

        let right = mc.get::<MCOutput>().to_json_args();
        assert_eq!(to_args(&left), right);

        let left = append_str_vecs([add1, left, add2]);
        mc.write_args_to_json_or_log();
        let right = read_json_args(&json);

        assert_eq!(left, right, "from json err");
    });

    let _ = std::fs::remove_dir_all(&d);
}
