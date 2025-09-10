use crate::{
    ArcPathBuf, MediaInfo, MuxConfigArg, MuxCurrent, MuxError, Muxer, ParseableArg, ToMkvmergeArgs,
    Tool, ToolOutput,
    i18n::logs,
    markers::{
        MCAudioTracks, MCChapters, MCDefaultTrackFlags, MCEnabledTrackFlags, MCFontAttachs,
        MCForcedTrackFlags, MCSpecials, MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks,
        MICmnExternalFonts, MICmnTrackOrder, MISubCharset, MITargets,
    },
};
use std::{ffi::OsString, mem, path::Path};

impl Muxer {
    #[inline(always)]
    pub(super) fn mux_current_matroska(mi: &mut MediaInfo, out: &Path) -> MuxCurrent<ToolOutput> {
        let mut args = vec![OsString::new(); 2];

        append_mkvmerge_args(&mut args, mi);
        push_fonts_to_args(&mut args, mi);

        if args.len() < 4 {
            logs::warn_not_out_change(out);
            mi.clear_current();
            return MuxCurrent::Continue;
        }

        args[0] = MuxConfigArg::Output.dashed().into();
        args[1] = OsString::from(out);

        mi.tools.run(Tool::Mkvmerge, &args).into()
    }
}

#[inline(always)]
fn append_mkvmerge_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) {
    let mut flag_args = match mi.try_to_mkvmerge_args_of_flags() {
        Ok(flag_args) => flag_args,
        Err(e) => return fallback_append_mkvmerge_args(args, mi, e),
    };

    let order = match mi.try_take_cmn::<MICmnTrackOrder>() {
        Ok(order) => order,
        Err(e) => return fallback_append_mkvmerge_args(args, mi, e),
    };

    let is_retimed = order[0].is_retimed;

    order.append_mkvmerge_args(args, mi, "".as_ref());

    order.iter_first_entries().for_each(|m| {
        if is_retimed {
            //args.append(&mut mem::take(&mut flag_args[m.number as usize]));
            match &m.retimed {
                Some(parts) => parts.iter().enumerate().for_each(|(i, p)| {
                    if i > 0 {
                        args.push("+".into());
                    }
                    args.push(p.into());
                }),
                None => args.push(m.media.as_path().into()),
            }
        } else {
            append_target_args(args, mi, &m.media);
            args.append(&mut mem::take(&mut flag_args[m.number as usize]));
            args.push(m.media.as_path().into());
        }
    });

    mi.set_cmn::<MICmnTrackOrder>(order);
}

#[inline]
fn fallback_append_mkvmerge_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, err: MuxError) {
    log::warn!("{}. Fallback", err);
    let paths: Vec<ArcPathBuf> = mi.cache.of_files.keys().cloned().collect();

    for path in paths {
        append_target_args(args, mi, &path);
        fallback_append_target_flags(args, mi, &path);
        args.push(path.as_path().into());
    }
}

#[inline(always)]
fn push_fonts_to_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) {
    if let Some(fonts) = mi.get_cmn::<MICmnExternalFonts>() {
        fonts.iter().for_each(|f| {
            args.push("--attach-file".into());
            args.push(f.into());
        })
    }
}

macro_rules! append_target_args {
    ($args:expr, $mi:ident, $media:expr; $( $field:ident ),*) => {
        // Cache MITargets if need. Immediate return if None
        if let None = $mi.get::<MITargets>($media) {
            return;
        }

        $(
            if let Some(targets) = $mi.immut::<MITargets>($media) {
                let field = $mi.cfg.target($field, targets);
                field.append_mkvmerge_args($args, $mi, $media);
            }
        )*
    };
}

fn append_target_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, media: &Path) {
    append_target_args!(
        args, mi, media;
        MCAudioTracks, MCSubTracks, MCVideoTracks,
        MCChapters, MCFontAttachs, MCTrackNames, MCTrackLangs,
        MCSpecials
    );

    if !mi.auto_flags.charsets {
        return;
    }

    if let Ok(charset) = mi.try_get::<MISubCharset>(media).map(|c| c.clone()) {
        charset.append_mkvmerge_args(args, mi, media);
    }
}

#[inline(always)]
fn fallback_append_target_flags(args: &mut Vec<OsString>, mi: &mut MediaInfo, media: &Path) {
    append_target_args!(
        args, mi, media;
        MCDefaultTrackFlags, MCForcedTrackFlags, MCEnabledTrackFlags
    );
}
