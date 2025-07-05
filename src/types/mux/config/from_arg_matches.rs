use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs, ForcedTFlags,
    Input, IsDefault, Msg, OffOnPro, OtherAttachs, Output, Retiming, Specials, SubTracks, Tool,
    Tools, TrackLangs, TrackNames, Verbosity, VideoTracks, from_arg_matches,
};
use clap::{ArgMatches, Error, FromArgMatches};

macro_rules! upd_fields {
    ($self:ident, $matches:ident; $( $field:ident, $ty:ty ),* ) => {{
        $(
            let val = <$ty>::from_arg_matches_mut($matches)?;
            if !val.is_default() {
                $self.$field = val;
            }
        )*
    }};

    ($self:ident, $matches:ident, @target; $( $field:ident, $ty:ty ),* ) => {{
        $(
            let val = <$ty>::from_arg_matches_mut($matches)?;
            if !val.is_default() {
                $self.$field = Some(val);
            }
        )*
    }};
}

impl FromArgMatches for MuxConfig {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let (input, output, retiming, tools) = try_io_rtm_tools(matches)?;

        Ok(Self {
            input,
            output,
            locale: Msg::get_lang_code(),
            verbosity: Verbosity::from_arg_matches_mut(matches)?,
            no_json: from_arg_matches!(matches, bool, NoConfig, || false),
            exit_on_err: from_arg_matches!(matches, bool, ExitOnErr, || false),
            off_on_pro: OffOnPro::from_arg_matches_mut(matches)?,
            retiming,
            audio_tracks: AudioTracks::from_arg_matches_mut(matches)?,
            sub_tracks: SubTracks::from_arg_matches_mut(matches)?,
            video_tracks: VideoTracks::from_arg_matches_mut(matches)?,
            button_tracks: ButtonTracks::from_arg_matches_mut(matches)?,
            chapters: Chapters::from_arg_matches_mut(matches)?,
            font_attachs: FontAttachs::from_arg_matches_mut(matches)?,
            other_attachs: OtherAttachs::from_arg_matches_mut(matches)?,
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

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        self.input.update_from_arg_matches_mut(matches)?;

        if let Some(output) = from_arg_matches!(matches, Output, Output, @no_default) {
            self.input.upd_out_need_num(output.need_num());
            self.tools.upd_json(Tools::make_json(output.get_temp_dir()));
            self.output = output;
        }

        self.locale = Msg::get_lang_code();

        self.retiming.update_from_arg_matches_mut(matches)?;

        if !self.retiming.is_default() {
            self.tools.try_upd_tool_path_if_none(Tool::Ffprobe)?;
        }

        upd_fields!(
            self, matches;
            verbosity, Verbosity,
            audio_tracks, AudioTracks,
            sub_tracks, SubTracks,
            video_tracks, VideoTracks,
            button_tracks, ButtonTracks,
            chapters, Chapters,
            font_attachs, FontAttachs,
            other_attachs, OtherAttachs,
            default_t_flags, DefaultTFlags,
            forced_t_flags, ForcedTFlags,
            enabled_t_flags, EnabledTFlags,
            track_names, TrackNames,
            track_langs, TrackLangs,
            specials, Specials
        );

        Ok(())
    }
}

fn try_io_rtm_tools(matches: &mut ArgMatches) -> Result<(Input, Output, Retiming, Tools), Error> {
    let mut input = Input::from_arg_matches_mut(matches)?;
    let output = from_arg_matches!(
        matches,
        Output,
        Output,
        || Output::try_from(&input), @try_default);
    let retiming = Retiming::from_arg_matches_mut(matches)?;
    let mut tools = Tools::try_from(Tool::iter_mkvtoolnix())?;

    input.upd_out_need_num(output.need_num());

    if !retiming.is_default() {
        tools.try_upd_tool_path(Tool::Ffprobe)?;
    }

    tools.upd_json(Tools::make_json(output.get_temp_dir()));

    Ok((input, output, retiming, tools))
}

impl FromArgMatches for MuxConfigTarget {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            audio_tracks: from_arg_matches!(matches, AudioTracks, @target),
            sub_tracks: from_arg_matches!(matches, SubTracks, @target),
            video_tracks: from_arg_matches!(matches, VideoTracks, @target),
            button_tracks: from_arg_matches!(matches, ButtonTracks, @target),
            chapters: from_arg_matches!(matches, Chapters, @target),
            font_attachs: from_arg_matches!(matches, FontAttachs, @target),
            other_attachs: from_arg_matches!(matches, OtherAttachs, @target),
            default_t_flags: from_arg_matches!(matches, DefaultTFlags, @target),
            forced_t_flags: from_arg_matches!(matches, ForcedTFlags, @target),
            enabled_t_flags: from_arg_matches!(matches, EnabledTFlags, @target),
            track_names: from_arg_matches!(matches, TrackNames, @target),
            track_langs: from_arg_matches!(matches, TrackLangs, @target),
            specials: from_arg_matches!(matches, Specials, @target),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        upd_fields!(
            self, matches, @target;
            audio_tracks, AudioTracks,
            sub_tracks, SubTracks,
            video_tracks, VideoTracks,
            button_tracks, ButtonTracks,
            chapters, Chapters,
            font_attachs, FontAttachs,
            other_attachs, OtherAttachs,
            default_t_flags, DefaultTFlags,
            forced_t_flags, ForcedTFlags,
            enabled_t_flags, EnabledTFlags,
            track_names, TrackNames,
            track_langs, TrackLangs,
            specials, Specials
        );

        Ok(())
    }
}
