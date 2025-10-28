use crate::{
    ArcPathBuf, MediaInfo, MuxCurrent, MuxError, Muxer, ToMkvmergeArgs, Tool, ToolOutput,
    TrackOrder, dashed,
    i18n::logs,
    markers::{
        MCAudioTracks, MCChapters, MCDefaultTrackFlags, MCEnabledTrackFlags, MCFontAttachs,
        MCForcedTrackFlags, MCRaws, MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks,
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
            return MuxCurrent::Continue;
        }

        args[0] = dashed!(Output).into();
        args[1] = OsString::from(out);

        mi.tools.run(Tool::Mkvmerge, &args).into()
    }
}

#[inline(always)]
fn append_mkvmerge_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) {
    let flag_args = match mi.try_to_mkvmerge_args_of_flags() {
        Ok(args) => args,
        Err(e) => return fallback_append_mkvmerge_args(args, mi, e),
    };

    let order = match mi.try_take_cmn::<MICmnTrackOrder>() {
        Ok(order) => order,
        Err(e) => return fallback_append_mkvmerge_args(args, mi, e),
    };

    if order.iter().next().map_or(false, |m| m.retimed.is_none()) {
        normal(args, mi, &order, flag_args);
    } else {
        retimed(args, mi, &order);
    }

    mi.set_cmn::<MICmnTrackOrder>(order);

    fn normal(
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        order: &TrackOrder,
        mut flag_args: Vec<Vec<OsString>>,
    ) {
        order.append_mkvmerge_args(args, mi, "".as_ref());
        order.iter_first_entries().for_each(|m| {
            append_target_args(args, mi, &m.media);
            args.append(&mut mem::take(&mut flag_args[m.number as usize]));
            args.push(m.media.as_path().into());
        });
    }

    fn retimed(args: &mut Vec<OsString>, mi: &mut MediaInfo, order: &TrackOrder) {
        order.iter().for_each(|m| {
            let rtm = m.retimed.as_ref().unwrap();
            rtm.append_mkvmerge_args(args, mi, "".as_ref());
        })
    }
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
        MCRaws
    );

    if !*mi.auto_flags.charsets {
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
