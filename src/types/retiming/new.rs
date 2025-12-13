mod cache;
mod external_segments;

use super::{Retiming, RetimingChapter, RetimingPart};
use crate::{
    ArcPathBuf, Config, Duration, MediaInfo, MuxError, Result, StreamType, StreamsOrder, ffmpeg,
    markers::*, types::helpers,
};
use cache::CacheMatroska;
use external_segments::find_external_segment;
use log::warn;
use std::{collections::HashMap, path::Path};

impl Retiming<'_, '_> {
    pub(crate) fn try_new<'a, 'b>(
        mi: &'b mut MediaInfo<'a>,
        order: &StreamsOrder,
    ) -> Result<Retiming<'a, 'b>> {
        let mut cache = CacheMatroska::default();
        let (base, i_base_stream, i_matroska_chapters) = try_base(mi, order, &mut cache)?;
        let base_dir = base.parent().unwrap_or(&mi.cfg.input.dir);

        let cs = try_chapters(mi, &cache, &base, i_matroska_chapters, base_dir)?;
        let len = cs.len();

        let mut parts: Vec<RetimingPart> = Vec::with_capacity(len);
        let mut i = 0usize;

        while i < len {
            let src = match save_then_src(mi, &cache, &base, &base_dir, &cs[i]) {
                Some(src) => src,
                None => {
                    i += 1;
                    continue;
                }
            };
            let i_end_chp = i_end_chp(&mi.cfg, &cs, i);

            let (start, start_offset, end, end_offset) =
                try_times(mi, &src, i_base_stream, cs[i].start, cs[i_end_chp].end)?;

            parts.push(RetimingPart {
                i_start_chp: i,
                i_end_chp,
                src,
                start,
                start_offset,
                end,
                end_offset,
            });
            i += 1 + i_end_chp - i;
        }

        if parts.is_empty() {
            return Err(err!("Not saved any part"));
        }

        let mut rtm = Retiming {
            tools: mi.cfg.into(),
            temp_dir: &mi.cfg.output.temp_dir,
            media_info: mi,
            job: mi.job,
            base,
            i_base_stream,
            chapters: cs,
            parts,
            base_splits: Vec::new(),
        };

        if !rtm.is_save_single_part() {
            rtm.init_base_splits()?;
        }

        return Ok(rtm);
    }
}

