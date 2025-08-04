use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags, TFlags};
use crate::{
    IsDefault, LangCode, MediaInfo, MuxConfigArg, MuxError, ParseableArg, TFlagType, TFlagsCounts,
    ToFfmpegArgs, ToJsonArgs, ToMkvmergeArg, ToMkvmergeArgs, TrackType, immut,
    markers::{MCLocale, MICmnTrackOrder, MISavedTracks, MITIItSigns, MITITrackIDs, MITargets},
    to_json_args, unwrap_or_return,
};
use enum_map::EnumMap;
use std::{collections::HashSet, ffi::OsString, path::Path};

impl MediaInfo<'_> {
    /// Converts a values of all track flags to mkvmerge-compatible arguments.
    pub fn try_to_mkvmerge_args_of_flags(&mut self) -> Result<Vec<Vec<OsString>>, MuxError> {
        let flags = self.try_to_flag_values()?;
        let order = self.try_get_cmn::<MICmnTrackOrder>()?;
        let i_track_type = order.i_track_type();

        let mut args: Vec<Vec<OsString>> = vec![Vec::new(); order.media().len()];

        flags.into_iter().enumerate().for_each(|(stream, flags)| {
            let (i, track) = (i_track_type[stream].0, i_track_type[stream].1);

            flags.into_iter().for_each(|(flag, val)| {
                if let Some(s) = TFlags::val_to_mkvmerge_arg(track, val) {
                    args[i].push(flag.to_mkvmerge_arg().into());
                    args[i].push(s.into());
                }
            })
        });

        Ok(args)
    }
}

macro_rules! flags_to_mkvmerge_args {
    ($flags:ident, $arg:ident) => {
        /// Returns arguments based on user-defined values only.
        ///
        /// If you need auto-values too, use [`MediaInfo::try_to_mkvmerge_args_of_flags`].
        impl ToMkvmergeArgs for $flags {
            fn try_append_mkvmerge_args(
                &self,
                args: &mut Vec<OsString>,
                mi: &mut MediaInfo,
                media: &Path,
            ) -> Result<(), MuxError> {
                if self.is_default() {
                    return Ok(());
                }

                let tracks = mi.try_take::<MISavedTracks>(media)?;

                tracks
                    .values()
                    .flat_map(|vals| vals.iter())
                    .for_each(|&track| {
                        let val = self.get_manual_val(mi, media, track);

                        if let Some(arg) = TFlags::val_to_mkvmerge_arg(track, val) {
                            args.push(MuxConfigArg::$arg.dashed().into());
                            args.push(arg.into());
                        }
                    });

                mi.set::<MISavedTracks>(media, tracks);

                Ok(())
            }
        }
    };
}

flags_to_mkvmerge_args!(DefaultTFlags, DefaultTrackFlag);
flags_to_mkvmerge_args!(ForcedTFlags, ForcedDisplayFlag);
flags_to_mkvmerge_args!(EnabledTFlags, TrackEnabledFlag);

impl ToFfmpegArgs for TFlags {
    fn try_append_ffmpeg_args(
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
    ) -> Result<(), MuxError> {
        let flags = mi.try_to_flag_values()?;

        TrackType::iter_customizable().for_each(|ty| {
            args.push(format!("-disposition:{}", ty.as_char()).into());
            args.push("none".into());
        });

        flags.into_iter().enumerate().for_each(|(stream, flags)| {
            let mut arg = String::with_capacity(14);

            TFlagType::iter_ffmpeg_supported().for_each(|flag| {
                if !matches!(flags[flag], Some(true)) {
                    return;
                }

                if arg.is_empty() {
                    arg.push_str(flag.as_str_ffmpeg());
                } else {
                    arg.push('+');
                    arg.push_str(flag.as_str_ffmpeg());
                }
            });

            if arg.is_empty() {
                return;
            }

            args.push(format!("-disposition:{}", stream).into());
            args.push(arg.into())
        });

        Ok(())
    }

    fn append_stream(
        args: &mut Vec<OsString>,
        mi: &mut MediaInfo,
        media: &Path,
        track: u64,
        out_stream: usize,
    ) {
        unwrap_or_return!(mi.init::<MITargets>(media));
        let tids = unwrap_or_return!(immut!(mi, MITITrackIDs, media, track));
        let targets = unwrap_or_return!(mi.immut::<MITargets>(media));

        let mut arg = String::with_capacity(14);

        TFlagType::iter_ffmpeg_supported().for_each(|flag| {
            let flags = mi.mux_config.trg_field_t_flags(targets, flag);
            let val = flags.get(&tids[0]).or_else(|| flags.get(&tids[1]));

            match val {
                Some(false) => return,
                None if flag != TFlagType::Default => return,
                _ => {}
            }

            if arg.is_empty() {
                arg.push_str(flag.as_str_ffmpeg());
            } else {
                arg.push('+');
                arg.push_str(flag.as_str_ffmpeg());
            }
        });

        if arg.is_empty() {
            return;
        }

        args.push(format!("-disposition:{}", out_stream).into());
        args.push(arg.into())
    }
}

