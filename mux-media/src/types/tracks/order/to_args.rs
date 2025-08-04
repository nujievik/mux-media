use crate::{
    MUXER_CODECS, MediaInfo, MuxConfigArg, MuxError, Muxer, ParseableArg, ToFfmpegArgs,
    ToMkvmergeArgs, TrackOrder, TrackType,
    markers::{MCMuxer, MCReencode, MICmnTrackOrder, MITICodec},
};
use std::{ffi::OsString, fmt::Write, path::Path};

impl ToMkvmergeArgs for TrackOrder {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        _: &mut MediaInfo,
        _: &Path,
    ) -> Result<(), MuxError> {
        if self.i_track_type.is_empty() {
            return Ok(());
        }

        let factor = if self.media.len() > 9 { 5 } else { 4 };
        let mut order_arg = String::with_capacity(self.i_track_type.len() * factor);
        for (i, (fid, track, _)) in self.i_track_type.iter().enumerate() {
            if i > 0 {
                order_arg.push(',');
            }
            let _ = write!(order_arg, "{}:{}", fid, track);
        }

        args.push(MuxConfigArg::TrackOrder.dashed().into());
        args.push(order_arg.into());

        Ok(())
    }
}

impl ToFfmpegArgs for TrackOrder {
    fn try_append_ffmpeg_args(
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
    ) -> Result<(), MuxError> {
        let order = mi.try_take_cmn::<MICmnTrackOrder>()?;
        let media = order.media();
        let muxer = *mi.mux_config.field::<MCMuxer>();
        let reencode = *mi.mux_config.field::<MCReencode>();

        media.iter().for_each(|input| {
            args.push("-i".into());
            args.push(input.as_path().into());
        });

        order.i_track_type().iter().for_each(|(i, track, _)| {
            args.push("-map".into());
            args.push(format!("{}:{}", i, track).into());
        });

        order
            .i_track_type()
            .iter()
            .enumerate()
            .for_each(|(stream, &(i, track, ty))| {
                if let Some(codec) = mi.get_ti::<MITICodec>(&media[i], track) {
                    if !reencode && MUXER_CODECS.is_supported(muxer, codec) {
                        args.push(format!("-c:{}", stream).into());
                        args.push("copy".into());
                        return;
                    }
                }

                if matches!(ty, TrackType::Sub) && matches!(muxer, Muxer::MP4) {
                    args.push(format!("-c:{}", stream).into());
                    args.push("mov_text".into());
                }
            });

        mi.set_cmn::<MICmnTrackOrder>(order);

        Ok(())
    }
}
