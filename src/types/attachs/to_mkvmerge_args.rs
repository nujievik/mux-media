use super::{Attachs, FontAttachs, OtherAttachs, id::AttachID};
use crate::{
    AttachType, CLIArg, IsDefault, MCFontAttachs, MCOtherAttachs, MIAttachsInfo, MITargets,
    MediaInfo, ToMkvmergeArg, ToMkvmergeArgs, ok_or_return_vec_new, to_mkvmerge_args,
};
use std::collections::BTreeSet;
use std::path::Path;

impl ToMkvmergeArg for AttachID {
    fn to_mkvmerge_arg(&self) -> String {
        match self {
            Self::U32(n) => n.to_string(),
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
        let targets = ok_or_return_vec_new!(mi.get::<MITargets>(path)).clone();
        let fonts = mi.mc.get_trg::<MCFontAttachs>(&targets);
        let other = mi.mc.get_trg::<MCOtherAttachs>(&targets);

        if fonts.is_default() && other.is_default() {
            return Vec::new();
        }

        let ai = ok_or_return_vec_new!(mi.get::<MIAttachsInfo>(path));
        let cnt_init = ai.len();

        if cnt_init == 0 {
            return Vec::new();
        }

        let u32_ids: BTreeSet<u32> = ai
            .into_iter()
            .filter_map(|(_, cache)| {
                let u32 = cache.id_u32;
                match cache.attach_type {
                    AttachType::Font if fonts.save_attach(u32) => Some(u32),
                    AttachType::Other if other.save_attach(u32) => Some(u32),
                    _ => None,
                }
            })
            .collect();

        let cnt = u32_ids.len();
        if cnt == cnt_init {
            return Vec::new();
        }

        if u32_ids.is_empty() {
            let no_arg = to_mkvmerge_args!(@cli_arg, NoAttachs);
            vec![no_arg]
        } else {
            let arg = to_mkvmerge_args!(@cli_arg, Attachs);
            let u32_ids = shortest_u32_ids(u32_ids, cnt, cnt_init);
            vec![arg, u32_ids]
        }
    }

    to_mkvmerge_args!(@fn_os);
}

#[inline]
fn shortest_u32_ids(mut u32_ids: BTreeSet<u32>, cnt: usize, cnt_init: usize) -> String {
    let inverse = cnt > (cnt_init / 2);

    if inverse {
        u32_ids = (1..=cnt_init)
            .filter_map(|n| {
                let aid = n as u32;
                (!u32_ids.contains(&aid)).then(|| aid)
            })
            .collect();
    }

    let mut u32_ids: String = u32_ids
        .into_iter()
        .map(|aid| aid.to_string())
        .collect::<Vec<_>>()
        .join(",");

    if inverse {
        u32_ids.insert(0, '!');
    }

    u32_ids
}
