use super::{Retiming, RetimingChapter, RetimingPart};
use crate::{
    ArcPathBuf, Duration, IsDefault, MediaInfo, MuxConfig, MuxError, Result, Tool, ToolPaths,
    Tools, TrackOrder, TrackType,
    markers::{MIMatroska, MIPlayableDuration, MITICodec, MIVideoDuration},
    mux_err,
};
use log::warn;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

impl Retiming<'_, '_> {
    pub(crate) fn try_new<'a, 'b>(
        mi: &'b mut MediaInfo<'a>,
        order: &TrackOrder,
    ) -> Result<Retiming<'a, 'b>> {
        let (base, track, chapters_i, is_linked) = try_base_track_chapters_i_is_linked(mi, order)?;
        try_resolve_tool_paths(&mi.cfg.tool_paths, &mi.cfg.output.temp_dir)?;

        let base_dir = base.parent().unwrap_or(&mi.cfg.input.dir);
        let matroska = mi.try_take::<MIMatroska>(&base)?;
        let cs = &matroska.chapters[chapters_i].chapters;
        let len = cs.len();
        let cs = RetimingChapter::try_new_vec(mi, &base, base_dir, cs, len)?;
        mi.set::<MIMatroska>(&base, matroska);

        let mut parts: Vec<RetimingPart> = Vec::with_capacity(len);
        let mut i = 0;

        while i < len {
            let src = match save_then_src(mi, &base, &base_dir, &cs[i]) {
                Some(src) => src,
                None => {
                    i += 1;
                    continue;
                }
            };
            let i_end_chp = i_end_chp(&mi.cfg, &cs, i);

            let (start, start_offset, end, end_offset, no_retiming) =
                try_times(mi, &src, track, cs[i].start, cs[i_end_chp].end)?;

            parts.push(RetimingPart {
                i_start_chp: i,
                i_end_chp,
                src,
                no_retiming,
                start,
                end,
                start_offset,
                end_offset,
            });

            i += 1 + i_end_chp - i;
        }

        if !is_linked && parts.len() == 1 && parts[0].no_retiming {
            return Err(MuxError::new_ok());
        }

        cache_sub_codecs(mi, order);

        return Ok(Retiming {
            tools: Tools::from(mi.tools.paths),
            temp_dir: &mi.cfg.output.temp_dir,
            media_info: mi,
            thread: mi.thread,
            base,
            track,
            chapters: cs,
            parts,
        });

        fn save_then_src(
            mi: &MediaInfo,
            base: &ArcPathBuf,
            base_dir: &Path,
            c: &RetimingChapter,
        ) -> Option<ArcPathBuf> {
            if !save_chapter(&mi.cfg, c) {
                return None;
            }

            match c.uid.as_ref() {
                Some(u) => match find_external_segment(mi, base_dir, &u) {
                    Ok(p) => Some(p),
                    Err(e) => {
                        warn!("{}. Skipping external segment", e);
                        None
                    }
                },
                None => Some(base.clone()),
            }
        }

        fn i_end_chp(cfg: &MuxConfig, cs: &Vec<RetimingChapter>, mut i: usize) -> usize {
            let uid = &cs[i].uid;
            for idx in (i + 1)..cs.len() {
                let eq_uid = uid == &cs[idx].uid;
                let save = save_chapter(cfg, &cs[idx]);

                if (eq_uid && save) || (!eq_uid && !save) {
                    i = idx;
                } else {
                    break;
                }
            }
            i
        }

        fn save_chapter(cfg: &MuxConfig, c: &RetimingChapter) -> bool {
            if cfg.retiming.no_linked && c.uid.is_some() {
                return false;
            }

            let rm = cfg.retiming.rm_segments.as_ref();
            let d = &c.display;
            if rm.map_or(false, |rm| d.iter().any(|s| rm.is_match(s))) {
                return false;
            }

            true
        }

        fn try_resolve_tool_paths(ps: &ToolPaths, temp_dir: &Path) -> Result<()> {
            ps.try_resolve(Tool::Ffmpeg, temp_dir)?;
            ps.try_resolve(Tool::Ffprobe, temp_dir)
        }

        fn cache_sub_codecs(mi: &mut MediaInfo, order: &TrackOrder) {
            order.iter().filter(|m| m.ty.is_sub()).for_each(|m| {
                let _ = mi.init_ti::<MITICodec>(&m.media, m.track);
            })
        }

        fn try_base_track_chapters_i_is_linked(
            mi: &mut MediaInfo,
            order: &TrackOrder,
        ) -> Result<(ArcPathBuf, u64, usize, bool)> {
            if let Some(ok) = find_map(mi, order, true, |cs| {
                cs.iter().enumerate().find_map(|(i, c)| {
                    c.chapters
                        .iter()
                        .any(|c| c.segment_uid.is_some())
                        .then(|| i)
                })
            }) {
                return Ok(ok);
            }

            if !mi.cfg.retiming.is_default() {
                if let Some(ok) = find_map(mi, order, false, |cs| cs.iter().next().map(|_| 0)) {
                    return Ok(ok);
                }
            }

            return Err(MuxError::new_ok());

            fn find_map<F>(
                mi: &mut MediaInfo,
                order: &TrackOrder,
                is_linked: bool,
                mut f: F,
            ) -> Option<(ArcPathBuf, u64, usize, bool)>
            where
                F: FnMut(&Vec<matroska::ChapterEdition>) -> Option<usize>,
            {
                for m in order.iter() {
                    if !matches!(m.ty, TrackType::Video) {
                        break;
                    }
                    if let Some(mat) = mi.get::<MIMatroska>(&m.media) {
                        if let Some(i) = f(&mat.chapters) {
                            return Some((m.media.clone(), m.track, i, is_linked));
                        }
                    }
                }
                None
            }
        }

        fn try_times(
            mi: &mut MediaInfo,
            src: &Path,
            tid: u64,
            start: Duration,
            end: Duration,
        ) -> Result<(Duration, f64, Duration, f64, bool)> {
            const ACCEPT_VIDEO_OFFSET: f64 = 10.0; // seconds

            let duration = *mi.try_get::<MIVideoDuration>(src)?;

            let start_offset = start.as_secs_f64();
            let end_offset = duration.as_secs_f64() - end.as_secs_f64();

            if start_offset + end_offset.abs() < ACCEPT_VIDEO_OFFSET {
                return Ok((
                    Duration::default(),
                    start_offset,
                    duration,
                    end_offset,
                    true,
                ));
            }

            let (start, start_offset) = try_nearest_time_offset(mi, src, tid, start, duration)?;
            let (end, end_offset) = try_nearest_time_offset(mi, src, tid, end, duration)?;

            Ok((start, start_offset, end, end_offset, false))
        }

        fn try_nearest_time_offset(
            mi: &MediaInfo,
            src: &Path,
            tid: u64,
            target: Duration,
            duration: Duration,
        ) -> Result<(Duration, f64)> {
            let t = target.as_secs_f64();
            let first = try_i_frame(mi, src, tid, target)?;
            let second = {
                let t = Duration::from_secs_f64(t + t - first.as_secs_f64());
                try_i_frame(mi, src, tid, t)?
            };

            // unwraps safe
            let (nearest, offset) = [first, second, duration]
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

        fn try_i_frame(
            mi: &MediaInfo,
            media: &Path,
            tid: u64,
            target: Duration,
        ) -> Result<Duration> {
            let p: fn(&str) -> &Path = Path::new;

            let args = [
                p("-select_streams"),
                &PathBuf::from(format!("v:{}", tid)),
                p("-read_intervals"),
                &PathBuf::from(format!("{}%+0.000001", target.as_secs())),
                p("-show_entries"),
                p("frame=pict_type,pts_time"),
                p("-of"),
                p("csv"),
                media,
            ];

            mi.tools
                .run(Tool::Ffprobe, args)
                .ok()
                .and_then(|out| {
                    out.as_str_stdout()
                        .splitn(3, ',')
                        .skip(1)
                        .next()
                        .and_then(|secs| {
                            secs.parse::<f64>().ok().map(|s| Duration::from_secs_f64(s))
                        })
                })
                .ok_or_else(|| mux_err!("Not found I frame"))
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
                        let m = find_external_segment(mi, base_dir, u)?;
                        *mi.try_get::<MIPlayableDuration>(&m)?
                    }
                    None => *mi.try_get::<MIPlayableDuration>(&base)?,
                };

                Ok(Self::new(c, c.time_start.into(), duration))
            })
            .collect();
    }
}

