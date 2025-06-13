use super::{AudioTracks, ButtonTracks, SubTracks, VideoTracks, id::TrackID};
use crate::{
    CLIArg, IsDefault, MCOffOnPro, MISavedAudioU32IDs, MISavedButtonU32IDs, MISavedSubU32IDs,
    MISavedVideoU32IDs, MITILang, MITIName, MITracksInfo, MediaInfo, ToMkvmergeArg, ToMkvmergeArgs,
    TrackLangs, TrackNames, ok_or_return_vec_new, to_mkvmerge_args,
};
use std::path::Path;

impl ToMkvmergeArg for TrackID {
    fn to_mkvmerge_arg(&self) -> String {
        match self {
            Self::U32(n) => n.to_string(),
            Self::Lang(code) => code.to_string(),
            Self::Range(rng) => rng.to_mkvmerge_arg(),
        }
    }
}

macro_rules! tracks_to_mkvmerge_args {
    ( $( $type:ident, $mi_marker:ident => $arg:ident, $no_arg:ident, )* ) => {
        $(
            impl ToMkvmergeArgs for $type {
                fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path
                ) -> Vec<String> {
                    if self.is_default() {
                        Vec::new()
                    } else if self.no_flag {
                        let no_arg = to_mkvmerge_args!(@cli_arg, $no_arg);
                        vec![no_arg]
                    } else {
                        let ids: String = ok_or_return_vec_new!(mi.get::<$mi_marker>(path))
                            .into_iter()
                            .map(|id| id.to_mkvmerge_arg())
                            .collect::<Vec<_>>()
                            .join(",");

                        if ids.is_empty() {
                            let no_arg = to_mkvmerge_args!(@cli_arg, $no_arg);
                            vec![no_arg]
                        } else {
                            let arg = to_mkvmerge_args!(@cli_arg, $arg);
                            vec![arg, ids]
                        }
                    }
                }

                to_mkvmerge_args!(@fn_os);
            }
        )*
    };
}

tracks_to_mkvmerge_args!(
    AudioTracks, MISavedAudioU32IDs => Audio, NoAudio,
    SubTracks, MISavedSubU32IDs => Subs, NoSubs,
    VideoTracks, MISavedVideoU32IDs => Video, NoVideo,
    ButtonTracks, MISavedButtonU32IDs => Buttons, NoButtons,
);

macro_rules! langs_or_names_to_mkvmerge_args {
    ($typ:ident, $arg:ident, $add_marker:ident, $tic_marker:ident) => {
        impl ToMkvmergeArgs for $typ {
            fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
                let arg = to_mkvmerge_args!(@cli_arg, $arg);
                let add = mi.mc.get::<MCOffOnPro>().$add_marker;
                let tids_u32: Vec<TrackID> = ok_or_return_vec_new!(mi.get::<MITracksInfo>(path))
                    .keys().copied().collect();

                let val_args: Vec<String> = tids_u32
                    .into_iter()
                    .filter_map(|tid| {
                        self.get(tid).map(|x| x.to_string())
                            .or_else(|| add.then(|| mi.get_ti::<$tic_marker>(path, tid))
                                .flatten()
                                .map(|x| x.to_string()))
                            .map(|val| format!("{}:{}", tid.to_mkvmerge_arg(), val.to_string()))
                    })
                    .collect();

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
    };
}

langs_or_names_to_mkvmerge_args!(TrackLangs, Langs, add_langs, MITILang);
langs_or_names_to_mkvmerge_args!(TrackNames, Names, add_names, MITIName);
