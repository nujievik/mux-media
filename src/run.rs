use crate::{
    MCExitOnErr, MCInput, MCLocale, MCOutput, MCTools, MCVerbosity, MediaInfo, Msg, MuxConfig,
    MuxError, MuxLogger, Output, Tool, TryInit,
};
use log::warn;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn run() -> Result<(), MuxError> {
    let mc = MuxConfig::try_init()?;

    MuxLogger::init_with_filter(mc.get::<MCVerbosity>().to_level_filter());
    Msg::upd_lang_code_or_log_warn(*mc.get::<MCLocale>());

    let output = mc.get::<MCOutput>();
    let result = mux(&mc, output);

    output.remove_created_dirs();
    result
}

#[inline]
fn mux(mc: &MuxConfig, output: &Output) -> Result<(), MuxError> {
    let input = mc.get::<MCInput>();
    let tools = mc.get::<MCTools>();
    let exit_on_err = *mc.get::<MCExitOnErr>();
    let mut mi: MediaInfo = mc.into();
    let mut fonts: Option<Vec<PathBuf>> = None;

    for media in input.iter_media_grouped_by_stem() {
        let out = output.build_out(media.out_name_middle);
        if out.exists() {
            warn!("File '{}' is already exists. Skipping", out.display());
            continue;
        }

        mi.upd_stem(media.stem);
        mi.try_insert_paths_with_filter(&media.files, exit_on_err)?;
        if mi.is_empty() {
            warn!(
                "No found any save Track or Attach for out '{}'. Skipping",
                out.display()
            );
            continue;
        }

        let mut args: Vec<OsString> = Vec::new();
        args.push("-o".into());
        args.push(out.into());
        mi.extend_vec_os_mkvmerge_args(&mut args);
        fonts
            .get_or_insert_with(|| input.collect_fonts())
            .iter()
            .for_each(|f| {
                args.push("--attach-file".into());
                args.push(f.into());
            });

        if mi.len_cache() == 1 && args.len() == 3 {
            warn!("No found any change for out. Skipping");
            continue;
        }

        tools.run(Tool::Mkvmerge, args, None)?;
        mi.clear();
    }

    Ok(())
}
