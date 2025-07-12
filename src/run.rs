use crate::{
    Input, MCExitOnErr, MCInput, MCOutput, MCTools, MCVerbosity, MediaInfo, Msg, MuxConfig,
    MuxError, MuxLogger, Output, Tool, TryFinalizeInit, TryInit, i18n::logs, json_arg,
};
use log::{error, info, trace, warn};
use std::{ffi::OsString, path::PathBuf};

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
        let out = output.build_out(media.out_name_middle.as_ref());

        if out.exists() {
            logs::warn_file_is_already_exists(&out);
            continue;
        }

        mi.upd_cmn_stem(media.stem.to_os_string());
        mi.try_insert_paths_with_filter(&media.files, exit_on_err)?;

        if mi.is_no_files() {
            logs::warn_not_out_save_any(&out);
            continue;
        }

        let mut args = vec![OsString::new(); 2];

        mi.append_os_mkvmerge_args(&mut args);
        push_fonts_to_args(&mut args, &mut fonts, input);

        if args.len() < 4 {
            logs::warn_not_out_change(&out);
            continue;
        }

        args[0] = json_arg!(Output).into();
        args[1] = out.into();

        match tools.run(Tool::Mkvmerge, &args) {
            Ok(out) => {
                trace!("{}", out);
                out.log_warns();
                cnt += 1;
            }
            Err(e) if exit_on_err => return Err(e),
            Err(e) => error!("{}", e),
        }

        mi.clear();
    }

    Ok(cnt)
}

#[inline(always)]
fn push_fonts_to_args(args: &mut Vec<OsString>, fonts: &mut Option<Vec<PathBuf>>, input: &Input) {
    fonts
        .get_or_insert_with(|| input.collect_fonts())
        .iter()
        .for_each(|f| {
            args.push("--attach-file".into());
            args.push(f.into());
        })
}
