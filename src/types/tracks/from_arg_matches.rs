use super::{AudioTracks, ButtonTracks, SubTracks, VideoTracks};
use crate::from_arg_matches;

from_arg_matches!(@impl, AudioTracks, Audio, NoAudio);
from_arg_matches!(@impl, SubTracks, Subs, NoSubs);
from_arg_matches!(@impl, VideoTracks, Video, NoVideo);
from_arg_matches!(@impl, ButtonTracks, Buttons, NoButtons);
