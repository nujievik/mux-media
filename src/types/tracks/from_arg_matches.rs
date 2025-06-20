use super::{AudioTracks, ButtonTracks, SubTracks, VideoTracks};
use crate::{cli_args, from_arg_matches};
use clap::FromArgMatches;

cli_args!(AudioTracks, AudioTracksArg; Audio => "audio", NoAudio => "no-audio");
cli_args!(SubTracks, SubTracksArg; Subs => "subs", NoSubs => "no-subs");
cli_args!(VideoTracks, VideoTracksArg; Video => "video", NoVideo => "no-video");
cli_args!(ButtonTracks, ButtonTracksArg; Buttons => "buttons", NoButtons => "no-buttons");

impl FromArgMatches for AudioTracks {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Audio, NoAudio);
}

impl FromArgMatches for SubTracks {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Subs, NoSubs);
}

impl FromArgMatches for VideoTracks {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Video, NoVideo);
}

impl FromArgMatches for ButtonTracks {
    from_arg_matches!(@unrealized_fns);
    from_arg_matches!(@fn_mut, Buttons, NoButtons);
}