macro_rules! flags_to_json_args {
    ($ty:ty, $arg:ident, $lim_arg:ident) => {
        impl ToJsonArgs for $ty {
            fn append_json_args(&self, args: &mut Vec<String>) {
                if self.is_default() {
                    return;
                }

                if let Some(unmapped) = self.unmapped {
                    args.push(to_json_args!($arg));
                    args.push(unmapped.to_string());
                    return;
                }

                if let Some(lim) = self.lim_for_unset {
                    args.push(to_json_args!($lim_arg));
                    args.push(lim.to_string());
                }

                let id_map = to_json_args!(@collect_id_map, self);

                if id_map.is_empty() {
                    return;
                }

                let id_map = id_map.into_iter().collect::<Vec<String>>().join(",");

                args.push(to_json_args!($arg));
                args.push(id_map);
            }
        }
    };
}

flags_to_json_args!(DefaultTFlags, Defaults, MaxDefaults);
flags_to_json_args!(ForcedTFlags, Forceds, MaxForceds);
flags_to_json_args!(EnabledTFlags, Enableds, MaxEnableds);

impl MediaInfo<'_> {
    fn try_to_flag_values(&mut self) -> Result<Vec<EnumMap<TFlagType, Option<bool>>>, MuxError> {
        let order = self.try_take_cmn::<MICmnTrackOrder>()?;
        let order_media = order.media();
        let i_track_type = order.i_track_type();
        let locale_lang = *self.mux_config.field::<MCLocale>();

        let mut counts = TFlagsCounts::default();
        let mut default_audio_langs: HashSet<LangCode> = HashSet::new();
        let mut has_default_audio = false;

        let mut values: Vec<EnumMap<TFlagType, Option<bool>>> =
            Vec::with_capacity(i_track_type.len());

        i_track_type.iter().for_each(|&(i, track, ty)| {
            let media = &order_media[i];
            let _ = unwrap_or_return!(self.init::<MITargets>(media));
            let track_ids = unwrap_or_return!(immut!(self, MITITrackIDs, media, track));
            let targets = unwrap_or_return!(self.immut::<MITargets>(media));

            let mut map: EnumMap<TFlagType, Option<bool>> = EnumMap::default();

            TFlagType::iter().for_each(|flag| {
                let flags = self.mux_config.trg_field_t_flags(targets, flag);

                let val = flags
                    .get(&track_ids[0])
                    .or_else(|| flags.get(&track_ids[1]))
                    .or_else(|| {
                        self.auto_flags().auto_t_flags(flag).then(|| {
                            let cnt = counts.val(flag, ty);
                            let val = flags.auto_val(cnt, flag);

                            if val && ty == TrackType::Sub && flag == TFlagType::Default {
                                let it_signs =
                                    *self.immut_ti::<MITIItSigns>(media, track).unwrap_or(&false);

                                if it_signs && !has_default_audio {
                                    return false;
                                }

                                if !it_signs && has_default_audio {
                                    return false;
                                }

                                if !it_signs
                                    && default_audio_langs.contains(&LangCode::from(&track_ids[1]))
                                {
                                    return false;
                                }
                            }

                            val
                        })
                    });

                if let Some(true) = val {
                    if ty == TrackType::Audio {
                        let lang = LangCode::from(&track_ids[1]);
                        if lang == locale_lang {
                            has_default_audio = true;
                        } else {
                            default_audio_langs.insert(lang);
                        }
                    }

                    counts.add(flag, ty);
                }

                map[flag] = val;
            });

            values.push(map);
        });

        Ok(values)
    }
}

impl TFlags {
    #[inline]
    fn val_to_mkvmerge_arg(track: u64, opt: Option<bool>) -> Option<String> {
        match opt {
            Some(true) => Some(track.to_string()),
            Some(false) => Some(format!("{}:0", track)),
            None => None,
        }
    }

    #[inline(always)]
    fn get_manual_val(&self, mi: &mut MediaInfo, path: &Path, num: u64) -> Option<bool> {
        mi.get_ti::<MITITrackIDs>(path, num)?
            .iter()
            .find_map(|tid| self.get(tid))
    }
}
