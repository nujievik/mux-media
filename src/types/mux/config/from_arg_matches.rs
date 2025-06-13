use super::{MuxConfig, TargetMuxConfig};
use crate::{
    AudioTracks, ButtonTracks, CLIArg, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs,
    ForcedTFlags, Input, IsDefault, LangCode, OffOnPro, OtherAttachs, Output, Retiming, Specials,
    SubTracks, Tool, Tools, TrackLangs, TrackNames, Verbosity, VideoTracks, cli_args,
    from_arg_matches,
};

cli_args!(MuxConfig, MuxConfigArg; Output => "output", Locale => "locale",
          ExitOnErr => "exit-on-err");

impl clap::FromArgMatches for MuxConfig {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        let retiming = Retiming::from_arg_matches_mut(matches)?;

        let mut tools = Tools::try_from(Tool::iter_mkvtoolnix())?;
        if !retiming.is_default() {
            tools.try_insert(Tool::Ffprobe)?;
        }

        let mut input = Input::from_arg_matches_mut(matches)?;
        let output = from_arg_matches!(
            matches,
            Output,
            Output,
            || Output::try_from(&input), @try_default);
        if output.need_num() {
            input.upd_out_need_num(true);
        }

        tools.update_json(Tools::make_json(output.get_temp_dir()));

        Ok(Self {
            input,
            output,
            verbosity: Verbosity::from_arg_matches_mut(matches)?,
            locale: from_arg_matches!(matches, LangCode, Locale, LangCode::default),
            exit_on_err: from_arg_matches!(matches, bool, ExitOnErr, || false),
            off_on_pro: OffOnPro::from_arg_matches_mut(matches)?,
            retiming,
            audio_tracks: AudioTracks::from_arg_matches_mut(matches)?,
            sub_tracks: SubTracks::from_arg_matches_mut(matches)?,
            video_tracks: VideoTracks::from_arg_matches_mut(matches)?,
            button_tracks: ButtonTracks::from_arg_matches_mut(matches)?,
            chapters: Chapters::from_arg_matches_mut(matches)?,
            other_attachs: OtherAttachs::from_arg_matches_mut(matches)?,
            font_attachs: FontAttachs::from_arg_matches_mut(matches)?,
            default_t_flags: DefaultTFlags::from_arg_matches_mut(matches)?,
            forced_t_flags: ForcedTFlags::from_arg_matches_mut(matches)?,
            enabled_t_flags: EnabledTFlags::from_arg_matches_mut(matches)?,
            track_names: TrackNames::from_arg_matches_mut(matches)?,
            track_langs: TrackLangs::from_arg_matches_mut(matches)?,
            specials: Specials::from_arg_matches_mut(matches)?,
            targets: None,
            tools,
        })
    }
}

impl clap::FromArgMatches for TargetMuxConfig {
    from_arg_matches!(@unrealized_fns);

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self {
            audio_tracks: from_arg_matches!(matches, AudioTracks, @target),
            sub_tracks: from_arg_matches!(matches, SubTracks, @target),
            video_tracks: from_arg_matches!(matches, VideoTracks, @target),
            button_tracks: from_arg_matches!(matches, ButtonTracks, @target),
            chapters: from_arg_matches!(matches, Chapters, @target),
            other_attachs: from_arg_matches!(matches, OtherAttachs, @target),
            font_attachs: from_arg_matches!(matches, FontAttachs, @target),
            default_t_flags: from_arg_matches!(matches, DefaultTFlags, @target),
            forced_t_flags: from_arg_matches!(matches, ForcedTFlags, @target),
            enabled_t_flags: from_arg_matches!(matches, EnabledTFlags, @target),
            track_names: from_arg_matches!(matches, TrackNames, @target),
            track_langs: from_arg_matches!(matches, TrackLangs, @target),
            specials: from_arg_matches!(matches, Specials, @target),
        })
    }
}
