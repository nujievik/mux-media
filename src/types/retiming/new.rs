mod cache;
mod external_segments;

use super::*;
use crate::media_info::*;
use crate::{
    ArcPathBuf, Config, MediaInfo, MuxError, Result, StreamType, StreamsOrder, Time, ffmpeg,
    types::helpers,
};
use cache::CacheMatroska;
use external_segments::find_external_segment;
use is_default::IsDefault;
use log::warn;
use std::{collections::HashMap, path::Path};

impl Retiming<'_, '_> {
    pub(crate) fn try_new<'a, 'b>(
        mi: &'b mut MediaInfo<'a>,
        order: &StreamsOrder,
    ) -> Result<Retiming<'a, 'b>> {
        let mut cache = CacheMatroska::default();
        let (base, i_base_stream, i_matroska_chapters) = try_base(mi, order, &mut cache)?;
        let base_dir = base.parent().unwrap_or(mi.cfg.input.dir());

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
            temp_dir: &mi.cfg.output.temp_dir(),
            media_info: mi,
            job: mi.job,
            base,
            i_base_stream,
            chapters: cs,
            parts,
            base_splits: Vec::new(),
        };
        rtm.init_base_splits()?;

        Ok(rtm)
    }
}

fn try_times(
    mi: &mut MediaInfo,
    src: &Path,
    i_stream: usize,
    start: Time,
    end: Time,
) -> Result<(Time, SignedTime, Time, SignedTime)> {
    const ACCEPT_VIDEO_OFFSET: Time = Time::from_secs(10);

    let duration = *mi.try_get(MarkMediaInfoVideoDuration, src)?;

    let (start, start_offset) = if start < ACCEPT_VIDEO_OFFSET {
        (Time::ZERO, SignedTime::new(true, start))
    } else {
        try_nearest_time_offset(src, i_stream, start, duration)?
    };

    let duration_offset = SignedTime::new(true, duration) - end;

    let (end, end_offset) = if duration_offset.as_unsigned_time() < ACCEPT_VIDEO_OFFSET {
        (duration, duration_offset)
    } else {
        try_nearest_time_offset(src, i_stream, end, duration)?
    };

    return Ok((start, start_offset, end, end_offset));

    fn try_nearest_time_offset(
        src: &Path,
        i_stream: usize,
        target: Time,
        duration: Time,
    ) -> Result<(Time, SignedTime)> {
        let first = try_i_frame(src, i_stream, target)?;
        let first_offset = SignedTime::new(true, target) - first;

        let second = {
            let offset = first_offset.as_unsigned_time().as_millis() / 2;
            let offset = SignedTime::new(first_offset.is_positive(), Time::from_millis(offset));
            let trg = (offset + target).as_unsigned_time();
            try_i_frame(src, i_stream, trg)?
        };
        let third = try_i_frame(src, i_stream, (first_offset + target).as_unsigned_time())?;

        // unwraps safe
        let (nearest, offset) = [first, second, third, duration]
            .into_iter()
            .map(|time| {
                let offset = SignedTime::new(true, target) - time;
                (time, offset, offset.as_unsigned_time())
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            .map(|t| (t.0, t.1))
            .unwrap();

        Ok((nearest, offset))
    }

    fn try_i_frame(src: &Path, i_stream: usize, target: Time) -> Result<Time> {
        let mut ictx = ffmpeg::format::input(src)?;
        let stream = ictx.stream(i_stream).ok_or(ffmpeg::Error::StreamNotFound)?;
        let (i, _) = helpers::ffmpeg_stream_i_tb(&stream);
        let ist_time_base = stream.time_base();

        let mut opened = helpers::try_ffmpeg_opened(StreamType::Video, &stream)?;
        let seek_target = (target.as_millis() as i64).rescale(
            MILLISECOND_TIME_BASE,
            Rational(1, ffmpeg::ffi::AV_TIME_BASE),
        );

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
                    Ok(_) => return Ok(ts_to_time(frame.pts().unwrap_or(0), ist_time_base)),
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
        let save = cfg.retiming.is_save_chapter(&cs[j]);

        if eq_uid && save {
            i = j;
        } else if !eq_uid && !save {
            continue;
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
    if !mi.cfg.retiming.is_save_chapter(c) {
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
                    *mi.try_get(MarkMediaInfoPlayableDuration, &src)?
                }
                None => *mi.try_get(MarkMediaInfoPlayableDuration, &base)?,
            };
            push(c, c.time_start.into(), duration);
        }

        chapters
    } else {
        let ictx = ffmpeg::format::input(base)?;
        ictx.chapters()
            .map(|c| RetimingChapter {
                start: ts_to_time(c.start(), c.time_base()),
                end: ts_to_time(c.end(), c.time_base()),
                uid: None,
                title: c.metadata().get("title").map(|v| v.to_owned()),
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

    if mi.cfg.retiming.parts.is_default() {
        return Err(MuxError::new_ok());
    }

    match order.get(0).filter(|m| m.ty.is_video()) {
        Some(m) => Ok((m.key.clone(), m.i_stream, None)),
        None => Err(MuxError::new_ok()),
    }
}
