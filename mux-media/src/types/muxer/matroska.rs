use super::{MuxCurrent, Muxer};
use crate::{
    ArcPathBuf, Input, MediaInfo, MuxError, TFlags, ToMkvmergeArgs, Tool, ToolOutput, Tools,
    TrackOrder,
    i18n::logs,
    json_arg,
    markers::{
        MCAudioTracks, MCButtonTracks, MCChapters, MCDefaultTFlags, MCEnabledTFlags, MCFontAttachs,
        MCForcedTFlags, MCSpecials, MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks,
        MISubCharset, MITargets,
    },
};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

impl Muxer {
    #[inline(always)]
    pub(super) fn mux_current_matroska(
        input: &Input,
        tools: &Tools,
        mi: &mut MediaInfo,
        fonts: &mut Option<Vec<PathBuf>>,
        out: &Path,
    ) -> MuxCurrent<ToolOutput> {
        let mut args = vec![OsString::new(); 2];

        append_os_mkvmerge_args(&mut args, mi);
        push_fonts_to_args(&mut args, fonts, input);

        if args.len() < 4 {
            logs::warn_not_out_change(out);
            mi.clear_current();
            return MuxCurrent::Continue;
        }

        args[0] = OsString::from(json_arg!(Output));
        args[1] = OsString::from(out);

        tools.run(Tool::Mkvmerge, &args).into()
    }
}

#[inline(always)]
fn append_os_mkvmerge_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) {
    match try_init_track_order(mi) {
        Ok(order) => {
            // MediaInfo and Path unused, just trait requirements
            args.append(&mut order.to_os_mkvmerge_args(mi, Path::new("")));

            let (i_route, paths, mut path_args) =
                TFlags::track_order_into_os_mkvmerge_args(mi, order);

            i_route.into_iter().for_each(|i| {
                append_target_args(args, mi, &paths[i]);
                args.append(&mut std::mem::take(&mut path_args[i]));
                args.push(paths[i].as_path().into());
            })
        }

        Err(e) => {
            log::warn!("{}", e);
            let paths: Vec<ArcPathBuf> = mi.cache().of_files.keys().cloned().collect();
            for path in paths {
                append_target_args(args, mi, &path);
                fallback_append_target_flags(args, mi, &path);
                args.push(path.as_path().into());
            }
        }
    };
}

#[inline(always)]
fn push_fonts_to_args(args: &mut Vec<OsString>, fonts: &mut Option<Vec<PathBuf>>, input: &Input) {
    fonts
        .get_or_insert_with(|| input.collect_fonts_with_filter_and_sort())
        .iter()
        .for_each(|f| {
            args.push("--attach-file".into());
            args.push(f.into());
        })
}

#[inline(always)]
fn try_init_track_order(mi: &mut MediaInfo) -> Result<TrackOrder, MuxError> {
    TrackOrder::try_from(mi)
}

macro_rules! append_target_args {
    ($args:expr, $mi:ident, $path:expr; $( $field:ident ),*) => {
        // Cache MITargets if need. Immediate return if None
        if let None = $mi.get::<MITargets>($path) {
            return;
        }

        $(
            if let Some(targets) = $mi.unmut::<MITargets>($path) {
                let mut args = $mi.mux_config.trg_field::<$field>(targets).to_os_mkvmerge_args($mi, $path);
                $args.append(&mut args);
            }
        )*
    };
}

fn append_target_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    append_target_args!(
        args, mi, path;
        MCAudioTracks, MCSubTracks, MCVideoTracks, MCButtonTracks,
        MCChapters, MCFontAttachs, MCTrackNames, MCTrackLangs,
        MCSpecials
    );

    if let Some(Ok(sc)) = mi
        .auto_flags()
        .auto_charsets
        .then(|| mi.try_get::<MISubCharset>(path).map(|sc| sc.clone()))
    {
        args.append(&mut sc.to_os_mkvmerge_args(mi, path))
    }
}

#[inline(always)]
fn fallback_append_target_flags(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    append_target_args!(
        args, mi, path;
        MCDefaultTFlags, MCForcedTFlags, MCEnabledTFlags
    );
}
