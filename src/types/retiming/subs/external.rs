use super::*;
use std::path::PathBuf;
use subtitle_lines::{SubtitleLines, Time, WriteOptions};

impl Retiming<'_, '_> {
    pub(super) fn try_external_sub(
        &self,
        src: &Path,
        i_stream: usize,
        dest: &Destination,
    ) -> Result<()> {
        let need_extract = SubType::new_from_extension(dest.src_ext).is_none();
        let mut opts = WriteOptions::new();
        let mut splits: Vec<PathBuf> = Vec::with_capacity(self.chapters.len());

        if need_extract {
            try_extract(src, i_stream, dest.ty, &dest.path)?;
        }

        for (i_part, p) in self.parts.iter().enumerate() {
            for i_chp in p.i_start_chp..=p.i_end_chp {
                if let Some((start, end, offset)) = get_start_end_offset(self, i_part, i_chp) {
                    opts.start = Some(start);
                    opts.end = Some(end);

                    if offset.is_positive() {
                        opts.add_time = Some(offset.as_unsigned_time());
                        opts.sub_time = None;
                    } else {
                        opts.sub_time = Some(offset.as_unsigned_time());
                        opts.add_time = None;
                    }

                    let src = if need_extract { &dest.path } else { src };
                    let lines = SubtitleLines::open_file(src)?;
                    let split = dest.destination_split(i_chp);

                    lines.write_with(&split, &opts)?;
                    splits.push(split);
                }
            }
        }

        merge(dest, &splits)
    }
}

fn get_start_end_offset(
    rtm: &Retiming<'_, '_>,
    i_part: usize,
    i_chp: usize,
) -> Option<(Time, Time, SignedTime)> {
    let p = &rtm.parts[i_part];
    let uid = &rtm.chapters[p.i_start_chp].uid;
    let chp = &rtm.chapters[i_chp];

    if uid != &chp.uid {
        return None;
    }

    let chp_nonuid = rtm.chapters_nonuid(i_chp);

    let trg_start = p.start_offset + chp.start + chp_nonuid;

    let end_offset = if i_chp == p.i_end_chp {
        p.end_offset
    } else {
        p.start_offset
    };
    let trg_end = end_offset + chp.end + chp_nonuid;

    let offset = SignedTime::new(true, rtm.len_prev_parts(i_part)) - p.start - chp_nonuid;

    Some((
        trg_start.as_unsigned_time(),
        trg_end.as_unsigned_time(),
        offset,
    ))
}
