use super::StreamsOrder;
use crate::{MediaInfo, Muxer, Result, ToFfmpegArgs, markers::*};
use std::{collections::HashSet, ffi::OsString, path::Path};

impl ToFfmpegArgs for StreamsOrder {
    fn append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
        let order = mi.try_take_cmn(MICmnStreamsOrder)?;

        let mut chapters_src_num = 0usize;
        let mut charenc_nums = HashSet::<usize>::new();

        order.iter_first_entries().for_each(|m| {
            if let Some((start, end)) = &m.src_time {
                args.push("-ss".into());
                args.push(start.to_string().into());
                args.push("-to".into());
                args.push(end.to_string().into());
            }

            if let Some(enc) = get_sub_charenc(mi, &m.key) {
                args.push("-sub_charenc".into());
                args.push(enc.into());
                let _ = charenc_nums.insert(m.src_num);
            }

            args.push("-i".into());
            args.push(m.src().into());
            chapters_src_num += 1;
        });

        let cfg = mi.cfg;
        let chapters = order
            .iter_first_entries()
            .find_map(|m| {
                mi.get(MITargetPaths, &m.key)
                    .and_then(|ts| cfg.get_targets(CfgChapters, ts))
            })
            .or_else(|| cfg.get_target(CfgChapters, "video"))
            .unwrap_or_else(|| &cfg.chapters);

        if let Some(f) = chapters.file.as_ref() {
            args.push("-i".into());
            args.push(f.into());
        }

        order.iter().for_each(|m| {
            args.push("-map".into());
            args.push(format!("{}:{}", m.src_num, m.i_stream).into());
        });

        if chapters.no_flag {
            args.push("-map_chapters".into());
            args.push("-1".into());
        } else if chapters.file.is_some() {
            args.push("-map_chapters".into());
            args.push(chapters_src_num.to_string().into());
        }

        let reencode = cfg.reencode;
        let muxer = cfg.muxer;

        for (i, m) in order.iter().enumerate() {
            if charenc_nums.contains(&m.src_num) {
                if let Some(ext) = m.key.extension() {
                    args.push(format!("-c:{}", i).into());
                    args.push(ext.into());
                    continue;
                }
            }

            let streams = mi.try_get(MIStreams, &m.key)?;
            let stream = &streams[m.key_i_stream];

            if !reencode && muxer.is_supported_copy(stream) {
                args.push(format!("-c:{}", i).into());
                args.push("copy".into());
                continue;
            }

            if m.ty.is_sub() && matches!(muxer, Muxer::MP4) {
                args.push(format!("-c:{}", i).into());
                args.push("mov_text".into());
            }
        }

        mi.set_cmn(MICmnStreamsOrder, order);
        Ok(())
    }
}

fn get_sub_charenc<'a>(mi: &'a mut MediaInfo, src: &Path) -> Option<&'a str> {
    if *mi.cfg.auto_flags.encs {
        mi.get(MISubCharEncoding, src)
            .and_then(|enc| enc.get_ffmpeg_sub_charenc())
    } else {
        None
    }
}
