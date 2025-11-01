use super::TrackOrder;
use crate::{
    MediaInfo, Muxer, Result, ToFfmpegArgs, TrackType,
    markers::{MICmnTrackOrder, MITICache},
};
use std::ffi::OsString;

impl ToFfmpegArgs for TrackOrder {
    fn try_append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
        let order = mi.try_take_cmn::<MICmnTrackOrder>()?;
        let muxer = mi.cfg.muxer;
        let reencode = mi.cfg.reencode;

        order.iter_first_entries().for_each(|m| {
            args.push("-i".into());
            args.push(m.media.as_path().into());
        });

        order.iter().for_each(|m| {
            args.push("-map".into());
            args.push(format!("{}:{}", m.number, m.track).into());
        });

        order.iter().enumerate().for_each(|(i, m)| {
            if let Some(c) = mi.get_ti::<MITICache>(&m.media, m.track) {
                if !reencode && muxer.is_supported_codec(c.codec_id) {
                    args.push(format!("-c:{}", i).into());
                    args.push("copy".into());
                    return;
                }
            }

            if matches!(m.ty, TrackType::Sub) && matches!(muxer, Muxer::MP4) {
                args.push(format!("-c:{}", i).into());
                args.push("mov_text".into());
            }
        });

        mi.set_cmn::<MICmnTrackOrder>(order);

        Ok(())
    }
}
