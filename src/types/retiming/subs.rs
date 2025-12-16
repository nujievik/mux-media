mod base;
mod destination;
mod external;
mod ty;
mod xs;

use super::{RetimedStream, Retiming};
use crate::{Duration, Result};
use destination::Destination;
use log::warn;
use rsubs_lib::{SRT, SRTLine, SSA, SSAEvent, VTT, VTTLine};
use std::{collections::HashMap, fs, path::Path, time::Duration as StdDuration};
use time::Time;
use ty::SubType;
use xs::Subs;

impl Retiming<'_, '_> {
    pub(crate) fn try_sub(&self, i: usize, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        let is_base = src == **self.base;

        if is_base && self.is_save_single_part() {
            return Ok(self.single_part_base_retimed_stream(src, i_stream));
        }

        let fall = |dest: &mut Destination, err| -> Result<()> {
            if matches!(dest.ty, SubType::Srt) {
                return Err(err);
            }
            warn!(
                "Fail retiming '{}' stream {} as .{}: {}. Try retime as .srt",
                src.display(),
                i_stream,
                dest.ty.as_ext(),
                err
            );
            dest.ty = SubType::Srt;
            dest.path.set_extension("srt");
            Ok(())
        };

        let mut dest = self.new_destination(i, src, i_stream, is_base);

        if is_base {
            if let Err(err) = self.try_base_sub(i_stream, &dest) {
                fall(&mut dest, err)?;
                self.try_base_sub(i_stream, &dest)?;
            }
        } else {
            if let Err(err) = self.try_external_sub(src, i_stream, &dest) {
                fall(&mut dest, err)?;
                self.try_external_sub(src, i_stream, &dest)?;
            }
        }

        Ok(RetimedStream {
            src: Some(dest.path),
            i_stream: 0,
            src_time: None,
        })
    }
}

impl Retiming<'_, '_> {
    fn len_prev_uid_parts(&self, i_part: usize) -> f64 {
        let src = &self.parts[i_part].src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.src == src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }

    fn len_prev_parts(&self, i_part: usize) -> f64 {
        self.parts[..i_part]
            .iter()
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }

    fn len_prev_nonuid_parts(&self, i_part: usize) -> f64 {
        let src = &self.parts[i_part].src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.src != src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }
}

fn save_idxs(rsub: &Subs, trg_start: Time, trg_end: Time) -> Vec<usize> {
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

fn try_extract(src: &Path, i_stream: usize, dest: &Destination) -> Result<()> {
    use crate::ffmpeg::{Rational, format};

    let mut ictx = format::input(&src)?;
    let istream = ictx
        .stream(i_stream)
        .ok_or_else(|| err!("invalid stream index"))?;

    let out_time_base = match dest.ty {
        SubType::Ssa => Rational::new(1, 100),
        _ => Rational::new(1, 1000),
    };
    let codec_id = istream.parameters().id();

    let mut octx = format::output(&dest.path)?;

    let ostream_index = {
        let mut ostream = octx.add_stream(codec_id)?;
        ostream.set_parameters(istream.parameters());
        ostream.set_time_base(out_time_base);
        ostream.index()
    };

    octx.write_header()?;

    for (stream, mut packet) in ictx.packets() {
        if stream.index() != i_stream {
            continue;
        }

        packet.set_stream(ostream_index);
        packet.rescale_ts(stream.time_base(), out_time_base);
        packet.write(&mut octx)?;
    }

    octx.write_trailer()?;
    Ok(())
}
