use super::{AudioTracks, SubTracks, VideoTracks};
use crate::{
    IsDefault, MediaInfo, MuxConfigArg, MuxError, ParseableArg, ToMkvmergeArgs, TrackType,
    markers::MISavedTracks,
};
use std::{ffi::OsString, path::Path};

macro_rules! tracks_to_mkvmerge_args {
    ( $( $type:ident, $track_type:expr => $arg:ident, $no_arg:ident, )* ) => { $(
        impl ToMkvmergeArgs for $type {
            fn try_append_mkvmerge_args(&self, args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) -> Result<(), MuxError> {
                if self.is_default() {
                    return Ok(());
                }

                if self.no_flag {
                    args.push(MuxConfigArg::$no_arg.dashed().into());
                    return Ok(());
                }

                let tracks = &mi.try_get::<MISavedTracks>(path)?[$track_type];
                let tracks_len = tracks.len();

                let factor = match tracks_len {
                    0 => {
                        args.push(MuxConfigArg::$no_arg.dashed().into());
                        return Ok(());
                    }
                    _ => (tracks_len as f64).log10().floor() as usize + 2,
                };

                let mut arg = String::with_capacity(tracks_len * factor);
                tracks.iter().enumerate().for_each(|(i, track)| {
                    if i > 0 {
                        arg.push(',');
                    }
                    use std::fmt::Write;
                    let _ = write!(arg, "{}", track);
                });

                args.push(MuxConfigArg::$arg.dashed().into());
                args.push(arg.into());

                Ok(())
            }
        }
    )* };
}

tracks_to_mkvmerge_args!(
    AudioTracks, TrackType::Audio => AudioTracks, NoAudio,
    SubTracks, TrackType::Sub => SubtitleTracks, NoSubtitles,
    VideoTracks, TrackType::Video => VideoTracks, NoVideo,
);
