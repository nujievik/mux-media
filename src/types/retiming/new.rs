use super::{Retiming, RetimingChapter, RetimingPart};
use crate::{
    ArcPathBuf, Duration, MediaInfo, MuxError, Result, Tool, Tools, TrackOrder, TrackType,
    markers::{MIMatroska, MIPlayableDuration, MITICodec},
    mux_err,
};
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(crate) fn try_new<'a, 'b>(
        mi: &'b mut MediaInfo<'a>,
        order: &TrackOrder,
    ) -> Result<Retiming<'a, 'b>> {
        let (base, track, chapters_i) = try_base_track_chapters_i(mi, order)?;
        let base_dir = base.parent().unwrap_or(&mi.cfg.input.dir);

        let matroska = mi.try_take::<MIMatroska>(&base)?;
        let cs = &matroska.chapters[chapters_i].chapters;
        let len = cs.len();
        let chapters = RetimingChapter::try_new_vec(mi, &base, base_dir, cs, len)?;
        mi.set::<MIMatroska>(&base, matroska);

        let mut parts: Vec<RetimingPart> = Vec::new();
        let mut i = 0;

        while i < len {
            let uid = &chapters[i].uid;
            let external_src = uid
                .as_ref()
                .and_then(|u| mi.find_external_segment(base_dir, &u));

            let mut i_end = i;
            for idx in (i + 1)..len {
                if uid == &chapters[idx].uid {
                    i_end = idx;
                } else {
                    break;
                }
            }

            let src = external_src.as_ref().unwrap_or(&base);
            let (start, start_offset) = try_nearest_time_offset(mi, src, track, chapters[i].start)?;
            let (end, end_offset) = try_nearest_time_offset(mi, src, track, chapters[i_end].end)?;

            parts.push(RetimingPart {
                i_start_chp: i,
                i_end_chp: i_end,
                external_src,
                start,
                end,
                start_offset,
                end_offset,
            });

            i += 1 + i_end - i;
        }

        cache_sub_codecs(mi, order);

        return Ok(Retiming {
            tools: Tools::from(mi.tools.paths),
            temp_dir: &mi.cfg.output.temp_dir,
            media_info: mi,
            thread: mi.thread,
            base,
            track,
            chapters,
            parts,
        });

        fn cache_sub_codecs(mi: &mut MediaInfo, order: &TrackOrder) {
            order.iter().filter(|m| m.ty.is_sub()).for_each(|m| {
                let _ = mi.init_ti::<MITICodec>(&m.media, m.track);
            })
        }

        fn try_base_track_chapters_i(
            mi: &mut MediaInfo,
            order: &TrackOrder,
        ) -> Result<(ArcPathBuf, u64, usize)> {
            for m in order.iter() {
                if !matches!(m.ty, TrackType::Video) {
                    break;
                }
                let opt = mi.get::<MIMatroska>(&m.media).and_then(|mat| {
                    mat.chapters.iter().enumerate().find_map(|(i, chp)| {
                        chp.chapters
                            .iter()
                            .any(|c| c.segment_uid.is_some())
                            .then(|| (m.media.clone(), m.track, i))
                    })
                });
                if let Some(t) = opt {
                    return Ok(t);
                }
            }
            Err(MuxError::new_ok().message("Not found any linked video"))
        }

        fn try_nearest_time_offset(
            mi: &mut MediaInfo,
            src: &Path,
            tid: u64,
            target: Duration,
        ) -> Result<(Duration, f64)> {
            const ACCEPT_VIDEO_OFFSET: f64 = 10.0; // seconds

            let duration = *mi.try_get::<MIPlayableDuration>(src)?;

            let offset_duration = duration.as_secs_f64() - target.as_secs_f64();
            if offset_duration.abs() <= ACCEPT_VIDEO_OFFSET {
                return Ok((duration, offset_duration));
            }

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
    fn new(start: Duration, end: Duration, uid: Option<Vec<u8>>) -> RetimingChapter {
        RetimingChapter { start, end, uid }
    }

    fn try_new_vec(
        mi: &mut MediaInfo,
        base: &Path,
        base_dir: &Path,
        cs: &[matroska::Chapter],
        len: usize,
    ) -> Result<Vec<RetimingChapter>> {
        (0..len)
            .map(|i| {
                let uid = &cs[i].segment_uid;

                if let Some(end) = cs[i].time_end {
                    return Ok(Self::new(cs[i].time_start.into(), end.into(), uid.clone()));
                }

                if let Some(i_next) = ((i + 1)..len).find(|idx| uid == &cs[*idx].segment_uid) {
                    let end = cs[i_next].time_start;
                    return Ok(Self::new(cs[i].time_start.into(), end.into(), uid.clone()));
                }

                let duration = match uid {
                    Some(u) => {
                        let m = mi
                            .find_external_segment(base_dir, u)
                            .ok_or_else(|| "Not found external src")?;
                        *mi.try_get::<MIPlayableDuration>(&m)?
                    }
                    None => *mi.try_get::<MIPlayableDuration>(&base)?,
                };

                Ok(Self::new(cs[i].time_start.into(), duration, uid.clone()))
            })
            .collect()
    }
}
