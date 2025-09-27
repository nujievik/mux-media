use super::{DefaultTrackFlags, EnabledTrackFlags, ForcedTrackFlags, TrackFlags};
use crate::{
    IsDefault, LangCode, MediaInfo, Result, ToFfmpegArgs, ToJsonArgs, ToMkvmergeArgs,
    TrackFlagType, TrackFlagsCounts, TrackType, dashed, immut,
    markers::{MICmnTrackOrder, MISavedTracks, MITIItSigns, MITITrackIDs, MITargets},
    to_json_args, unwrap_or_return,
};
use enum_map::EnumMap;
use std::{collections::HashSet, ffi::OsString, path::Path};

impl MediaInfo<'_> {
    /// Converts a values of all track flags to mkvmerge-compatible arguments.
    pub fn try_to_mkvmerge_args_of_flags(&mut self) -> Result<Vec<Vec<OsString>>> {
        let flags = self.try_to_flag_values()?;
        let order = self.try_get_cmn::<MICmnTrackOrder>()?;

        let len = order.iter_first_entries().count();
        let mut args: Vec<Vec<OsString>> = vec![Vec::new(); len];

        flags.into_iter().enumerate().for_each(|(i, flags)| {
            let number = order[i].number as usize;
            let track = order[i].track;
            flags.into_iter().for_each(|(flag, val)| {
                if let Some(s) = TrackFlags::val_to_mkvmerge_arg(track, val) {
                    args[number].push(flag.as_cli_arg().dashed().into());
                    args[number].push(s.into());
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
            ) -> Result<()> {
                if self.is_default() {
                    return Ok(());
                }

                let tracks = mi.try_take::<MISavedTracks>(media)?;

                tracks
                    .values()
                    .flat_map(|vals| vals.iter())
                    .for_each(|&track| {
                        let val = self.get_manual_val(mi, media, track);

                        if let Some(arg) = TrackFlags::val_to_mkvmerge_arg(track, val) {
                            args.push(dashed!($arg).into());
                            args.push(arg.into());
                        }
                    });

                mi.set::<MISavedTracks>(media, tracks);

                Ok(())
            }
        }
    };
}

flags_to_mkvmerge_args!(DefaultTrackFlags, DefaultTrackFlag);
flags_to_mkvmerge_args!(ForcedTrackFlags, ForcedDisplayFlag);
flags_to_mkvmerge_args!(EnabledTrackFlags, TrackEnabledFlag);

impl ToFfmpegArgs for TrackFlags {
    fn try_append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
        let flags = mi.try_to_flag_values()?;

        TrackType::iter_customizable().for_each(|ty| {
            args.push(format!("-disposition:{}", ty.as_first_s()).into());
            args.push("none".into());
        });

        flags.into_iter().enumerate().for_each(|(stream, flags)| {
            let mut arg = String::with_capacity(14);

            TrackFlagType::iter_ffmpeg_supported().for_each(|flag| {
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

        TrackFlagType::iter_ffmpeg_supported().for_each(|flag| {
            let flags = mi.cfg.target_track_flags(targets, flag);
            let val = flags.get(&tids[0]).or_else(|| flags.get(&tids[1]));

            match val {
                Some(false) => return,
                None if flag != TrackFlagType::Default => return,
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

flags_to_json_args!(DefaultTrackFlags, Defaults, MaxDefaults);
flags_to_json_args!(ForcedTrackFlags, Forceds, MaxForceds);
flags_to_json_args!(EnabledTrackFlags, Enableds, MaxEnableds);

impl MediaInfo<'_> {
    // Returns vec len == TrackOrder.len()
    fn try_to_flag_values(&mut self) -> Result<Vec<EnumMap<TrackFlagType, Option<bool>>>> {
        let order = self.try_take_cmn::<MICmnTrackOrder>()?;
        let locale = self.cfg.locale;

        let mut counts = TrackFlagsCounts::default();
        let mut default_audio_langs: HashSet<LangCode> = HashSet::new();
        let mut has_default_audio = false;

        let mut values: Vec<EnumMap<TrackFlagType, Option<bool>>> = Vec::with_capacity(order.len());

        order.iter().for_each(|m| {
            let media = &m.media;
            let _ = unwrap_or_return!(self.init::<MITargets>(media));
            let track_ids = unwrap_or_return!(immut!(self, MITITrackIDs, media, m.track));
            let targets = unwrap_or_return!(self.immut::<MITargets>(media));

            let mut map: EnumMap<TrackFlagType, Option<bool>> = EnumMap::default();

            TrackFlagType::iter().for_each(|flag| {
                let flags = self.cfg.target_track_flags(targets, flag);

                let val = flags
                    .get(&track_ids[0])
                    .or_else(|| flags.get(&track_ids[1]))
                    .or_else(|| {
                        self.auto_flags.track[flag].then(|| {
                            let cnt = counts.val(flag, m.ty);
                            let val = flags.auto_val(cnt, flag);

                            if val && m.ty == TrackType::Sub && flag == TrackFlagType::Default {
                                let it_signs = *self
                                    .immut_ti::<MITIItSigns>(media, m.track)
                                    .unwrap_or(&false);

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
                    if m.ty == TrackType::Audio {
                        let lang = LangCode::from(&track_ids[1]);
                        if lang == locale {
                            has_default_audio = true;
                        } else {
                            default_audio_langs.insert(lang);
                        }
                    }

                    counts.add(flag, m.ty);
                }

                map[flag] = val;
            });

            values.push(map);
        });

        self.set_cmn::<MICmnTrackOrder>(order);
        Ok(values)
    }
}

impl TrackFlags {
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
