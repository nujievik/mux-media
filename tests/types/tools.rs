use super::common;
use mux_media::{Tool, Tools};
use serde_json::from_reader;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;

fn init() -> Tools {
    Tools::try_from_tools(Tool::iter()).unwrap()
}

fn init_with_json(name: &str) -> Tools {
    let json = common::data_file(name);
    init().json(&json)
}

#[test]
fn test_set_all_paths() {
    init();
}

#[test]
fn test_write_json() {
    let json = common::data_file("output/write_json.json");
    let _ = fs::remove_file(&json);

    let tools = init().json(&json);

    let srt = common::data_file("srt.srt");
    let args = ["-i".to_string(), srt.to_string_lossy().into_owned()];

    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());

    let file = File::open(&json).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json_args: Vec<String> = from_reader(reader).expect("Failed to parse JSON as Vec<String>");

    assert_eq!(json_args, args);
}

#[test]
fn test_not_panic_on_bad_utf8() {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;

        let tools = init_with_json("output/bad_utf8.json");
        let bad_bytes = &[0x66, 0x6f, 0x6f, 0xFF];
        let args = [OsStr::from_bytes(bad_bytes)];
        assert!(tools.run(Tool::Mkvmerge, &args).is_err());
    }

    #[cfg(windows)]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let tools = init_with_json("output/bad_utf8.json");
        let bad_bytes = [0x0066, 0x006F, 0x006F, 0xD800];
        let args = [OsString::from_wide(&bad_bytes)];
        assert!(tools.run(Tool::Mkvmerge, &args).is_err());
    }
}

#[test]
fn test_tool_helps() {
    let tools = init();
    let args = ["-h"];
    for tool in Tool::iter() {
        assert!(tools.run(tool, &args).is_ok());
    }
}

#[test]
fn test_err_incorrect_cmd() {
    let tools = init();
    assert!(tools.run(Tool::Mkvmerge, &["incorrect"]).is_err());
}

#[test]
fn test_mkvmerge_i() {
    let tools = init();
    let path = common::data_file("srt.srt");
    let args = [OsStr::new("-i"), path.as_os_str()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());
}

#[test]
fn test_err_missing_file() {
    let tools = init();
    let path = common::data_file("missing_file.srt");
    let args = [OsStr::new("-i"), path.as_os_str()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_err());
}

#[test]
fn test_tool_output() {
    let tools = init();
    let out = tools.run(Tool::Mkvmerge, ["-h"]).unwrap();
    assert!(out.as_str_stdout().contains("Global options:"));
    assert!(out.as_str_stderr().is_empty());
}
