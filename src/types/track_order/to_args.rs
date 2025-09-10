use super::TrackOrder;
use crate::{
    MUXER_CODECS, MediaInfo, MuxConfigArg, Muxer, ParseableArg, Result, ToFfmpegArgs,
    ToMkvmergeArgs, TrackType,
    markers::{MICmnTrackOrder, MITICodec},
};
use std::{ffi::OsString, fmt::Write, path::Path};

impl ToMkvmergeArgs for TrackOrder {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        _: &mut MediaInfo,
        _: &Path,
    ) -> Result<()> {
        if self.is_empty() {
            return Ok(());
        }

        let mut order_arg = String::new();
        for (i, m) in self.iter().enumerate() {
            let comma = if i > 0 { "," } else { "" };
            let track = if m.retimed.is_some() { 0 } else { m.track };
            let _ = write!(order_arg, "{}{}:{}", comma, m.number, track);
        }

        args.push(MuxConfigArg::TrackOrder.dashed().into());
        args.push(order_arg.into());

        Ok(())
    }
}

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
            if let Some(codec) = mi.get_ti::<MITICodec>(&m.media, m.track) {
                if !reencode && MUXER_CODECS.is_supported(muxer, codec) {
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
