use super::common::*;
use mux_media::*;
use serde_json::from_reader;
#[cfg(all(
    feature = "with_embedded_bins",
    windows,
    any(target_arch = "x86", target_arch = "x86_64")
))]
use std::path::Path;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

fn new() -> Tools {
    Tools::try_from_tools(Tool::iter()).unwrap()
}

#[test]
fn test_set_paths() {
    let tools = new();
    Tool::iter().for_each(|tool| {
        let exp = PathBuf::from(tool.as_ref());
        assert_eq!(&exp, tools.get_path(tool).unwrap());
    })
}

#[cfg(all(
    feature = "with_embedded_bins",
    windows,
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[test]
fn test_set_bundled_paths() {
    use mux_media::markers::MCTools;

    let d = data("");
    let dir = data("tools_bundled/");
    let args = vec![Path::new("-i"), &d, Path::new("-o"), &dir];

    let mut mc = cfg(args);
    mc.try_finalize_init().unwrap();
    let mut tools = mc.field::<MCTools>().clone();
    tools.try_upd_path(Tool::Ffmpeg).unwrap();

    let dir = dir.join(".temp-mux-media");

    Tool::iter().for_each(|tool| {
        let exp = dir.join(format!("{}.exe", tool.as_ref()));
        assert_eq!(&exp, tools.get_path(tool).unwrap())
    });

    let args = vec![
        Path::new("-i"),
        &d,
        Path::new("-o"),
        &dir,
        Path::new("--user-tools"),
    ];
    let mut mc = cfg(args);
    mc.try_finalize_init().unwrap();
    let mut tools = mc.field::<MCTools>().clone();
    tools.try_upd_path(Tool::Ffmpeg).unwrap();

    Tool::iter()
        .for_each(|tool| assert_eq!(Path::new(tool.as_ref()), tools.get_path(tool).unwrap()));
}

#[test]
fn test_write_json() {
    let json = data("output/write_json.json");
    let _ = fs::remove_file(&json);

    let mut tools = new();
    tools.set_json(&json);

    let srt = data("srt.srt");
    let args = ["-i".to_string(), srt.to_string_lossy().into_owned()];

    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());

    let file = File::open(&json).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json_args: Vec<String> = from_reader(reader).expect("Failed to parse JSON as Vec<String>");

    assert_eq!(json_args, args);
}

#[test]
fn test_tool_helps() {
    let tools = new();
    let args = ["-h"];
    for tool in Tool::iter() {
        assert!(tools.run(tool, &args).is_ok());
    }
}

#[test]
fn test_err_incorrect_cmd() {
    let tools = new();
    assert!(tools.run(Tool::Mkvmerge, &["incorrect"]).is_err());
}

#[test]
fn test_mkvmerge_i() {
    let tools = new();
    let path = data("srt.srt");
    let args = [OsStr::new("-i"), path.as_os_str()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());
}

#[test]
fn test_err_missing_file() {
    let tools = new();
    let path = data("missing_file.srt");
    let args = [OsStr::new("-i"), path.as_os_str()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_err());
}

#[test]
fn test_tool_output() {
    let tools = new();
    let out = tools.run(Tool::Mkvmerge, ["-h"]).unwrap();
    assert!(out.as_str_stdout().contains("Global options:"));
    assert!(out.as_str_stderr().is_empty());
}