static EXTERNAL_SEGMENTS: LazyLock<RwLock<ExternalSegments>> =
    LazyLock::new(|| RwLock::new(ExternalSegments::default()));

#[derive(Clone, Debug, Default)]
struct ExternalSegments {
    pub map: HashMap<Box<[u8]>, ArcPathBuf>,
    pub dir_set: HashSet<PathBuf>,
}

fn find_external_segment(mi: &MediaInfo, dir: &Path, uid: &[u8]) -> Result<ArcPathBuf> {
    if let Some(res) = get_cached(dir, uid) {
        return res;
    }
    insert_all_in_dir(mi, dir);
    return get_cached(dir, uid).unwrap();

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

    fn insert_all_in_dir(mi: &MediaInfo, dir: &Path) {
        mi.cfg
            .input
            .iter_matroska_in_dir(dir)
            .par_bridge()
            .for_each(|m| {
                if let Some(u) = match mi.immut::<MIMatroska>(&m) {
                    Some(mat) => mat.info.uid.clone(),
                    None => MediaInfo::help_build_matroska(&m).map_or(None, |mat| mat.info.uid),
                } {
                    let mut es = EXTERNAL_SEGMENTS.write().unwrap();
                    es.map.insert(u.into(), m.into());
                }
            });

        let mut es = EXTERNAL_SEGMENTS.write().unwrap();
        es.dir_set.insert(dir.to_owned());
    }

    fn error(dir: &Path, uid: &[u8]) -> MuxError {
        mux_err!(
            "Not found external matroska segment '{:?}' in the directory '{}'",
            uid,
            dir.display()
        )
    }
}
