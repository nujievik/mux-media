use super::common::*;
use mux_media::*;
use std::{path::PathBuf, sync::LazyLock};

static TOOLS: LazyLock<Tools> = LazyLock::new(|| {
    let paths = Box::new(ToolPaths {
        sys: true,
        ..Default::default()
    });
    paths
        .try_resolve_many(Tool::iter().collect::<Vec<_>>(), "")
        .unwrap();
    let paths = Box::leak::<'static>(paths);
    Tools(&*paths)
});

#[test]
fn test_set_paths() {
    Tool::iter().for_each(|t| {
        let exp = PathBuf::from(t.as_ref());
        assert_eq!(TOOLS.0[t].get().unwrap(), &exp);
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
            assert_eq!(tools.0[t].get().unwrap(), &exp);
        })
    };

    with_args(vec![p("-i"), &d, p("-o"), &dir], true);
    with_args(vec![p("-i"), &d, p("-o"), &dir, p("--sys")], false);
}

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
    assert!(out.stdout.contains("Global options"));
}
