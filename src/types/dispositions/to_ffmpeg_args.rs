use super::*;
use crate::{DispositionType, MediaInfo, Result, StreamType, ToFfmpegArgs, markers::*};
use enum_map::EnumMap;
use std::{collections::HashSet, ffi::OsString};

impl ToFfmpegArgs for Dispositions {
    fn append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
        StreamType::iter_track().for_each(|ty| {
            args.push(format!("-disposition:{}", ty.as_first_s()).into());
            args.push("0".into());
        });

        let order = mi.try_take_cmn(MICmnStreamsOrder)?;
        let cfg = mi.cfg;
        let auto = cfg.auto_flags.map_dispositions();

        let mut counts: EnumMap<StreamType, EnumMap<DispositionType, usize>> = EnumMap::default();
        let mut default_audio_langs: HashSet<LangCode> = HashSet::new();
        let mut has_locale_audio = false;

        for (i, m) in order.iter_track().enumerate() {
            let key = &m.key;
            let target_paths = mi.try_take(MITargetPaths, key)?;
            let streams = mi.try_take(MIStreams, key)?;
            let stream = &streams[m.key_i_stream];

            let mut val = |ty| {
                let (i_key, values) = cfg.stream_val_dispositions(ty, &target_paths, stream);

                let v = values.get(&i_key, &stream.lang).or_else(|| {
                    if auto[ty] {
                        let cnt = counts[stream.ty][ty];
                        let mut v = cnt < values.max(ty);

                        if v && stream.ty == StreamType::Sub && ty == DispositionType::Default {
                            let it_signs = mi.it_signs(&m.key, stream);

                            if (it_signs && !has_locale_audio)
                                || (!it_signs && has_locale_audio)
                                || (!it_signs && default_audio_langs.contains(&*stream.lang))
                            {
                                v = false;
                            }
                        }

                        Some(v)
                    } else {
                        None
                    }
                });

                if let Some(true) = v {
                    counts[stream.ty][ty] += 1;
                    if stream.ty.is_audio() {
                        default_audio_langs.insert(*stream.lang);
                        if *stream.lang == cfg.locale {
                            has_locale_audio = true;
                        }
                    }
                }

                v
            };

            let mut arg = String::with_capacity(15);

            DispositionType::iter().for_each(|ty| {
                if let Some(true) = val(ty) {
                    arg.push('+');
                    arg.push_str(ty.as_ref().into());
                }
            });

            if !arg.is_empty() {
                args.push(format!("-disposition:{}", i).into());
                args.push(arg.into());
            }

            mi.set(MITargetPaths, key, target_paths);
            mi.set(MIStreams, key, streams);
        }

        mi.set_cmn(MICmnStreamsOrder, order);
        Ok(())
    }
}
