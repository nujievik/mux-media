use super::{Attachs, FontAttachs, OtherAttachs};
use crate::{
    AttachType, IsDefault, MediaInfo, Result, ToMkvmergeArgs, dashed, immut,
    markers::{MCFontAttachs, MCOtherAttachs, MIAttachsInfo, MITargets},
    to_json_args,
};
use std::{collections::BTreeSet, ffi::OsString, path::Path};

to_json_args!(@tracks_or_attachs, FontAttachs, Fonts, NoFonts);
to_json_args!(@tracks_or_attachs, OtherAttachs, Attachs, NoAttachs);

impl ToMkvmergeArgs for FontAttachs {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
    ) -> Result<()> {
        self.0.try_append_mkvmerge_args(args, mi, media)?;
        Ok(())
    }
}

impl ToMkvmergeArgs for OtherAttachs {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
    ) -> Result<()> {
        self.0.try_append_mkvmerge_args(args, mi, media)?;
        Ok(())
    }
}

impl ToMkvmergeArgs for Attachs {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
    ) -> Result<()> {
        let targets = immut!(@try, mi, MITargets, media)?;

        let fonts = mi.cfg.target(MCFontAttachs, targets);
        let other = mi.cfg.target(MCOtherAttachs, targets);

        if fonts.is_default() && other.is_default() {
            return Ok(());
        }

        let ai = mi.try_get::<MIAttachsInfo>(media)?;
        let cnt_init = ai.len();

        if cnt_init == 0 {
            return Ok(());
        }

        let nums: BTreeSet<u64> = ai
            .into_iter()
            .filter_map(|(num, cache)| {
                let id = &cache.id;
                match cache.attach_type {
                    AttachType::Font if fonts.save_attach(id) => Some(*num),
                    AttachType::Other if other.save_attach(id) => Some(*num),
                    _ => None,
                }
            })
            .collect();

        let cnt = nums.len();

        if cnt == 0 {
            args.push(dashed!(NoAttachments).into());
            return Ok(());
        }

        if cnt == cnt_init {
            return Ok(());
        }

        args.push(dashed!(Attachments).into());
        args.push(shortest_track_of_nums(nums, cnt, cnt_init).into());

        Ok(())
    }
}

#[inline(always)]
fn shortest_track_of_nums(mut nums: BTreeSet<u64>, cnt: usize, cnt_init: usize) -> String {
    let inverse = cnt > (cnt_init / 2);

    if inverse {
        nums = (1..=cnt_init)
            .filter_map(|num| {
                let num = num as u64;
                (!nums.contains(&num)).then(|| num)
            })
            .collect();
    }

    let mut nums: String = nums
        .into_iter()
        .map(|aid| aid.to_string())
        .collect::<Vec<_>>()
        .join(",");

    if inverse {
        nums.insert(0, '!');
    }

    nums
}
