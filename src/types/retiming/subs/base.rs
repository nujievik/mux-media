use super::*;

macro_rules! map_internal_subs {
    ($old_map:ident, $sub:ident, $new_ty:ty) => {
        $old_map
            .into_iter()
            .filter_map(|(k, v)| match v {
                Subs::$sub(old) => Some((k, old)),
                _ => {
                    eprintln!("Unexpected unmatch pattern");
                    None
                }
            })
            .collect::<HashMap<&Path, $new_ty>>()
    };
}

impl Retiming<'_, '_> {
    pub(super) fn try_base_sub(&self, i_stream: usize, dest: &Destination) -> Result<()> {
        let old = try_map_old(self, i_stream, &dest)?;

        let src_idxs_offset: Vec<_> = self
            .parts
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let src = p.src.as_path();
                get_idxs_offset(self, &old[src], i).map(|(xs, t)| (src, xs, t))
            })
            .collect();

        if src_idxs_offset.is_empty() {
            Err(err!("Not saved any subtitle line"))
        } else {
            let new = retime(old, src_idxs_offset);
            new.try_write(&dest.path)
        }
    }
}

fn retime(old: HashMap<&Path, Subs>, src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>) -> Subs {
    // unwrap safe, map is not empty
    return match old.values().next().unwrap() {
        Subs::Srt(_) => Subs::Srt(srt(old, src_idxs_offset)),
        Subs::Ssa(_) => Subs::Ssa(ssa(old, src_idxs_offset)),
        Subs::Vtt(_) => Subs::Vtt(vtt(old, src_idxs_offset)),
    };

    fn srt(old: HashMap<&Path, Subs>, src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>) -> SRT {
        let old = map_internal_subs!(old, Srt, SRT);
        let mut lines: Vec<SRTLine> = Vec::with_capacity(capacity(&src_idxs_offset));
        let mut i = 1u32;
        for (src, idxs, offset) in src_idxs_offset {
            push_retimed_srt_lines(&mut lines, &mut i, &old[src].lines, idxs, offset)
        }
        SRT { lines }
    }

    fn ssa(old: HashMap<&Path, Subs>, src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>) -> SSA {
        let old = map_internal_subs!(old, Ssa, SSA);
        let mut events: Vec<SSAEvent> = Vec::with_capacity(capacity(&src_idxs_offset));
        for (src, idxs, offset) in src_idxs_offset {
            push_retimed_ssa_events(&mut events, &old[src].events, idxs, offset)
        }
        // unwrap safe, map is not empty.
        let mut new = old.into_values().next().unwrap();
        new.events = events;
        new
    }

    fn vtt(old: HashMap<&Path, Subs>, src_idxs_offset: Vec<(&Path, Vec<usize>, f64)>) -> VTT {
        let old = map_internal_subs!(old, Vtt, VTT);
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

fn get_idxs_offset(rtm: &Retiming<'_, '_>, old: &Subs, i_part: usize) -> Option<(Vec<usize>, f64)> {
    let p = &rtm.parts[i_part];
    let idxs = save_idxs(old, p.start.into(), p.end.into());
    let offset =
        rtm.len_prev_nonuid_parts(i_part) + rtm.len_prev_uid_parts(i_part) - p.start.as_secs_f64();

    Some((idxs, offset))
}

fn try_map_old<'a>(
    rtm: &'a Retiming<'_, '_>,
    i_stream: usize,
    dest: &Destination,
) -> Result<HashMap<&'a Path, Subs>> {
    let mut map: HashMap<&Path, Subs> = HashMap::new();
    for p in rtm.parts.iter() {
        let src = p.src.as_path();
        if !map.contains_key(&src) {
            let _ = fs::remove_file(&dest.path);
            try_extract(src, i_stream, dest)?;
            let sub = Subs::new(&dest.path, dest.ty)?;
            map.insert(src, sub);
        }
    }
    Ok(map)
}
