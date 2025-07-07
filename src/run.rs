use crate::{
    MCExitOnErr, MCInput, MCOutput, MCTools, MCVerbosity, MediaInfo, Msg, MuxConfig, MuxError,
    MuxLogger, Output, Tool, TryFinalizeInit, TryInit,
};
use log::{info, warn};
use std::ffi::OsString;
use std::path::PathBuf;

pub fn run() -> Result<(), MuxError> {
    let mc = {
        let mut mc = MuxConfig::try_init()?;
        mc.try_finalize_init()?;
        mc
    };

    MuxLogger::init_with_filter(mc.get::<MCVerbosity>().to_level_filter());

    let output = mc.get::<MCOutput>();
    let result = try_mux(&mc, output);
    output.remove_created_dirs();

    result.map(|cnt| match cnt {
        0 => warn!("{}", Msg::NotMuxedAny),
        _ => {
            info!("{} {} media", Msg::SuccessMuxed, cnt);
            mc.write_args_to_json_or_log();
        }
    })
}

#[inline(always)]
fn try_mux(mc: &MuxConfig, output: &Output) -> Result<usize, MuxError> {
    let input = mc.get::<MCInput>();
    let tools = mc.get::<MCTools>();
    let exit_on_err = *mc.get::<MCExitOnErr>();
    let mut mi: MediaInfo = mc.into();
    let mut fonts: Option<Vec<PathBuf>> = None;
    let mut cnt = 0;

    for media in input.iter_media_grouped_by_stem() {
        let out = output.build_out(media.out_name_middle);
        if out.exists() {
            warn!(
                "{} '{}' {}. {}",
                Msg::File,
                out.display(),
                Msg::IsAlreadyExists,
                Msg::Skipping
            );
            continue;
        }

        mi.upd_cmn_stem(media.stem);
        mi.try_insert_paths_with_filter(&media.files, exit_on_err)?;
        if mi.is_no_files() {
            warn!(
                "{} '{}'. {}",
                Msg::NotOutSaveAny,
                out.display(),
                Msg::Skipping
            );
            continue;
        }

        let mut args: Vec<OsString> = Vec::new();

        args.push("-o".into());
        args.push(out.clone().into());

        mi.append_vec_os_mkvmerge_args(&mut args);
        fonts
            .get_or_insert_with(|| input.collect_fonts())
            .iter()
            .for_each(|f| {
                args.push("--attach-file".into());
                args.push(f.into());
            });

        if mi.len() == 1 && args.len() == 3 {
            warn!(
                "{} '{}'. {}",
                Msg::NotOutChange,
                out.display(),
                Msg::Skipping
            );
            continue;
        }

        tools.run(Tool::Mkvmerge, args, None)?;
        mi.clear();
        cnt += 1;
    }

    Ok(cnt)
}
