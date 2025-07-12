use super::common;
use mux_media::{Tool, Tools};
use serde_json::from_reader;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;

fn new() -> Tools {
    Tools::try_from_tools(Tool::iter()).unwrap()
}

#[test]
fn test_set_all_paths() {
    new();
}

#[test]
fn test_write_json() {
    let json = common::data_file("output/write_json.json");
    let _ = fs::remove_file(&json);

    let tools = new().json(&json);

    let srt = common::data_file("srt.srt");
    let args = ["-i".to_string(), srt.to_string_lossy().into_owned()];

    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());

    let file = File::open(&json).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json_args: Vec<String> = from_reader(reader).expect("Failed to parse JSON as Vec<String>");

    assert_eq!(json_args, args);
}

#[cfg(unix)]
#[test]
fn test_not_panic_on_bad_utf8() {
    use std::os::unix::ffi::OsStrExt;

    let json = common::data_file("output/bad_utf8.json");
    let tools = new().json(&json);

    let bad_bytes = &[0x66, 0x6f, 0x6f, 0xFF];
    let args = [OsStr::from_bytes(bad_bytes)];
    assert!(tools.run(Tool::Mkvmerge, &args).is_err());
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
    let path = common::data_file("srt.srt");
    let args = [OsStr::new("-i"), path.as_os_str()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());
}

#[test]
fn test_err_missing_file() {
    let tools = new();
    let path = common::data_file("missing_file.srt");
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
