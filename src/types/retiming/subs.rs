use super::Retiming;
use crate::{
    Duration, MediaInfo, Result, Tool, Tools, TrackOrderItemRetimed, TrackType, markers::MITICodec,
};
use log::warn;
use rsubs_lib::{SRT, SRTLine, SSA, SSAEvent, VTT, VTTLine};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Duration as StdDuration,
};
use time::Time;

impl Retiming<'_, '_> {
    pub(crate) fn try_sub(
        &self,
        i: usize,
        src: &Path,
        track: u64,
    ) -> Result<TrackOrderItemRetimed> {
        if self.parts.len() == 1 && src == **self.base && self.parts[0].no_retiming {
            return Ok(TrackOrderItemRetimed::new(
                vec![src.into()],
                true,
                TrackType::Sub,
            ));
        }

        let fall = |dest: &mut PathBuf, ty, err| -> Result<()> {
            if matches!(ty, SubType::Srt) {
                return Err(err);
            }
            warn!(
                "Fail retiming '{}' track {} as .{}: {}. Try retime as .srt",
                src.display(),
                track,
                ty.as_ext(),
                err
            );
            dest.set_extension("srt");
            Ok(())
        };

        let dest = if src == **self.base {
            let ty = SubType::from_codec_id(self.media_info, src, track);
            let mut dest = self.temp_dir.join(format!(
                "{}-sub-base-{}.{}",
                self.thread,
                track,
                ty.as_ext()
            ));
            if let Err(err) = self.try_base_sub(track, &dest) {
                fall(&mut dest, ty, err)?;
                self.try_base_sub(track, &dest)?;
            }
            dest
        } else {
            let ty = SubType::try_from_extension(src)
                .unwrap_or_else(|_| SubType::from_codec_id(self.media_info, src, track));
            let mut dest = self
                .temp_dir
                .join(format!("{}-sub-{}.{}", self.thread, i, ty.as_ext()));
            if let Err(err) = self.try_external_sub(src, &dest) {
                fall(&mut dest, ty, err)?;
                self.try_external_sub(src, &dest)?;
            }
            dest
        };

        Ok(TrackOrderItemRetimed::new(
            vec![dest],
            false,
            TrackType::Sub,
        ))
    }
}

macro_rules! map_internal_rsub {
    ($old_map:ident, $sub:ident, $new_ty:ty) => {
        $old_map
            .into_iter()
            .filter_map(|(k, v)| match v {
                RSub::$sub(old) => Some((k, old)),
                _ => {
                    eprintln!("Unexpected unmatch pattern");
                    None
                }
            })
            .collect::<HashMap<&Path, $new_ty>>()
    };
}

