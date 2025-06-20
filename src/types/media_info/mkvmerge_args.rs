use super::MediaInfo;
use crate::{
    LangCode, MCAudioTracks, MCButtonTracks, MCChapters, MCFontAttachs, MCLocale, MCSpecials,
    MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks, MITILang, MITargets, MuxError,
    TFlagType, TFlagsCounts, ToMkvmergeArg, ToMkvmergeArgs, TrackID, TrackOrder, TrackType,
};
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

impl MediaInfo<'_> {
    pub fn collect_os_mkvmerge_args(&mut self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();
        self.extend_vec_os_mkvmerge_args(&mut args);
        args
    }

    pub fn extend_vec_os_mkvmerge_args(&mut self, args: &mut Vec<OsString>) {
        match self.init_track_order() {
            Ok(order) => {
                // self and Path unused, just trait requirements
                args.extend(order.to_os_mkvmerge_args(self, Path::new("")));

                let (mut paths, i_num_type) = (order.paths, order.i_num_type);
                let mut path_args: Vec<Vec<OsString>> = vec![Vec::new(); paths.len()];

                let mut counts = TFlagsCounts::default();
                let mut default_audio_langs: HashSet<LangCode> = HashSet::new();
                let locale_lang = *self.mc.get::<MCLocale>();

                for (i, num, tt) in i_num_type.iter() {
                    let (i, num, tt) = (*i, *num, *tt);

                    let path = &paths[i];
                    let lang = *self.get_ti::<MITILang>(path, num).unwrap_or(&LangCode::Und);

                    // unwrap() safe because targets was cached in TrackOrder::try_from(self)
                    let targets = self.unmut_get::<MITargets>(path).unwrap();

                    for ft in TFlagType::iter() {
                        let flags = self.mc.get_trg_t_flags(targets, ft);

                        let val = flags
                            .get(&TrackID::Num(num))
                            .or_else(|| flags.get(&TrackID::Lang(lang)))
                            .or_else(|| {
                                self.off_on_pro.add_t_flags(ft).then(|| {
                                    let cnt = counts.get(ft, tt);
                                    let mut val = flags.auto_val(cnt, ft);
                                    if val && tt == TrackType::Sub && ft == TFlagType::Default {
                                        val = !default_audio_langs.contains(&lang)
                                            || !default_audio_langs.contains(&locale_lang);
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

                        if let Some(_) = val {
                            let val = if val.unwrap() { "" } else { ":0" };
                            let val = format!("{}{}", num, val);
                            path_args[i].push(ft.to_mkvmerge_arg().into());
                            path_args[i].push(val.into());
                        }
                    }
                }

                let mut added = vec![false; paths.len()];

                for (i, _, _) in i_num_type {
                    if !added[i] {
                        added[i] = true;
                        extend_file_args(args, self, &paths[i]);
                        args.extend(std::mem::take(&mut path_args[i]));
                        args.push(std::mem::take(&mut paths[i]).into_os_string());
                    }
                }
            }

            Err(e) => {
                log::warn!("{}", e);
                let paths: Vec<PathBuf> = self.cache.keys().cloned().collect();
                for path in paths {
                    extend_file_args(args, self, &path);
                    args.push(path.into_os_string());
                }
            }
        };
    }

    #[inline(always)]
    fn init_track_order(&mut self) -> Result<TrackOrder, MuxError> {
        TrackOrder::try_from(self)
    }
}

fn extend_file_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    let targets = match mi.get::<MITargets>(path) {
        Some(targets) => targets.clone(),
        None => return,
    };

    args.extend(
        mi.mc
            .get_trg::<MCAudioTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCSubTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCVideoTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCButtonTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );

    args.extend(
        mi.mc
            .get_trg::<MCChapters>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    // Adds fonts and other attachs both
    args.extend(
        mi.mc
            .get_trg::<MCFontAttachs>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );

    args.extend(
        mi.mc
            .get_trg::<MCTrackNames>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCTrackLangs>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCSpecials>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
}
