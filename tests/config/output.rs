use super::*;
use std::{env::current_dir, path::PathBuf};

fn cwd_dir(subdirs: &str) -> PathBuf {
    ensure_long_path_prefix(current_dir().unwrap()).join(subdirs)
}

fn assert_eq_cwd_dirs(o: &ConfigOutput, dir_subdirs: &str) {
    let dir = cwd_dir(dir_subdirs);
    assert_eq!(o.dir(), &dir);
    assert_eq!(o.temp_dir(), dir.join(".temp-mux-media"));
}

#[test]
fn default() {
    let o = ConfigOutput::default();
    assert_eq_cwd_dirs(&o, "muxed");
    assert_eq!(o, ConfigOutput::new("./muxed").unwrap());
}

#[test]
fn new() {
    for x in ["a", "b", "c/d/e"] {
        let o = ConfigOutput::new(format!("./{}", x)).unwrap();
        assert_eq_cwd_dirs(&o, x);
    }
}

#[test]
fn try_from_input() {
    ["", "x1_set"].iter().for_each(|i_dir| {
        let i_dir = data(i_dir);
        let i = cfg([p("-i"), &i_dir]).input;
        let o = ConfigOutput::try_from(&i).unwrap();

        assert_eq!(o.dir(), i_dir.join("muxed"));
        assert_eq!(o.temp_dir(), i_dir.join("muxed").join(".temp-mux-media"));
    })
}

#[test]
fn parse() {
    for x in ["a", "b", "c/d/e"] {
        let cwd_dir = format!("./{}", x);
        let o = ConfigOutput::new(&cwd_dir).unwrap();

        assert_eq!(o, cfg(["-o", &cwd_dir]).output);
    }
}

#[test]
fn create_and_remove_dirs() {
    let base = temp("output/create_and_delete_dirs");
    let dir = base.join("1").join("2").join("3");
    let _ = fs::remove_dir_all(&base);
    assert!(!base.exists());

    let mut o = ConfigOutput::new(&dir).unwrap();
    o.create_dirs().unwrap();
    assert!(dir.join(".temp-mux-media").exists());

    o.remove_created_dirs();
    assert!(!base.exists());
}

#[test]
fn to_args() {
    for x in ["a", "b", "c/d/e"] {
        let dir = cwd_dir(x);
        let o = ConfigOutput::new(format!("./{}", x)).unwrap();

        let args = crate::common::to_args(["--output", dir.to_str().unwrap()]);
        assert_eq!(args, o.to_args());
    }
}
