use super::{AudioTracks, SubTracks, VideoTracks};

to_json_args!(@tracks_or_attachs, AudioTracks, Audio, NoAudio);
to_json_args!(@tracks_or_attachs, SubTracks, Subs, NoSubs);
to_json_args!(@tracks_or_attachs, VideoTracks, Video, NoVideo);
