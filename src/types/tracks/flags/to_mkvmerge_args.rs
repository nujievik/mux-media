use crate::{
    CLIArg, DefaultTFlags, EnabledTFlags, ForcedTFlags, MCOffOnPro, MITILang, MITracksInfo,
    MediaInfo, TFlags, ToMkvmergeArg, ToMkvmergeArgs, TrackID, TrackType, ok_or_return_vec_new,
    to_mkvmerge_args,
};

impl TFlags {
    #[inline]
    fn auto_val(&self, cnt: u32, _: bool) -> bool {
        self.less_cnt(cnt)
    }
}

impl DefaultTFlags {
    #[inline]
    fn auto_val(&self, cnt: u32, _has_locale_audio: bool) -> bool {
        self.less_cnt(cnt)
    }
}

macro_rules! any_t_flags_to_mkvmerge_args {
    ($($t_flags:ident, $arg:ident, $add_off:ident, $get_cnt:ident, $add_cnt:ident;)*) => {
        $(
            impl ToMkvmergeArgs for $t_flags {
                fn to_mkvmerge_args(&self, fi: &mut MediaInfo, path: &std::path::Path
                ) -> Vec<String> {
                    let arg = to_mkvmerge_args!(@cli_arg, $arg);
                    let add = fi.mc.get::<MCOffOnPro>().$add_off;
                    let ti = ok_or_return_vec_new!(fi.get::<MITracksInfo>(path));

                    let tid_tt_pairs: Vec<(TrackID, TrackType)> = ti
                        .iter().map(|(id, cache)| (*id, cache.track_type)).collect();
                    let mut counts = std::mem::take(&mut fi.counts);

                    let val_args: Vec<String> = tid_tt_pairs
                        .into_iter()
                        .filter_map(|(tid_u32, tt)| {
                            self.get(tid_u32)
                                .or_else(|| {
                                    fi.get_ti::<MITILang>(path, tid_u32)
                                        .and_then(|lang| self.get(TrackID::Lang(*lang)))
                                })
                                .or_else(|| {
                                    add.then(|| {
                                        let cnt = counts.$get_cnt(tt);
                                        self.auto_val(cnt, false)
                                    })
                                })
                                .map(|val| {
                                    if val {
                                        counts.$add_cnt(tt);
                                    }
                                    format!("{}{}", tid_u32.to_mkvmerge_arg(),
                                            if val { "" } else { ":0" }
                                    )
                                })
                        })
                        .collect();

                    fi.counts = counts;

                    if val_args.is_empty() {
                        return Vec::new();
                    }

                    let mut args: Vec<String> = Vec::with_capacity(val_args.len() * 2);
                    for val in val_args {
                        args.push(arg.clone());
                        args.push(val);
                    }

                    args
                }

                to_mkvmerge_args!(@fn_os);
            }
        )*
    };
}

any_t_flags_to_mkvmerge_args!(
    DefaultTFlags, Defaults, add_defaults, get_default, add_default;
    ForcedTFlags, Forceds, add_forceds, get_forced, add_forced;
    EnabledTFlags, Enableds, add_enableds, get_enabled, add_enabled;
);
