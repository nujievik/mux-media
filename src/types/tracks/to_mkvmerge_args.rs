use super::{AudioTracks, ButtonTracks, SubTracks, VideoTracks};
use crate::{
    IsDefault, MISavedTracks, MediaInfo, MkvmergeArg, MkvmergeNoArg, ToMkvmergeArg, ToMkvmergeArgs,
    TrackType, mkvmerge_arg, mkvmerge_no_arg, to_mkvmerge_args, unwrap_or_return_vec,
};
use std::path::Path;

mkvmerge_arg!(AudioTracks, "-a");
mkvmerge_no_arg!(AudioTracks, "-A");
mkvmerge_arg!(SubTracks, "-s");
mkvmerge_no_arg!(SubTracks, "-S");
mkvmerge_arg!(VideoTracks, "-d");
mkvmerge_no_arg!(VideoTracks, "-D");
mkvmerge_arg!(ButtonTracks, "-b");
mkvmerge_no_arg!(ButtonTracks, "-B");

macro_rules! tracks_to_mkvmerge_args {
    ( $( $type:ident, $track_type:expr => $arg:ident, $no_arg:ident, )* ) => {
        $(
            impl ToMkvmergeArgs for $type {
                fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
                    if self.is_default() {
                        Vec::new()
                    } else if self.no_flag {
                        vec![Self::MKVMERGE_NO_ARG.into()]
                    } else {
                        let nums: String = unwrap_or_return_vec!(
                            mi.get::<MISavedTracks>(path)
                        )[$track_type]
                            .iter()
                            .map(|num| num.to_mkvmerge_arg())
                            .collect::<Vec<_>>()
                            .join(",");

                        if nums.is_empty() {
                            vec![Self::MKVMERGE_NO_ARG.into()]
                        } else {
                            vec![Self::MKVMERGE_ARG.into(), nums]
                        }
                    }
                }

                to_mkvmerge_args!(@fn_os);
            }
        )*
    };
}

tracks_to_mkvmerge_args!(
    AudioTracks, TrackType::Audio => Audio, NoAudio,
    SubTracks, TrackType::Sub => Subs, NoSubs,
    VideoTracks, TrackType::Video => Video, NoVideo,
    ButtonTracks, TrackType::Button => Buttons, NoButtons,
);
