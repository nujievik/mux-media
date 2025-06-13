use super::{
    AudioTracks, ButtonTracks, DefaultTFlags, EnabledTFlags, ForcedTFlags, SubTracks, TrackLangs,
    TrackNames, VideoTracks,
};
use crate::{CLIArg, cli_args, from_arg_matches};
use clap::{ArgMatches, Error, FromArgMatches};

cli_args!(AudioTracks, AudioTracksArg; Audio => "audio", "-a", NoAudio => "no-audio", "-A");
cli_args!(SubTracks, SubTracksArg; Subs => "subs", "-s", NoSubs => "no-subs", "-S");
cli_args!(VideoTracks, VideoTracksArg; Video => "video", "-d", NoVideo => "no-video", "-D");
cli_args!(ButtonTracks, ButtonTracksArg; Buttons => "buttons", "-b",
          NoButtons => "no-buttons", "-B");

cli_args!(DefaultTFlags, DefaultTFlagsArg; Defaults => "defaults", "--default-track-flag",
          LimDefaults => "lim-defaults", "");
cli_args!(ForcedTFlags, ForcedTFlagsArg; Forceds => "forceds", "--forced-display-flag",
          LimForceds => "lim-forceds", "");
cli_args!(EnabledTFlags, EnabledTFlagsArg; Enableds => "enableds", "--track-enabled-flag",
          LimEnableds => "lim-enableds", "");

cli_args!(TrackNames, TrackNamesArg; Names => "names", "--track-name");
cli_args!(TrackLangs, TrackLangsArg; Langs => "langs", "--language");

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

impl FromArgMatches for TrackNames {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(from_arg_matches!(matches, Self, Names, Self::default))
    }
}

impl FromArgMatches for TrackLangs {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(from_arg_matches!(matches, Self, Langs, Self::default))
    }
}

macro_rules! flags_from_arg_matches {
    ($type:ident, $arg:ident, $lim_arg:ident) => {
        impl FromArgMatches for $type {
            from_arg_matches!(@unrealized_fns);

            fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
                let lim = from_arg_matches!(matches, u32, $lim_arg, @no_default);
                Ok(from_arg_matches!(matches, $type, $arg, Self::default).lim_for_unset(lim))
            }
        }
    };
}

flags_from_arg_matches!(DefaultTFlags, Defaults, LimDefaults);
flags_from_arg_matches!(ForcedTFlags, Forceds, LimForceds);
flags_from_arg_matches!(EnabledTFlags, Enableds, LimEnableds);
