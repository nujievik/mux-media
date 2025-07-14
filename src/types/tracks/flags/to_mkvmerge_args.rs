use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags, TFlags};
use crate::{
    IsDefault, LangCode, MCLocale, MISavedTracks, MITILang, MITITrackIDs, MITargets, MediaInfo,
    MkvmergeArg, TFlagType, TFlagsCounts, ToMkvmergeArg, ToMkvmergeArgs, TrackID, TrackOrder,
    TrackType, mkvmerge_arg, to_mkvmerge_args, unwrap_or_return_vec,
};
use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
};

mkvmerge_arg!(DefaultTFlags, "--default-track-flag");
mkvmerge_arg!(ForcedTFlags, "--forced-display-flag");
mkvmerge_arg!(EnabledTFlags, "--track-enabled-flag");

impl TFlags {
    /// Converts a values of all track flags to mkvmerge recognizable args.
    pub fn track_order_to_os_mkvmerge_args(
        mi: &mut MediaInfo,
        order: TrackOrder,
    ) -> (Vec<usize>, Vec<PathBuf>, Vec<Vec<OsString>>) {
        let (paths, i_num_type) = (order.paths, order.i_num_type);

        let paths_len = paths.len();
        let mut i_route: Vec<usize> = Vec::with_capacity(paths_len);
        let mut added: Vec<bool> = vec![false; paths_len];
        let mut path_args: Vec<Vec<OsString>> = vec![Vec::new(); paths_len];

        let mut counts = TFlagsCounts::default();
        let mut default_audio_langs: HashSet<LangCode> = HashSet::new();
        let locale_lang = *mi.mc.get::<MCLocale>();

        for (i, num, tt) in i_num_type.into_iter() {
            if !added[i] {
                added[i] = true;
                i_route.push(i);
            }

            let path = &paths[i];
            let lang = *mi.get_ti::<MITILang>(path, num).unwrap_or(&LangCode::Und);

            let targets = match mi.unmut_get::<MITargets>(path) {
                Some(targets) => targets,
                None => continue,
            };

            for ft in TFlagType::iter() {
                let flags = mi.mc.get_trg_t_flags(targets, ft);

                let val = flags
                    .get(&TrackID::Num(num))
                    .or_else(|| flags.get(&TrackID::Lang(lang)))
                    .or_else(|| {
                        mi.off_on_pro.add_t_flags(ft).then(|| {
                            let cnt = counts.get(ft, tt);
                            let mut val = flags.auto_val(cnt, ft);
                            if val && tt == TrackType::Sub && ft == TFlagType::Default {
                                val = !default_audio_langs.contains(&lang)
                                    && !default_audio_langs.contains(&locale_lang);
                            }
                            val
                        })
                    });

                if let Some(true) = val {
                    if tt == TrackType::Audio && ft == TFlagType::Default {
                        default_audio_langs.insert(lang);
                    }
                    counts.add(ft, tt);
                }

                if let Some(val) = val_to_arg(num, val) {
                    path_args[i].push(ft.to_mkvmerge_arg().into());
                    path_args[i].push(val.into());
                }
            }
        }

        (i_route, paths, path_args)
    }
}

#[inline(always)]
fn val_to_arg(num: u64, opt: Option<bool>) -> Option<String> {
    match opt {
        Some(true) => Some(num.to_string()),
        Some(false) => Some(format!("{}:0", num)),
        None => None,
    }
}

impl TFlags {
    #[inline(always)]
    fn get_manual_val(&self, mi: &mut MediaInfo, path: &Path, num: u64) -> Option<bool> {
        mi.get_ti::<MITITrackIDs>(path, num)?
            .iter()
            .find_map(|tid| self.get(tid))
    }
}

macro_rules! flags_to_mkvmerge_args {
    ($flags:ident) => {
        /// Returns arguments based on user-defined values only.
        ///
        /// If you need auto-values too, use `TFlags::track_order_to_os_mkvmerge_args()`.
        impl ToMkvmergeArgs for $flags {
            fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
                if self.is_default() {
                    return Vec::new();
                }

                let nums: Vec<u64> = unwrap_or_return_vec!(mi.get::<MISavedTracks>(path))
                    .values()
                    .flatten()
                    .map(|num| *num)
                    .collect();

                if nums.is_empty() {
                    return Vec::new();
                }

                nums
                    .into_iter()
                    .filter_map(|num| {
                        val_to_arg(num, self.get_manual_val(mi, path, num)).map(|arg| {
                            [Self::MKVMERGE_ARG.to_string(), arg]
                        })
                    })
                    .flatten()
                    .collect()
            }

            to_mkvmerge_args!(@fn_os);
        }
    }
}

flags_to_mkvmerge_args!(DefaultTFlags);
flags_to_mkvmerge_args!(ForcedTFlags);
flags_to_mkvmerge_args!(EnabledTFlags);
