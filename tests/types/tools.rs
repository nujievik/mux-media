use super::common::*;
use mux_media::*;
use serde_json::from_reader;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    sync::LazyLock,
};

static TOOLS: LazyLock<Tools> = LazyLock::new(|| {
    let paths = Box::new(ToolPaths {
        user_tools: true,
        ..Default::default()
    });
    paths
        .try_resolve_many(Tool::iter().collect::<Vec<_>>(), "")
        .unwrap();
    let paths = Box::leak::<'static>(paths);
    Tools { paths, json: None }
});

#[test]
fn test_set_paths() {
    Tool::iter().for_each(|t| {
        let exp = PathBuf::from(t.as_ref());
        assert_eq!(TOOLS.paths[t].get().unwrap(), &exp);
    })
}

#[cfg(feature = "static")]
#[test]
fn test_set_bundled_paths() {
    let d = data("");
    let dir = data("tools_bundled/");
    let temp_dir = dir.join(".temp-mux-media");

    let with_args = |args, is_bundled| {
        let mut mc = cfg(args);
        mc.try_finalize_init().unwrap();
        mc.tool_paths
            .try_resolve_many(Tool::iter().collect::<Vec<_>>(), &mc.output.temp_dir)
            .unwrap();
        let tools = Tools::from(&mc);

        Tool::iter().for_each(|t| {
            let s = t.as_ref();
            let exp = if is_bundled {
                temp_dir.join(s)
            } else {
                PathBuf::from(s)
            };
            assert_eq!(tools.paths[t].get().unwrap(), &exp);
        })
    };

    with_args(vec![p("-i"), &d, p("-o"), &dir], true);
    with_args(vec![p("-i"), &d, p("-o"), &dir, p("--user-tools")], false);
}

/*
#[test]
fn test_write_json() {
    let json = data("output/write_json.json");
    let _ = fs::remove_file(&json);
    let paths = &ToolPaths::default();
    let mut tools = Tools { paths, json: None };
    tools.json = Some(PathBuf::from(&json));

    let srt = data("srt.srt");
    let args = ["-i".to_owned(), srt.to_string_lossy().into_owned()];
    assert!(tools.run(Tool::Mkvmerge, &args).is_ok());

    let file = File::open(&json).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json_args: Vec<String> = from_reader(reader).expect("Failed to parse JSON as Vec<String>");

    assert_eq!(json_args, args);
}
*/

#[test]
fn test_tool_helps() {
    Tool::iter().for_each(|t| {
        TOOLS.run(t, ["-h"]).unwrap();
    });
}

#[test]
fn test_errs() {
    assert!(TOOLS.run(Tool::Ffmpeg, &["incorrect"]).is_err());
    let path = data("missing_file.srt");
    assert!(TOOLS.run(Tool::Ffmpeg, [p("-i"), &path]).is_err());
}

#[test]
fn test_tool_output() {
    let out = TOOLS.run(Tool::Ffmpeg, ["-h"]).unwrap();
    assert!(out.as_str_stdout().contains("Global options"));
    assert!(out.as_str_stderr().is_empty());
}