impl Retiming<'_, '_> {
    fn try_base_sub(&self, tid: u64, dest: &Path) -> Result<()> {
        let old = try_map_old(self, tid, &dest)?;

        let src_idxs_offset: Vec<_> = self
            .parts
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let src = p.src.as_path();
                get_idxs_offset(self, &old[src], i).map(|(xs, t)| (src, xs, t))
            })
            .collect();

        return if src_idxs_offset.is_empty() {
            Err(err!("Not saved any subtitle line"))
        } else {
            let new = retime(old, src_idxs_offset);
            new.try_write(dest)
        };

        fn retime(
            old: HashMap<&Path, RSub>,
            src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>,
        ) -> RSub {
            // unwrap safe, map is not empty
            return match old.values().next().unwrap() {
                RSub::Srt(_) => RSub::Srt(srt(old, src_idxs_offset)),
                RSub::Ssa(_) => RSub::Ssa(ssa(old, src_idxs_offset)),
                RSub::Vtt(_) => RSub::Vtt(vtt(old, src_idxs_offset)),
            };

            fn srt(
                old: HashMap<&Path, RSub>,
                src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>,
            ) -> SRT {
                let old = map_internal_rsub!(old, Srt, SRT);
                let mut lines: Vec<SRTLine> = Vec::with_capacity(capacity(&src_idxs_offset));
                let mut i = 1u32;
                for (src, idxs, offset) in src_idxs_offset {
                    push_retimed_srt_lines(&mut lines, &mut i, &old[src].lines, idxs, offset)
                }
                SRT { lines }
            }

            fn ssa(
                old: HashMap<&Path, RSub>,
                src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>,
            ) -> SSA {
                let old = map_internal_rsub!(old, Ssa, SSA);
                let mut events: Vec<SSAEvent> = Vec::with_capacity(capacity(&src_idxs_offset));
                for (src, idxs, offset) in src_idxs_offset {
                    push_retimed_ssa_events(&mut events, &old[src].events, idxs, offset)
                }
                // unwrap safe, map is not empty.
                let mut new = old.into_values().next().unwrap();
                new.events = events;
                new
            }

            fn vtt(
                old: HashMap<&Path, RSub>,
                src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>,
            ) -> VTT {
                let old = map_internal_rsub!(old, Vtt, VTT);
                let mut lines: Vec<VTTLine> = Vec::with_capacity(capacity(&src_idxs_offset));
                for (src, idxs, offset) in src_idxs_offset {
                    push_retimed_vtt_lines(&mut lines, &old[src].lines, idxs, offset);
                }
                // unwrap safe, map is not empty.
                let mut new = old.into_values().next().unwrap();
                new.lines = lines;
                new
            }

            fn capacity(src_idxs_offset: &Vec<(&Path, Vec<usize>, f64)>) -> usize {
                src_idxs_offset.iter().map(|t| t.1.len()).sum()
            }
        }

        fn get_idxs_offset(
            rtm: &Retiming<'_, '_>,
            old: &RSub,
            i_part: usize,
        ) -> Option<(Vec<usize>, f64)> {
            let p = &rtm.parts[i_part];
            let idxs = save_idxs(old, p.start.into(), p.end.into());
            let offset = rtm.parts_nonuid(i_part);
            Some((idxs, offset))
        }

        fn try_map_old<'a>(
            rtm: &'a Retiming<'_, '_>,
            track: u64,
            dest: &Path,
        ) -> Result<HashMap<&'a Path, RSub>> {
            let mut map: HashMap<&Path, RSub> = HashMap::new();
            for p in rtm.parts.iter() {
                let src = p.src.as_path();
                if !map.contains_key(&src) {
                    let _ = fs::remove_file(dest);
                    try_extract(&rtm.tools, src, track, dest)?;
                    let sub = RSub::try_from_file(dest)?;
                    map.insert(src, sub);
                }
            }
            return Ok(map);

            fn try_extract(tools: &Tools<'_>, src: &Path, track: u64, dest: &Path) -> Result<()> {
                let p: fn(&str) -> &Path = Path::new;
                let map = format!("0:{}", track);
                let args = [p("-i"), src, p("-map"), p(&map), dest];
                tools.run(Tool::Ffmpeg, &args).map(|_| ())
            }
        }
    }

    fn try_external_sub(&self, src: &Path, dest: &Path) -> Result<()> {
        let old = RSub::try_from_file(src)?;
        let mut idxs_offset: Vec<(Vec<usize>, f64)> = Vec::with_capacity(self.chapters.len());

        for (i_part, p) in self.parts.iter().enumerate() {
            for i_chp in p.i_start_chp..=p.i_end_chp {
                if let Some((idxs, offset)) = get_idxs_offset(self, &old, i_chp, i_part) {
                    idxs_offset.push((idxs, offset))
                }
            }
        }

        return if idxs_offset.is_empty() {
            Err(err!("Not saved any subtitle line"))
        } else {
            let new = retime(old, idxs_offset);
            new.try_write(dest)
        };

        fn retime(old: RSub, idxs_offset: Vec<(Vec<usize>, f64)>) -> RSub {
            return match old {
                RSub::Srt(old) => RSub::Srt(srt(old, idxs_offset)),
                RSub::Ssa(old) => RSub::Ssa(ssa(old, idxs_offset)),
                RSub::Vtt(old) => RSub::Vtt(vtt(old, idxs_offset)),
            };

            fn srt(old: SRT, idxs_offset: Vec<(Vec<usize>, f64)>) -> SRT {
                let mut lines: Vec<SRTLine> = Vec::with_capacity(capacity(&idxs_offset));
                let mut i = 1u32;
                for (idxs, offset) in idxs_offset {
                    push_retimed_srt_lines(&mut lines, &mut i, &old.lines, idxs, offset)
                }
                SRT { lines }
            }

            fn ssa(mut old: SSA, idxs_offset: Vec<(Vec<usize>, f64)>) -> SSA {
                let mut events: Vec<SSAEvent> = Vec::with_capacity(capacity(&idxs_offset));
                for (idxs, offset) in idxs_offset {
                    push_retimed_ssa_events(&mut events, &old.events, idxs, offset)
                }
                old.events = events;
                old
            }

            fn vtt(mut old: VTT, idxs_offset: Vec<(Vec<usize>, f64)>) -> VTT {
                let mut lines: Vec<VTTLine> = Vec::with_capacity(capacity(&idxs_offset));
                for (idxs, offset) in idxs_offset {
                    push_retimed_vtt_lines(&mut lines, &old.lines, idxs, offset);
                }
                old.lines = lines;
                old
            }

            fn capacity(idxs_offset: &Vec<(Vec<usize>, f64)>) -> usize {
                idxs_offset.iter().map(|t| t.0.len()).sum()
            }
        }

        fn get_idxs_offset(
            rtm: &Retiming<'_, '_>,
            old: &RSub,
            i_chp: usize,
            i_part: usize,
        ) -> Option<(Vec<usize>, f64)> {
            let p = &rtm.parts[i_part];
            let uid = &rtm.chapters[p.i_start_chp].uid;
            let chp = &rtm.chapters[i_chp];

            if uid != &chp.uid {
                return None;
            }

            let chp_nonuid = rtm.chapters_nonuid(i_chp);

            let trg_start =
                Duration::from_secs_f64(chp.start.as_secs_f64() + p.start_offset + chp_nonuid);
            let end_offset = if i_chp == p.i_end_chp {
                p.end_offset
            } else {
                p.start_offset
            };
            let trg_end = Duration::from_secs_f64(chp.end.as_secs_f64() + end_offset + chp_nonuid);

            let idxs = save_idxs(old, trg_start.into(), trg_end.into());

            if idxs.is_empty() {
                None
            } else {
                let offset = rtm.parts_nonuid(i_part) - chp_nonuid;
                Some((idxs, offset))
            }
        }
    }
}

