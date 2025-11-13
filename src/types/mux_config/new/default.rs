use crate::{IsDefault, MuxConfig};

impl MuxConfig {
    pub const JSON_NAME: &str = "mux-media.json";

    pub(crate) const THREADS_DEFAULT: u8 = 4;
}

impl Default for MuxConfig {
    /// Returns a new [`MuxConfig`] with 4 [`MuxConfig::threads`] and all other default fields.
    /// ```
    /// # use mux_media::MuxConfig;
    /// let c = MuxConfig::default();
    /// assert_eq!(c.input, Default::default());
    /// assert_eq!(c.output, Default::default());
    /// assert_eq!(c.locale, Default::default());
    /// assert_eq!(c.verbosity, Default::default());
    /// assert_eq!(c.exit_on_err, false);
    /// assert_eq!(c.save_config, false);
    /// assert_eq!(c.reencode, false);
    /// assert_eq!(c.threads, 4);
    /// assert_eq!(c.auto_flags, Default::default());
    /// assert_eq!(c.audio_tracks, Default::default());
    /// assert_eq!(c.sub_tracks, Default::default());
    /// assert_eq!(c.video_tracks, Default::default());
    /// assert_eq!(c.chapters, Default::default());
    /// assert_eq!(c.font_attachs, Default::default());
    /// assert_eq!(c.other_attachs, Default::default());
    /// assert_eq!(c.default_track_flags, Default::default());
    /// assert_eq!(c.forced_track_flags, Default::default());
    /// assert_eq!(c.track_names, Default::default());
    /// assert_eq!(c.track_langs, Default::default());
    /// assert_eq!(c.retiming, Default::default());
    /// assert_eq!(c.targets, None);
    /// assert_eq!(c.tool_paths, Default::default());
    /// assert_eq!(c.muxer, Default::default());
    /// assert_eq!(c.is_output_constructed_from_input, false);
    /// ```
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
            track_names: Default::default(),
            track_langs: Default::default(),
            retiming: Default::default(),
            targets: Default::default(),
            tool_paths: Default::default(),
            muxer: Default::default(),
            is_output_constructed_from_input: Default::default(),
        }
    }
}

impl IsDefault for MuxConfig {
    /// Returns `true` if all `self` fields eq [`MuxConfig::default`] fields.
    ///
    /// ```
    /// # use mux_media::*;
    /// assert!(MuxConfig::default().is_default());
    ///
    /// let input = Input { solo: true, ..Default::default() };
    /// assert!(!MuxConfig { input, ..Default::default() }.is_default());
    ///
    /// let output = Output { dir: "x".into(), ..Default::default() };
    /// assert!(!MuxConfig { output, ..Default::default() }.is_default());
    ///
    /// assert!(!MuxConfig { locale: LangCode::Eng, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { verbosity: Verbosity::Quiet, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { exit_on_err: true, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { save_config: true, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { reencode: true, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { threads: 1, ..Default::default() }.is_default());
    ///
    /// let auto_flags = AutoFlags { pro: Value::User(true), ..Default::default() };
    /// assert!(!MuxConfig { auto_flags, ..Default::default() }.is_default());
    ///
    /// let t = Tracks { no_flag: true, ..Default::default() };
    /// assert!(!MuxConfig { audio_tracks: AudioTracks(t.clone()), ..Default::default() }.is_default());
    /// assert!(!MuxConfig { sub_tracks: SubTracks(t.clone()), ..Default::default() }.is_default());
    /// assert!(!MuxConfig { video_tracks: VideoTracks(t.clone()), ..Default::default() }.is_default());
    ///
    /// let chapters = Chapters { no_flag: true, ..Default::default() };
    /// assert!(!MuxConfig { chapters, ..Default::default() }.is_default());
    ///
    /// let a = Attachs { no_flag: true, ..Default::default() };
    /// assert!(!MuxConfig { font_attachs: FontAttachs(a.clone()), ..Default::default() }.is_default());
    /// assert!(!MuxConfig { other_attachs: OtherAttachs(a.clone()), ..Default::default() }.is_default());
    ///
    /// let f = TrackFlags { unmapped: Some(true), ..Default::default() };
    /// assert!(!MuxConfig { default_track_flags: DefaultTrackFlags(f.clone()), ..Default::default() }.is_default());
    /// assert!(!MuxConfig { forced_track_flags: ForcedTrackFlags(f.clone()), ..Default::default() }.is_default());
    ///
    /// let track_names = TrackNames { unmapped: Some("x".into()), ..Default::default() };
    /// assert!(!MuxConfig { track_names, ..Default::default() }.is_default());
    ///
    /// let track_langs = TrackLangs { unmapped: Some(LangCode::Eng), ..Default::default() };
    /// assert!(!MuxConfig { track_langs, ..Default::default() }.is_default());
    ///
    /// let retiming = RetimingOptions { no_linked: true, ..Default::default() };
    /// assert!(!MuxConfig { retiming, ..Default::default() }.is_default());
    ///
    /// let tool_paths = ToolPaths { sys: true, ..Default::default() };
    /// assert!(!MuxConfig { tool_paths, ..Default::default() }.is_default());
    ///
    /// assert!(!MuxConfig { targets: Some(Default::default()), ..Default::default() }.is_default());
    /// assert!(!MuxConfig { muxer: Muxer::AVI, ..Default::default() }.is_default());
    /// assert!(!MuxConfig { is_output_constructed_from_input: true, ..Default::default() }.is_default());
    /// ```
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
            && self.track_names.is_default()
            && self.track_langs.is_default()
            && self.retiming.is_default()
            && self.targets.is_default()
            && self.tool_paths.is_default()
            && self.muxer.is_default()
            && self.is_output_constructed_from_input.is_default()
    }
}
