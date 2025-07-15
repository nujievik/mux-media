use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs, ForcedTFlags,
    Input, IsDefault, LangCode, Msg, OffOnPro, OtherAttachs, Output, Retiming, Specials, SubTracks,
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
}

impl FromArgMatches for MuxConfig {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let input = Input::from_arg_matches_mut(matches)?;

        let locale = match from_arg_matches!(matches, LangCode, Locale, @no_default) {
            Some(lang) => {
                let _ = Msg::try_upd_lang(lang);
                lang
            }
            None => Msg::get_lang(),
        };

        Ok(Self {
            output: from_arg_matches!(matches, Output, Output, || Output::try_from(&input), @try_default),
            input,
            locale,
            verbosity: Verbosity::from_arg_matches_mut(matches)?,
            no_json: from_arg_matches!(matches, bool, NoJson, || false),
            exit_on_err: from_arg_matches!(matches, bool, ExitOnErr, || false),
            off_on_pro: OffOnPro::from_arg_matches_mut(matches)?,
            retiming: Retiming::from_arg_matches_mut(matches)?,
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
            user_tools: from_arg_matches!(matches, bool, UserTools, || false),
            tools: Tools::default(),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        if let Some(lang) = from_arg_matches!(matches, LangCode, Locale, @no_default) {
            let _ = Msg::try_upd_lang(lang);
            self.locale = lang;
        }

        self.input.update_from_arg_matches_mut(matches)?;

        match from_arg_matches!(matches, Output, Output, @no_default) {
            Some(out) => self.output = out,
            None => self.output = Output::try_from(&self.input)?,
        }

        from_arg_matches!(
            @upd, self, matches;
            no_json, bool, NoJson,
            exit_on_err, bool, ExitOnErr,
            user_tools, bool, UserTools
        );

        self.off_on_pro.update_from_arg_matches_mut(matches)?;
        self.retiming.update_from_arg_matches_mut(matches)?;

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

macro_rules! upd_target_fields {
    ($self:ident, $matches:ident; $( $field:ident, $ty:ty ),* ) => {{
        $(
            let val = <$ty>::from_arg_matches_mut($matches)?;
            if !val.is_default() {
                $self.$field = Some(val);
            }
        )*
    }};
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
        upd_target_fields!(
            self, matches;
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
