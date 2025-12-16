use super::*;

impl Retiming<'_, '_> {
    pub(super) fn try_external_sub(
        &self,
        src: &Path,
        i_stream: usize,
        dest: &Destination,
    ) -> Result<()> {
        let old = if let Some(_) = SubType::new_from_extension(dest.src_ext) {
            Subs::new(src, dest.ty)?
        } else {
            try_extract(src, i_stream, dest)?;
            Subs::new(&dest.path, dest.ty)?
        };

        let mut idxs_offset: Vec<(Vec<usize>, f64)> = Vec::with_capacity(self.chapters.len());

        for (i_part, p) in self.parts.iter().enumerate() {
            for i_chp in p.i_start_chp..=p.i_end_chp {
                if let Some((idxs, offset)) = get_idxs_offset(self, &old, i_chp, i_part) {
                    idxs_offset.push((idxs, offset))
                }
            }
        }

        if idxs_offset.is_empty() {
            Err(err!("Not saved any subtitle line"))
        } else {
            let new = retime(old, idxs_offset);
            new.try_write(&dest.path)
        }
    }
}

fn retime(old: Subs, idxs_offset: Vec<(Vec<usize>, f64)>) -> Subs {
    return match old {
        Subs::Srt(old) => Subs::Srt(srt(old, idxs_offset)),
        Subs::Ssa(old) => Subs::Ssa(ssa(old, idxs_offset)),
        Subs::Vtt(old) => Subs::Vtt(vtt(old, idxs_offset)),
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
    old: &Subs,
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

    let trg_start = Duration::from_secs_f64(chp.start.as_secs_f64() + p.start_offset + chp_nonuid);
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
        let offset = rtm.len_prev_parts(i_part) - p.start.as_secs_f64() - chp_nonuid;
        Some((idxs, offset))
    }
}