fn save_idxs(rsub: &RSub, trg_start: Time, trg_end: Time) -> Vec<usize> {
    rsub.iter_i_start_end()
        .filter_map(|(i, start, end)| {
            if start > trg_end || end < trg_start {
                None
            } else {
                Some(i)
            }
        })
        .collect()
}

fn push_retimed_srt_lines(
    lines: &mut Vec<SRTLine>,
    sequence_number: &mut u32,
    old: &Vec<SRTLine>,
    idxs: Vec<usize>,
    offset: f64,
) {
    let (sign, offset) = sign_duration(offset);
    for i in idxs {
        let (start, end) = retime_start_end(old[i].start, old[i].end, sign, offset);
        lines.push(SRTLine {
            sequence_number: *sequence_number,
            start,
            end,
            text: old[i].text.clone(),
        });
        *sequence_number += 1;
    }
}

fn push_retimed_ssa_events(
    events: &mut Vec<SSAEvent>,
    old: &Vec<SSAEvent>,
    idxs: Vec<usize>,
    offset: f64,
) {
    let (sign, offset) = sign_duration(offset);
    for i in idxs {
        let (start, end) = retime_start_end(old[i].start, old[i].end, sign, offset);
        events.push(SSAEvent {
            layer: old[i].layer,
            start,
            end,
            style: old[i].style.clone(),
            name: old[i].name.clone(),
            margin_l: old[i].margin_l,
            margin_r: old[i].margin_r,
            margin_v: old[i].margin_v,
            effect: old[i].effect.clone(),
            text: old[i].text.clone(),
            line_type: old[i].line_type.clone(),
        });
    }
}

