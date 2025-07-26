use super::{AudioTracks, ButtonTracks, SubTracks, VideoTracks};
use crate::to_json_args;

to_json_args!(@tracks_or_attachs, AudioTracks, Audio, NoAudio);
to_json_args!(@tracks_or_attachs, SubTracks, Subs, NoSubs);
to_json_args!(@tracks_or_attachs, VideoTracks, Video, NoVideo);
to_json_args!(@tracks_or_attachs, ButtonTracks, Buttons, NoButtons);