fn try_times(
    mi: &mut MediaInfo,
    src: &Path,
    i_stream: usize,
    start: Duration,
    end: Duration,
) -> Result<(Duration, f64, Duration, f64)> {
    const ACCEPT_VIDEO_OFFSET: f64 = 10.0; // seconds

    let duration = *mi.try_get(MIVideoDuration, src)?;
    let zero_start_offset = start.as_secs_f64();
    let end_offset = duration.as_secs_f64() - end.as_secs_f64();

    let (start, start_offset) = if zero_start_offset < ACCEPT_VIDEO_OFFSET {
        (Duration::default(), zero_start_offset)
    } else {
        try_nearest_time_offset(src, i_stream, start, duration)?
    };

    let (end, end_offset) = if end_offset.abs() < ACCEPT_VIDEO_OFFSET {
        (duration, end_offset)
    } else {
        try_nearest_time_offset(src, i_stream, end, duration)?
    };

    return Ok((start, start_offset, end, end_offset));

    fn try_nearest_time_offset(
        src: &Path,
        i_stream: usize,
        target: Duration,
        duration: Duration,
    ) -> Result<(Duration, f64)> {
        let t = target.as_secs_f64();
        let first = try_i_frame(src, i_stream, target)?;
        let second = {
            let offset = (t - first.as_secs_f64()) / 2.0;
            let t = Duration::from_secs_f64(t + offset);
            try_i_frame(src, i_stream, t)?
        };
        let third = {
            let t = Duration::from_secs_f64(t + t - first.as_secs_f64());
            try_i_frame(src, i_stream, t)?
        };

        // unwraps safe
        let (nearest, offset) = [first, second, third, duration]
            .into_iter()
            .map(|d| {
                let diff = d.as_secs_f64() - t;
                (d, diff, diff.abs())
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .map(|t| (t.0, t.1))
            .unwrap();

        Ok((nearest, offset))
    }

    fn try_i_frame(src: &Path, i_stream: usize, target: Duration) -> Result<Duration> {
        let mut ictx = ffmpeg::format::input(src)?;
        let stream = ictx.stream(i_stream).ok_or(ffmpeg::Error::StreamNotFound)?;
        let (i, tb) = helpers::ffmpeg_stream_i_tb(&stream);

        let mut opened = helpers::try_ffmpeg_opened(StreamType::Video, &stream)?;
        let seek_target = (target.as_secs_f64() * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
        ictx.seek(seek_target, ..)?;
        opened.flush();

        for (s, packet) in ictx.packets() {
            if s.index() != i {
                continue;
            }

            opened.send_packet(&packet)?;

            loop {
                let mut frame = ffmpeg::util::frame::Video::empty();
                match opened.receive_frame(&mut frame) {
                    Ok(_) => {
                        let pts_time = frame.pts().map(|pts| pts as f64 * tb).unwrap_or(0f64);
                        return Ok(Duration::from_secs_f64(pts_time));
                    }
                    Err(ffmpeg::Error::Other { errno: 11 }) => break,
                    Err(ffmpeg::Error::Eof) => break,
                    Err(e) => return Err(err!("Ffmpeg decoder error: {}", e)),
                }
            }
        }

        Err(err!("Not found I frame"))
    }
}

fn i_end_chp(cfg: &Config, cs: &Vec<RetimingChapter>, mut i: usize) -> usize {
    let uid = &cs[i].uid;
    for j in (i + 1)..cs.len() {
        let eq_uid = uid == &cs[j].uid;
        let save = cfg.retiming_options.is_save_chapter(&cs[j]);

        if (eq_uid && save) || (!eq_uid && !save) {
            i = j;
        } else {
            break;
        }
    }
    i
}

fn save_then_src(
    mi: &MediaInfo,
    cache: &CacheMatroska,
    base: &ArcPathBuf,
    base_dir: &Path,
    c: &RetimingChapter,
) -> Option<ArcPathBuf> {
    if !mi.cfg.retiming_options.is_save_chapter(c) {
        return None;
    }

    return match c.uid.as_ref() {
        Some(u) => match find_external_segment(mi, cache, base_dir, &u) {
            Ok(p) => Some(p),
            Err(e) => {
                warn!("{}. Skipping external segment", e);
                None
            }
        },
        None => Some(base.clone()),
    };
}

fn try_chapters(
    mi: &mut MediaInfo,
    cache: &CacheMatroska,
    base: &Path,
    i_matroska_chapters: Option<usize>,
    base_dir: &Path,
) -> Result<Vec<RetimingChapter>> {
    let chapters: Vec<RetimingChapter> = if let Some(i) = i_matroska_chapters {
        let mat = cache.immut(base).unwrap(); // safe: earlier cached
        let cs = &mat.chapters[i].chapters;
        let len = cs.len();

        let mut chapters: Vec<RetimingChapter> = Vec::with_capacity(len);
        let mut push = |c: &matroska::Chapter, start, end| {
            chapters.push(RetimingChapter {
                start,
                end,
                uid: c.segment_uid.clone(),
                title: c.display.get(0).map(|t| t.string.clone()),
            })
        };

        for i in 0..len {
            let c = &cs[i];
            let uid = &c.segment_uid;

            if let Some(end) = c.time_end {
                push(c, c.time_start.into(), end.into());
                continue;
            }

            if let Some(j) = ((i + 1)..len).find(|j| uid == &cs[*j].segment_uid) {
                let end = cs[j].time_start;
                push(c, c.time_start.into(), end.into());
                continue;
            }

            let duration = match uid {
                Some(u) => {
                    let src = find_external_segment(mi, cache, base_dir, u)?;
                    *mi.try_get(MIPlayableDuration, &src)?
                }
                None => *mi.try_get(MIPlayableDuration, &base)?,
            };
            push(c, c.time_start.into(), duration);
        }

        chapters
    } else {
        let ictx = ffmpeg::format::input(base)?;
        ictx.chapters()
            .map(|c| {
                let tb = helpers::rational_as_f64(c.time_base());
                RetimingChapter {
                    start: Duration::from_secs_f64(c.start() as f64 * tb),
                    end: Duration::from_secs_f64(c.end() as f64 * tb),
                    uid: None,
                    title: c.metadata().get("title").map(|v| v.to_owned()),
                }
            })
            .collect()
    };

    Ok(chapters)
}

fn try_base(
    mi: &mut MediaInfo,
    order: &StreamsOrder,
    cache: &mut CacheMatroska,
) -> Result<(ArcPathBuf, usize, Option<usize>)> {
    for m in order.iter() {
        if !m.ty.is_video() {
            break;
        }
        let mat = some_or!(cache.get(&m.key), continue);

        if let Some(i) = mat.chapters.iter().enumerate().find_map(|(i, chp)| {
            chp.chapters
                .iter()
                .any(|c| c.segment_uid.is_some())
                .then(|| i)
        }) {
            return Ok((m.key.clone(), m.i_stream, Some(i)));
        }
    }

    if !mi.cfg.retiming_options.is_has_parts_cfg() {
        return Err(MuxError::new_ok());
    }

    match order.get(0).filter(|m| m.ty.is_video()) {
        Some(m) => Ok((m.key.clone(), m.i_stream, None)),
        None => Err(MuxError::new_ok()),
    }
}
