use crate::{
    MediaInfo, Msg, MuxCurrent, MuxError, Muxer, ToFfmpegArgs, Tool, ToolOutput, TrackFlags,
    TrackNames, TrackType, i18n::logs, markers::MICmnTrackOrder,
};
use log::warn;
use std::{ffi::OsString, path::Path};

impl Muxer {
    #[inline(always)]
    pub(super) fn mux_current_avi(mi: &mut MediaInfo, out: &Path) -> MuxCurrent<ToolOutput> {
        let mut args = Vec::<OsString>::new();

        if let Err(e) = try_append_args(&mut args, mi) {
            return MuxCurrent::Err(e);
        }

        if args.len() < 3 {
            logs::warn_not_out_save_any(out);
            mi.clear_current();
            return MuxCurrent::Continue;
        }

        args.push(out.into());

        mi.tools.run(Tool::Ffmpeg, &args).into()
    }
}

#[inline(always)]
fn try_append_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<(), MuxError> {
    let order = mi.try_take_cmn::<MICmnTrackOrder>()?;

    let mut out_args: Vec<OsString> = Vec::with_capacity(4);
    let mut name_args: Vec<OsString> = Vec::with_capacity(4);
    let mut disposition_args: Vec<OsString> = Vec::with_capacity(4);

    let mut pusheds: [Option<(u64, u64)>; 2] = [None, None];
    let mut fid = 0usize;

    let mut push = |ty| match order.iter().filter(|m| m.ty == ty).next() {
        Some(m) => {
            args.push("-i".into());
            args.push(m.media.as_path().into());

            out_args.push("-map".into());
            out_args.push(format!("{}:{}", fid, m.track).into());

            TrackNames::append_stream(&mut name_args, mi, &m.media, m.track, fid);
            TrackFlags::append_stream(&mut disposition_args, mi, &m.media, m.track, fid);

            pusheds[fid] = Some((m.number, m.track));
            fid += 1;
        }

        None => warn!("{} {}", Msg::NotFoundTrack, ty.as_str_mkvtoolnix()),
    };

    push(TrackType::Video);
    push(TrackType::Audio);

    order
        .iter()
        .filter(|m| {
            let current = Some((m.number, m.track));
            current != pusheds[0] && current != pusheds[1]
        })
        .for_each(|m| logs::avi_container_does_not_support(&m.media, m.track));

    args.append(&mut out_args);
    args.append(&mut name_args);
    args.append(&mut disposition_args);

    Ok(())
}
