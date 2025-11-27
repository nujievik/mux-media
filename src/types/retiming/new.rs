use super::{Retiming, RetimingChapter, RetimingPart};
use crate::{
    ArcPathBuf, Config, Duration, IsDefault, MediaInfo, MuxError, Result, StreamType, StreamsOrder,
    ffmpeg, markers::*, types::helpers,
};
use log::warn;
use matroska::Matroska;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

impl Retiming<'_, '_> {
    pub(crate) fn try_new<'a, 'b>(
        mi: &'b mut MediaInfo<'a>,
        order: &StreamsOrder,
    ) -> Result<Retiming<'a, 'b>> {
        let mut cache = CacheMatroska::default();
        let (base, i_base_stream, i_chapters, _) =
            try_base_i_stream_i_chapters_is_linked(mi, order, &mut cache)?;

        let base_dir = base.parent().unwrap_or(&mi.cfg.input.dir);
        let matroska = cache.immut(&base).unwrap();
        let cs = &matroska.chapters[i_chapters].chapters;
        let len = cs.len();
        let cs = RetimingChapter::try_new_vec(mi, &cache, &base, base_dir, cs, len)?;

        let mut parts: Vec<RetimingPart> = Vec::with_capacity(len);
        let mut i = 0;

        while i < len {
            let src = match save_then_src(mi, &cache, &base, &base_dir, &cs[i]) {
                Some(src) => src,
                None => {
                    i += 1;
                    continue;
                }
            };
            let i_end_chp = i_end_chp(&mi.cfg, &cs, i);

            let (start, start_offset, end, end_offset, _no_retiming) =
                try_times(mi, &src, i_base_stream, cs[i].start, cs[i_end_chp].end)?;

            parts.push(RetimingPart {
                i_start_chp: i,
                i_end_chp,
                src,
                _no_retiming,
                start,
                start_offset,
                end,
                end_offset,
            });

            i += 1 + i_end_chp - i;
        }

        let mut rtm = Retiming {
            tools: mi.cfg.into(),
            temp_dir: &mi.cfg.output.temp_dir,
            media_info: mi,
            thread: mi.thread,
            base,
            i_base_stream,
            chapters: cs,
            parts,
            base_splits: Vec::new(),
        };

        rtm.init_base_splits()?;
        return Ok(rtm);

        fn save_then_src(
            mi: &MediaInfo,
            cache: &CacheMatroska,
            base: &ArcPathBuf,
            base_dir: &Path,
            c: &RetimingChapter,
        ) -> Option<ArcPathBuf> {
            if !is_save_chapter(&mi.cfg, c) {
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

        fn is_save_chapter(cfg: &Config, c: &RetimingChapter) -> bool {
            if cfg.retiming_options.no_linked && c.uid.is_some() {
                return false;
            }

            let rm = cfg.retiming_options.rm_segments.as_ref();
            let d = &c.display;
            if rm.map_or(false, |rm| d.iter().any(|s| rm.is_match(s))) {
                return false;
            }

            true
        }

        fn i_end_chp(cfg: &Config, cs: &Vec<RetimingChapter>, mut i: usize) -> usize {
            let uid = &cs[i].uid;
            for idx in (i + 1)..cs.len() {
                let eq_uid = uid == &cs[idx].uid;
                let save = is_save_chapter(cfg, &cs[idx]);

                if (eq_uid && save) || (!eq_uid && !save) {
                    i = idx;
                } else {
                    break;
                }
            }
            i
        }

        fn try_base_i_stream_i_chapters_is_linked(
            mi: &mut MediaInfo,
            order: &StreamsOrder,
            cache: &mut CacheMatroska,
        ) -> Result<(ArcPathBuf, usize, usize, bool)> {
            if let Some(ok) = find_map(order, cache, true, |cs| {
                cs.iter().enumerate().find_map(|(i, c)| {
                    c.chapters
                        .iter()
                        .any(|c| c.segment_uid.is_some())
                        .then(|| i)
                })
            }) {
                return Ok(ok);
            }

            if !mi.cfg.retiming_options.is_default() {
                if let Some(ok) = find_map(order, cache, false, |cs| cs.iter().next().map(|_| 0)) {
                    return Ok(ok);
                }
            }

            return Err(MuxError::new_ok());

            fn find_map<F>(
                order: &StreamsOrder,
                cache: &mut CacheMatroska,
                is_linked: bool,
                mut f: F,
            ) -> Option<(ArcPathBuf, usize, usize, bool)>
            where
                F: FnMut(&Vec<matroska::ChapterEdition>) -> Option<usize>,
            {
                for m in order.iter() {
                    // always video streams is first.
                    if !m.ty.is_video() {
                        break;
                    }
                    if let Some(mat) = cache.get(&m.key) {
                        if let Some(i) = f(&mat.chapters) {
                            return Some((m.key.clone(), m.i_stream, i, is_linked));
                        }
                    }
                }
                None
            }
        }

        fn try_times(
            mi: &mut MediaInfo,
            src: &Path,
            i_stream: usize,
            start: Duration,
            end: Duration,
        ) -> Result<(Duration, f64, Duration, f64, bool)> {
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

            Ok((start, start_offset, end, end_offset, false))
        }

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
}

impl RetimingChapter {
    fn new(c: &matroska::Chapter, start: Duration, end: Duration) -> RetimingChapter {
        RetimingChapter {
            start,
            end,
            uid: c.segment_uid.clone(),
            display: c.display.iter().map(|d| d.string.clone()).collect(),
        }
    }

    fn try_new_vec(
        mi: &mut MediaInfo,
        cache: &CacheMatroska,
        base: &Path,
        base_dir: &Path,
        cs: &Vec<matroska::Chapter>,
        len: usize,
    ) -> Result<Vec<RetimingChapter>> {
        return (0..len)
            .map(|i| {
                let c = &cs[i];
                let uid = &c.segment_uid;

                if let Some(end) = c.time_end {
                    return Ok(Self::new(c, c.time_start.into(), end.into()));
                }

                if let Some(i_next) = ((i + 1)..len).find(|idx| uid == &cs[*idx].segment_uid) {
                    let end = cs[i_next].time_start;
                    return Ok(Self::new(c, c.time_start.into(), end.into()));
                }

                let duration = match uid {
                    Some(u) => {
                        let src = find_external_segment(mi, cache, base_dir, u)?;
                        *mi.try_get(MIPlayableDuration, &src)?
                    }
                    None => *mi.try_get(MIPlayableDuration, &base)?,
                };

                Ok(Self::new(c, c.time_start.into(), duration))
            })
            .collect();
    }
}

#[derive(Default)]
struct CacheMatroska(HashMap<ArcPathBuf, Option<Matroska>>);

impl CacheMatroska {
    fn get(&mut self, src: &ArcPathBuf) -> Option<&Matroska> {
        if self.0.get(src).is_none() {
            let v = matroska::open(src).ok();
            self.0.insert(src.clone(), v);
        }
        self.immut(src)
    }

    fn immut(&self, src: &Path) -> Option<&Matroska> {
        self.0.get(src).map_or(None, |v| v.as_ref())
    }
}

static EXTERNAL_SEGMENTS: LazyLock<RwLock<ExternalSegments>> =
    LazyLock::new(|| RwLock::new(ExternalSegments::default()));

#[derive(Clone, Debug, Default)]
struct ExternalSegments {
    pub map: HashMap<Box<[u8]>, ArcPathBuf>,
    pub dir_set: HashSet<PathBuf>,
}

fn find_external_segment(
    mi: &MediaInfo,
    cache: &CacheMatroska,
    dir: &Path,
    uid: &[u8],
) -> Result<ArcPathBuf> {
    return if let Some(res) = get_cached(dir, uid) {
        res
    } else {
        insert_all_in_dir(mi, cache, dir);
        get_cached(dir, uid).unwrap()
    };

    fn get_cached(dir: &Path, uid: &[u8]) -> Option<Result<ArcPathBuf>> {
        let es = EXTERNAL_SEGMENTS.read().unwrap();

        if let Some(p) = es.map.get(uid) {
            Some(Ok(p.clone()))
        } else if es.dir_set.contains(dir) {
            Some(Err(error(dir, uid)))
        } else {
            None
        }
    }

    fn insert_all_in_dir(mi: &MediaInfo, cache: &CacheMatroska, dir: &Path) {
        mi.cfg
            .input
            .iter_matroska_in_dir(dir)
            .par_bridge()
            .for_each(|m| {
                if let Some(u) = match cache.immut(&m) {
                    Some(mat) => mat.info.uid.clone(),
                    None => matroska::open(&m).ok().map_or(None, |mat| mat.info.uid),
                } {
                    let mut es = EXTERNAL_SEGMENTS.write().unwrap();
                    es.map.insert(u.into(), m.into());
                }
            });

        let mut es = EXTERNAL_SEGMENTS.write().unwrap();
        es.dir_set.insert(dir.to_owned());
    }

    fn error(dir: &Path, uid: &[u8]) -> MuxError {
        err!(
            "Not found external matroska segment '{:?}' in the directory '{}'",
            uid,
            dir.display()
        )
    }
}
