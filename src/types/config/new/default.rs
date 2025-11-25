use crate::{Config, IsDefault};

impl Config {
    pub const JSON_NAME: &str = "mux-media.json";

    pub(crate) const THREADS_DEFAULT: u8 = 1;
}

impl Default for Config {
    /// Returns a new [`Config`] with 4 [`Config::threads`] and all other default fields.
    /// ```
    /// use mux_media::Config;
    ///
    /// let c = Config::default();
    /// assert_eq!(c.input, Default::default());
    /// assert_eq!(c.output, Default::default());
    /// assert_eq!(c.locale, Default::default());
    /// assert_eq!(c.log_level, Default::default());
    /// assert_eq!(c.exit_on_err, false);
    /// assert_eq!(c.save_config, false);
    /// assert_eq!(c.reencode, false);
    /// assert_eq!(c.threads, 1);
    /// assert_eq!(c.auto_flags, Default::default());
    /// assert_eq!(c.streams, Default::default());
    /// assert_eq!(c.chapters, Default::default());
    /// assert_eq!(c.defaults, Default::default());
    /// assert_eq!(c.forceds, Default::default());
    /// assert_eq!(c.names, Default::default());
    /// assert_eq!(c.langs, Default::default());
    /// assert_eq!(c.retiming_options, Default::default());
    /// assert_eq!(c.targets, None);
    /// assert_eq!(c.tool_paths, Default::default());
    /// assert_eq!(c.muxer, Default::default());
    /// assert_eq!(c.is_output_constructed_from_input, false);
    /// ```
    fn default() -> Config {
        Config {
            input: Default::default(),
            output: Default::default(),
            locale: Default::default(),
            log_level: Default::default(),
            exit_on_err: Default::default(),
            save_config: Default::default(),
            reencode: Default::default(),
            threads: Self::THREADS_DEFAULT,
            auto_flags: Default::default(),
            streams: Default::default(),
            chapters: Default::default(),
            defaults: Default::default(),
            forceds: Default::default(),
            names: Default::default(),
            langs: Default::default(),
            retiming_options: Default::default(),
            targets: Default::default(),
            tool_paths: Default::default(),
            muxer: Default::default(),
            is_output_constructed_from_input: Default::default(),
        }
    }
}

impl IsDefault for Config {
    /// Returns `true` if all `self` fields eq [`Config::default`] fields.
    ///
    /// ```
    /// use mux_media::*;
    /// use is_default::IsDefault;
    ///
    /// assert!(Config::default().is_default());
    ///
    /// let input = Input { solo: true, ..Default::default() };
    /// assert!(!Config { input, ..Default::default() }.is_default());
    ///
    /// let output = Output { dir: "x".into(), ..Default::default() };
    /// assert!(!Config { output, ..Default::default() }.is_default());
    ///
    /// assert!(!Config { locale: LangCode::Eng, ..Default::default() }.is_default());
    /// assert!(!Config { log_level: LogLevel(log::LevelFilter::Error), ..Default::default() }.is_default());
    /// assert!(!Config { exit_on_err: true, ..Default::default() }.is_default());
    /// assert!(!Config { save_config: true, ..Default::default() }.is_default());
    /// assert!(!Config { reencode: true, ..Default::default() }.is_default());
    /// assert!(!Config { threads: 8, ..Default::default() }.is_default());
    ///
    /// let auto_flags = AutoFlags { pro: Value::User(true), ..Default::default() };
    /// assert!(!Config { auto_flags, ..Default::default() }.is_default());
    ///
    /// let streams = Streams { no_flag: true, ..Default::default() };
    /// assert!(!Config { streams, ..Default::default() }.is_default());
    ///
    /// let chapters = Chapters { no_flag: true, ..Default::default() };
    /// assert!(!Config { chapters, ..Default::default() }.is_default());
    ///
    /// let d = Dispositions { single_val: Some(true), ..Default::default() };
    /// assert!(!Config { defaults: DefaultDispositions(d.clone()), ..Default::default() }.is_default());
    /// assert!(!Config { forceds: ForcedDispositions(d), ..Default::default() }.is_default());
    ///
    /// let names = NameMetadata(Metadata { single_val: Some("x".into()), ..Default::default() });
    /// assert!(!Config { names, ..Default::default() }.is_default());
    ///
    /// let langs = LangMetadata(Metadata { single_val: Some(LangCode::Eng), ..Default::default() });
    /// assert!(!Config { langs, ..Default::default() }.is_default());
    ///
    /// let retiming_options = RetimingOptions { no_linked: true, ..Default::default() };
    /// assert!(!Config { retiming_options, ..Default::default() }.is_default());
    ///
    /// let tool_paths = ToolPaths { sys: true, ..Default::default() };
    /// assert!(!Config { tool_paths, ..Default::default() }.is_default());
    ///
    /// assert!(!Config { targets: Some(Default::default()), ..Default::default() }.is_default());
    /// assert!(!Config { muxer: Muxer::AVI, ..Default::default() }.is_default());
    /// assert!(!Config { is_output_constructed_from_input: true, ..Default::default() }.is_default());
    /// ```
    fn is_default(&self) -> bool {
        self.input.is_default()
            && self.output.is_default()
            && self.locale.is_default()
            && self.log_level.is_default()
            && self.exit_on_err.is_default()
            && self.save_config.is_default()
            && self.reencode.is_default()
            && self.threads == Self::THREADS_DEFAULT
            && self.auto_flags.is_default()
            && self.streams.is_default()
            && self.chapters.is_default()
            && self.defaults.is_default()
            && self.forceds.is_default()
            && self.names.is_default()
            && self.langs.is_default()
            && self.retiming_options.is_default()
            && self.targets.is_default()
            && self.tool_paths.is_default()
            && self.muxer.is_default()
            && self.is_output_constructed_from_input.is_default()
    }
}
