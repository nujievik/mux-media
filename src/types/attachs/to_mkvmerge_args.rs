use super::{Attachs, FontAttachs, OtherAttachs, id::AttachID};
use crate::{
    AttachType, IsDefault, MCFontAttachs, MCOtherAttachs, MIAttachsInfo, MITargets, MediaInfo,
    MkvmergeArg, MkvmergeNoArg, ToMkvmergeArg, ToMkvmergeArgs, mkvmerge_arg, mkvmerge_no_arg,
    to_mkvmerge_args, unmut_get, unwrap_or_return_vec,
};
use std::collections::BTreeSet;
use std::path::Path;

mkvmerge_arg!(Attachs, "-m");
mkvmerge_no_arg!(Attachs, "-M");

impl ToMkvmergeArg for AttachID {
    fn to_mkvmerge_arg(&self) -> String {
        match self {
            Self::Num(n) => n.to_string(),
            Self::Range(rng) => rng.to_mkvmerge_arg(),
        }
    }
}

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
        let targets = unwrap_or_return_vec!(unmut_get!(mi, MITargets, path));

        let fonts = mi.mc.get_trg::<MCFontAttachs>(targets);
        let other = mi.mc.get_trg::<MCOtherAttachs>(targets);

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
            .filter_map(|(_, cache)| {
                let num = cache.num;
                let tid = AttachID::Num(num);
                match cache.attach_type {
                    AttachType::Font if fonts.save_attach(&tid) => Some(num),
                    AttachType::Other if other.save_attach(&tid) => Some(num),
                    _ => None,
                }
            })
            .collect();

        let cnt = nums.len();
        if cnt == cnt_init {
            return Vec::new();
        }

        if nums.is_empty() {
            vec![Self::MKVMERGE_NO_ARG.into()]
        } else {
            let arg: String = Self::MKVMERGE_ARG.into();
            let nums = shortest_track_nums(nums, cnt, cnt_init);
            vec![arg, nums]
        }
    }

    to_mkvmerge_args!(@fn_os);
}

#[inline]
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
