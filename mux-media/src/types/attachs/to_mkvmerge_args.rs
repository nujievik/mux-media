use super::{Attachs, FontAttachs, OtherAttachs};
use crate::{
    AttachType, IsDefault, MediaInfo, MkvmergeArg, MkvmergeNoArg, ToMkvmergeArgs,
    markers::{MCFontAttachs, MCOtherAttachs, MIAttachsInfo, MITargets},
    mkvmerge_arg, mkvmerge_no_arg, to_mkvmerge_args, unmut, unwrap_or_return_vec,
};
use std::{collections::BTreeSet, path::Path};

mkvmerge_arg!(Attachs, "-m");
mkvmerge_no_arg!(Attachs, "-M");

impl ToMkvmergeArgs for FontAttachs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
        self.inner().to_mkvmerge_args(mi, path)
    }

    to_mkvmerge_args!(@fn_os);
}

impl ToMkvmergeArgs for OtherAttachs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
        self.inner().to_mkvmerge_args(mi, path)
    }

    to_mkvmerge_args!(@fn_os);
}

impl ToMkvmergeArgs for Attachs {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
        let targets = unwrap_or_return_vec!(unmut!(mi, MITargets, path));

        let fonts = mi.mux_config.trg_field::<MCFontAttachs>(targets);
        let other = mi.mux_config.trg_field::<MCOtherAttachs>(targets);

        if fonts.is_default() && other.is_default() {
            return Vec::new();
        }

        let ai = unwrap_or_return_vec!(mi.get::<MIAttachsInfo>(path));
        let cnt_init = ai.len();

        if cnt_init == 0 {
            return Vec::new();
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
            return vec![Self::MKVMERGE_NO_ARG.to_owned()];
        }

        if cnt == cnt_init {
            return Vec::new();
        }

        let arg = Self::MKVMERGE_ARG.to_owned();
        let nums = shortest_track_nums(nums, cnt, cnt_init);

        vec![arg, nums]
    }

    to_mkvmerge_args!(@fn_os);
}

#[inline(always)]
fn shortest_track_nums(mut nums: BTreeSet<u64>, cnt: usize, cnt_init: usize) -> String {
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
