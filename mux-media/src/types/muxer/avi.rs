use crate::{
    MediaInfo, Msg, MuxCurrent, MuxError, Muxer, TFlags, ToFfmpegArgs, Tool, ToolOutput,
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

        mi.tools().run(Tool::Ffmpeg, &args).into()
    }
}

#[inline(always)]
fn try_append_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<(), MuxError> {
    let (media, i_track_type) = mi
        .try_take_cmn::<MICmnTrackOrder>()?
        .into_media_and_i_track_type();

    let mut out_args: Vec<OsString> = Vec::with_capacity(4);
    let mut name_args: Vec<OsString> = Vec::with_capacity(4);
    let mut disposition_args: Vec<OsString> = Vec::with_capacity(4);

    let mut pusheds: [Option<(usize, u64)>; 2] = [None, None];
    let mut fid = 0;

    let mut push = |ty| match i_track_type.iter().filter(|(_, _, t)| t == &ty).next() {
        Some(&(i, track, _)) => {
            let media = &media[i];

            args.push("-i".into());
            args.push(media.as_path().into());

            out_args.push("-map".into());
            out_args.push(format!("{}:{}", fid, track).into());

            TrackNames::append_stream(&mut name_args, mi, media, track, fid);
            TFlags::append_stream(&mut disposition_args, mi, media, track, fid);

            pusheds[fid] = Some((i, track));
            fid = 1;
        }

        None => warn!("{} {}", Msg::NotFoundTrack, ty.as_str_mkvtoolnix()),
    };

    push(TrackType::Video);
    push(TrackType::Audio);

    let is_pusheds = |array_i: usize, i: &usize, track: &u64| -> bool {
        pusheds[array_i]
            .filter(|(pu_i, pu_t)| (i == pu_i && track == pu_t))
            .is_some()
    };

    i_track_type
        .into_iter()
        .filter(|(i, t, _)| !is_pusheds(0, i, t) || !is_pusheds(1, i, t))
        .for_each(|(i, t, _)| logs::avi_container_does_not_support(&media[i], t));

    args.append(&mut out_args);
    args.append(&mut name_args);
    args.append(&mut disposition_args);

    Ok(())
}