fn push_retimed_vtt_lines(
    lines: &mut Vec<VTTLine>,
    old: &Vec<VTTLine>,
    idxs: Vec<usize>,
    offset: f64,
) {
    let (sign, offset) = sign_duration(offset);
    for i in idxs {
        let (start, end) = retime_start_end(old[i].start, old[i].end, sign, offset);
        lines.push(VTTLine {
            identifier: old[i].identifier.clone(),
            start,
            end,
            settings: old[i].settings.clone(),
            text: old[i].text.clone(),
        });
    }
}

fn retime_start_end(start: Time, end: Time, sign: bool, offset: StdDuration) -> (Time, Time) {
    if sign {
        (start + offset, end + offset)
    } else {
        (start - offset, end - offset)
    }
}

// true is sign positive
fn sign_duration(offset: f64) -> (bool, StdDuration) {
    (
        offset.is_sign_positive(),
        StdDuration::from_secs_f64(offset.abs()),
    )
}

#[derive(Copy, Clone, Debug)]
enum SubType {
    Srt,
    Ssa,
    Vtt,
}

impl SubType {
    fn try_from_extension(file: &Path) -> Result<SubType> {
        let ext = file
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| err!("Not found extension"))?;

        if ext.eq_ignore_ascii_case("ssa") || ext.eq_ignore_ascii_case("ass") {
            Ok(Self::Ssa)
        } else if ext.eq_ignore_ascii_case("srt") {
            Ok(Self::Srt)
        } else if ext.eq_ignore_ascii_case("vtt") {
            Ok(Self::Vtt)
        } else {
            Err(err!("Unsupported extension ({})", ext))
        }
    }

    fn from_codec_id(mi: &MediaInfo, file: &Path, track: u64) -> SubType {
        let c = mi
            .immut_ti::<MITICodec>(file, track)
            .map_or("", |c| c.as_str());
        match c {
            "SubStationAlpha" | "S_TEXT/ASS" => SubType::Ssa,
            "SubRip/SRT" | "S_TEXT/UTF8" => SubType::Srt,
            "WebVTT" | "S_TEXT/WEBVTT" => SubType::Vtt,
            _ => {
                warn!("Unsupported codec: {}. Try use .srt", c);
                SubType::Srt
            }
        }
    }

    const fn as_ext(self) -> &'static str {
        match self {
            Self::Srt => "srt",
            Self::Ssa => "ass",
            Self::Vtt => "vtt",
        }
    }
}

#[derive(Debug)]
enum RSub {
    Srt(SRT),
    Ssa(SSA),
    Vtt(VTT),
}

impl RSub {
    fn try_from_file(file: impl AsRef<Path>) -> Result<RSub> {
        let f = file.as_ref();
        let ty = SubType::try_from_extension(f)?;

        let s = fs::read_to_string(f)?;
        let s = s.trim_start();
        let s = s.strip_prefix('\u{feff}').unwrap_or(s);
        let s = s.trim_start();
        let s = s.trim_end();

        let rsub = match ty {
            SubType::Srt => RSub::Srt(SRT::parse(s)?),
            SubType::Ssa => RSub::Ssa(SSA::parse(s)?),
            SubType::Vtt => RSub::Vtt(VTT::parse(s)?),
        };
        Ok(rsub)
    }
}

macro_rules! box_iter {
    ($sub:ident, $field:ident) => {
        Box::new(
            $sub.$field
                .iter()
                .enumerate()
                .map(|(i, f)| (i, f.start.into(), f.end.into())),
        )
    };
}

impl RSub {
    fn iter_i_start_end(&self) -> Box<dyn Iterator<Item = (usize, Time, Time)> + '_> {
        match self {
            Self::Srt(sub) => box_iter!(sub, lines),
            Self::Ssa(sub) => box_iter!(sub, events),
            Self::Vtt(sub) => box_iter!(sub, lines),
        }
    }

    fn try_write(&self, dest: &Path) -> Result<()> {
        let mut file = fs::File::create(dest)?;
        match self {
            Self::Srt(s) => write!(file, "{}", s),
            Self::Ssa(s) => write!(file, "{}", s),
            Self::Vtt(s) => write!(file, "{}", s),
        }?;
        Ok(())
    }
}
