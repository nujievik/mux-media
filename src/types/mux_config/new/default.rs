use crate::{IsDefault, MuxConfig};

impl MuxConfig {
    pub const JSON_NAME: &str = "mux-media.json";

    pub(crate) const THREADS_DEFAULT: u8 = 4;
}

impl Default for MuxConfig {
    fn default() -> MuxConfig {
        MuxConfig {
            input: Default::default(),
            output: Default::default(),
            locale: Default::default(),
            verbosity: Default::default(),
            exit_on_err: Default::default(),
            save_config: Default::default(),
            reencode: Default::default(),
            threads: Self::THREADS_DEFAULT,
            auto_flags: Default::default(),
            audio_tracks: Default::default(),
            sub_tracks: Default::default(),
            video_tracks: Default::default(),
            chapters: Default::default(),
            font_attachs: Default::default(),
            other_attachs: Default::default(),
            default_track_flags: Default::default(),
            forced_track_flags: Default::default(),
            enabled_track_flags: Default::default(),
            track_names: Default::default(),
            track_langs: Default::default(),
            specials: Default::default(),
            retiming: Default::default(),
            targets: Default::default(),
            tool_paths: Default::default(),
            muxer: Default::default(),
            is_output_constructed_from_input: Default::default(),
        }
    }
}

impl IsDefault for MuxConfig {
    fn is_default(&self) -> bool {
        self.input.is_default()
            && self.output.is_default()
            && self.locale.is_default()
            && self.verbosity.is_default()
            && self.exit_on_err.is_default()
            && self.save_config.is_default()
            && self.reencode.is_default()
            && self.threads == Self::THREADS_DEFAULT
            && self.auto_flags.is_default()
            && self.audio_tracks.is_default()
            && self.sub_tracks.is_default()
            && self.video_tracks.is_default()
            && self.chapters.is_default()
            && self.font_attachs.is_default()
            && self.other_attachs.is_default()
            && self.default_track_flags.is_default()
            && self.forced_track_flags.is_default()
            && self.enabled_track_flags.is_default()
            && self.track_names.is_default()
            && self.track_langs.is_default()
            && self.specials.is_default()
            && self.retiming.is_default()
            && self.targets.is_default()
            && self.tool_paths.is_default()
            && self.muxer.is_default()
            && self.is_output_constructed_from_input.is_default()
    }
}
