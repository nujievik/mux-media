use super::super::{
    Config, ConfigAutoFlags, ConfigChapters, ConfigDispositions, ConfigInput, ConfigLangMetadata,
    ConfigLocales, ConfigLogLevel, ConfigOutput, ConfigRetiming, ConfigStreams, ConfigStreamsOrder,
    ConfigSubsEncodings, ConfigTarget, ConfigTitleMetadata, ConfigTsOffsets, InputType,
};
use super::{LogLang, Target};
use crate::{
    CliArg, Extension, GlobSetPattern, IsDefault, LangCode, Msg, MuxError, RangeUsize, StreamType,
    VERSION, Value, undashed,
};
use clap::{ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser};
use log::LevelFilter;
use std::{ffi::CStr, path::PathBuf};

macro_rules! rm {
    ($matches:ident, $arg:ident, $ty:ty) => {
        $matches.remove_one::<$ty>(undashed!($arg))
    };
}

macro_rules! rm_or {
    ($matches:ident, $arg:ident, $ty:ty, $op:expr) => {
        rm!($matches, $arg, $ty).unwrap_or_else(|| $op())
    };
}

macro_rules! flag {
    ($matches:ident, $arg:ident) => {
        $matches.get_flag(undashed!($arg))
    };
}

macro_rules! get_streams {
    ($matches:ident, $arg:ident, $no_arg:ident) => {
        if flag!($matches, $no_arg) {
            let mut new = ConfigStreams::default();
            new.no_flag = true;
            Some(new)
        } else {
            rm!($matches, $arg, ConfigStreams)
        }
    };
}

macro_rules! streams {
    ($matches:ident, $arg:ident, $no_arg:ident) => {
        get_streams!($matches, $arg, $no_arg).unwrap_or_default()
    };
}

macro_rules! get_dispositions {
    ($matches:ident, $arg:ident, $max_arg:ident) => {{
        let max = rm!($matches, $max_arg, usize);
        if let Some(mut val) = rm!($matches, $arg, ConfigDispositions) {
            val.max_in_auto = max;
            Some(val)
        } else if let Some(max) = max {
            Some(ConfigDispositions {
                max_in_auto: Some(max),
                ..Default::default()
            })
        } else {
            None
        }
    }};
}

macro_rules! dispositions {
    ($matches:ident, $arg:ident, $lim_arg:ident) => {
        get_dispositions!($matches, $arg, $lim_arg).unwrap_or_else(|| ConfigDispositions::default())
    };
}

macro_rules! upd {
    ($field:expr, $matches:ident, $arg:ident, $ty:ty) => {
        if let Some(val) = rm!($matches, $arg, $ty) {
            $field = val;
        }
    };

    ($field:expr, $matches:ident, $arg:ident, $ty:ty, @opt) => {
        if let Some(val) = rm!($matches, $arg, $ty) {
            $field = Some(val);
        }
    };
}

macro_rules! upd_flag {
    ($field:expr, $matches:ident, $arg:ident) => {
        if flag!($matches, $arg) {
            $field = true;
        }
    };
}

macro_rules! upd_streams {
    ($field:expr, $matches:ident, $arg:ident, $no_arg:ident) => {
        if flag!($matches, $no_arg) {
            $field.no_flag = true;
        } else if let Some(val) = rm!($matches, $arg, ConfigStreams) {
            $field = val;
        }
    };
}

macro_rules! upd_dispositions {
    ($field:expr, $matches:ident, $arg:ident, $max_arg:ident) => {{
        if let Some(val) = rm!($matches, $arg, ConfigDispositions) {
            $field = val;
        }
        if let Some(max) = rm!($matches, $max_arg, usize) {
            $field.max_in_auto = Some(max);
        }
    }};
}

impl Parser for Config {}

impl FromArgMatches for Config {
    fn from_arg_matches(m: &ArgMatches) -> Result<Self, Error> {
        Self::from_arg_matches_mut(&mut m.clone())
    }

    fn update_from_arg_matches(&mut self, m: &ArgMatches) -> Result<(), Error> {
        self.update_from_arg_matches_mut(&mut m.clone())
    }

    fn from_arg_matches_mut(m: &mut ArgMatches) -> Result<Self, Error> {
        let locale = locale(m);

        printable_args(&m)?;
        let mut cfg = cfg(m, locale)?;

        if m.contains_id(undashed!(Target)) {
            cfg.update_from_arg_matches_mut(m)?;
        }

        return Ok(cfg);

        fn cfg(m: &mut ArgMatches, locales: ConfigLocales) -> Result<Config, Error> {
            let input = try_input(m)?;

            let output = match rm!(m, Output, ConfigOutput) {
                Some(o) => o,
                None => ConfigOutput::try_from(&input)?,
            };

            let mut cfg = Config {
                input,
                output,

                locales,
                log_level: log_level(m),
                exit_on_err: flag!(m, ExitOnErr),
                save_config: flag!(m, SaveConfig),
                jobs: rm_or!(m, Jobs, u8, || Config::JOBS_DEFAULT),

                auto_flags: auto_flags(m),
                clean_flags: flag!(m, CleanFlags),
                clean_meta: flag!(m, CleanMeta),

                streams: streams!(m, Streams, NoStreams),
                chapters: get_chapters(m).unwrap_or_else(|| ConfigChapters::default()),
                order: rm_or!(m, Order, ConfigStreamsOrder, ConfigStreamsOrder::default),
                defaults: dispositions!(m, Defaults, MaxDefaults),
                forceds: dispositions!(m, Forceds, MaxForceds),
                titles: rm_or!(m, Titles, ConfigTitleMetadata, ConfigTitleMetadata::default),
                langs: rm_or!(m, Langs, ConfigLangMetadata, ConfigLangMetadata::default),
                subs_encodings: rm_or!(
                    m,
                    SubsEncodings,
                    ConfigSubsEncodings,
                    ConfigSubsEncodings::default
                ),
                ts_offsets: rm_or!(m, TsOffsets, ConfigTsOffsets, ConfigTsOffsets::default),
                retiming: retiming(m),

                per_stream_type: Default::default(),
                by_path: Default::default(),

                info: flag!(m, Info),
            };

            upd_per_stream_type_streams_from_global_aliases(&mut cfg, m);
            Ok(cfg)
        }

        fn locale(m: &ArgMatches) -> ConfigLocales {
            let (locale, locale_log) = update_log_language(m);
            let locale = locale.unwrap_or_else(|| Msg::lang());
            let locale_streams = m.get_one::<LangCode>(undashed!(LocaleStreams)).copied();

            ConfigLocales {
                locale,
                locale_streams,
                locale_log,
            }
        }

        fn log_level(m: &mut ArgMatches) -> ConfigLogLevel {
            if flag!(m, Quiet) {
                ConfigLogLevel(LevelFilter::Error)
            } else {
                ConfigLogLevel::from_count(m.get_count(undashed!(Verbose)))
            }
        }

        fn auto_flags(m: &mut ArgMatches) -> ConfigAutoFlags {
            let mut new = ConfigAutoFlags::default();

            if flag!(m, NoAuto) {
                new.no_auto = true;
            }
            let no_auto = new.no_auto;

            new.defaults = val(flag!(m, AutoDefaults), flag!(m, NoAutoDefaults), no_auto);
            new.forceds = val(flag!(m, AutoForceds), flag!(m, NoAutoForceds), no_auto);
            new.titles = val(flag!(m, AutoTitles), flag!(m, NoAutoTitles), no_auto);
            new.langs = val(flag!(m, AutoLangs), flag!(m, NoAutoLangs), no_auto);
            new.subs_encodings = val(
                flag!(m, AutoSubsEncodings),
                flag!(m, NoAutoSubsEncodings),
                no_auto,
            );
            new.ts_offsets = val(flag!(m, AutoTsOffsets), flag!(m, NoAutoTsOffsets), no_auto);

            return new;

            fn val(arg: bool, no_arg: bool, no_auto: bool) -> Value<bool> {
                if arg {
                    Value::User(true)
                } else if no_arg {
                    Value::User(false)
                } else {
                    Value::Auto(!no_auto)
                }
            }
        }

        fn retiming(m: &mut ArgMatches) -> ConfigRetiming {
            let mut opts = rm_or!(m, Parts, ConfigRetiming, ConfigRetiming::default);
            opts.no_linked = flag!(m, NoLinked);
            opts
        }
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        locale(self, m);
        input(self, m);
        output(self, m);
        log_level(self, m);

        upd_flag!(self.exit_on_err, m, ExitOnErr);
        upd_flag!(self.save_config, m, SaveConfig);
        upd!(self.jobs, m, Jobs, u8);

        auto_flags(self, m);
        upd_flag!(self.clean_flags, m, CleanFlags);
        upd_flag!(self.clean_meta, m, CleanMeta);
        upd_per_stream_type_streams_from_global_aliases(self, m);

        upd_streams!(self.streams, m, Streams, NoStreams);
        upd_chapters(&mut self.chapters, m);
        upd!(self.order, m, Order, ConfigStreamsOrder);
        upd_dispositions!(self.defaults, m, Defaults, MaxDefaults);
        upd_dispositions!(self.forceds, m, Forceds, MaxForceds);
        upd!(self.titles, m, Titles, ConfigTitleMetadata);
        upd!(self.langs, m, Langs, ConfigLangMetadata);
        upd!(self.subs_encodings, m, SubsEncodings, ConfigSubsEncodings);
        upd!(self.ts_offsets, m, TsOffsets, ConfigTsOffsets);
        upd_flag!(self.info, m, Info);

        retiming_options(self, m);

        let mut m: &mut ArgMatches = m;
        let mut _owned_m: Option<ArgMatches> = None;
        let mut cmd: Option<Command> = None;

        while let Some(mut t_args) = m.get_raw(undashed!(Target)) {
            // unwrap is safe: target require as min 1 argument.
            let target = t_args.next().unwrap();
            let target = Target::parse_os(target)?;

            if let Target::Global = &target {
                return self.try_update_from(t_args);
            }

            let matches = cmd
                .get_or_insert_with(|| ConfigTarget::command())
                .clone()
                .try_get_matches_from(t_args)?;
            _owned_m = Some(matches);
            m = _owned_m.as_mut().unwrap();

            match target {
                Target::Global => unreachable!(), // already processed
                Target::StreamType(ty) => {
                    self.per_stream_type[ty].update_from_arg_matches_mut(m)?
                }
                Target::Path(path) => match self.by_path.entry(path) {
                    indexmap::map::Entry::Occupied(mut entry) => {
                        entry.get_mut().update_from_arg_matches_mut(m)?;
                    }
                    indexmap::map::Entry::Vacant(entry) => {
                        let val = ConfigTarget::from_arg_matches_mut(m)?;
                        entry.insert(val);
                    }
                },
            }
        }

        return Ok(());

        fn locale(cfg: &mut Config, m: &mut ArgMatches) {
            let (locale, locale_log) = update_log_language(m);

            if let Some(l) = locale {
                cfg.locales.locale = l;
            }

            if let Some(l) = locale_log {
                cfg.locales.locale_log = Some(l);
            }

            if let Some(l) = m.get_one::<LangCode>(undashed!(LocaleStreams)) {
                cfg.locales.locale_streams = Some(*l);
            }
        }

        fn input(cfg: &mut Config, m: &mut ArgMatches) {
            let input = &mut cfg.input;

            if let Some(Ok(ty)) = try_input_ty(m) {
                input.ty = ty;
            }

            upd_flag!(input.overwrite, m, Overwrite);
            upd!(input.range, m, Range, RangeUsize, @opt);
            upd!(input.skip, m, Skip, GlobSetPattern, @opt);
            upd!(input.depth, m, Depth, u8);

            upd_flag!(input.orphans_too, m, OrphansToo);

            if input.file_dirs.values().any(|v| !v.is_empty()) {
                input.file_dirs = Default::default();
            }
        }

        fn output(cfg: &mut Config, m: &mut ArgMatches) {
            if let Some(output) = rm!(m, Output, ConfigOutput) {
                cfg.output = output;
            }
        }

        fn log_level(cfg: &mut Config, m: &mut ArgMatches) {
            if flag!(m, Quiet) {
                cfg.log_level = ConfigLogLevel(LevelFilter::Error);
            } else {
                match m.get_count(undashed!(Verbose)) {
                    0 => (),
                    cnt => cfg.log_level = ConfigLogLevel::from_count(cnt),
                }
            }
        }

        fn auto_flags(cfg: &mut Config, m: &mut ArgMatches) {
            let auto = &mut cfg.auto_flags;

            if flag!(m, NoAuto) {
                auto.no_auto = true;
            }
            let no_auto = auto.no_auto;

            upd(
                flag!(m, AutoDefaults),
                flag!(m, NoAutoDefaults),
                no_auto,
                &mut auto.defaults,
            );
            upd(
                flag!(m, AutoForceds),
                flag!(m, NoAutoForceds),
                no_auto,
                &mut auto.forceds,
            );

            upd(
                flag!(m, AutoTitles),
                flag!(m, NoAutoTitles),
                no_auto,
                &mut auto.titles,
            );
            upd(
                flag!(m, AutoLangs),
                flag!(m, NoAutoLangs),
                no_auto,
                &mut auto.langs,
            );
            upd(
                flag!(m, AutoSubsEncodings),
                flag!(m, NoAutoSubsEncodings),
                no_auto,
                &mut auto.subs_encodings,
            );
            upd(
                flag!(m, AutoTsOffsets),
                flag!(m, NoAutoTsOffsets),
                no_auto,
                &mut auto.ts_offsets,
            );

            fn upd(arg: bool, no_arg: bool, no_auto: bool, val: &mut Value<bool>) {
                if arg {
                    *val = Value::User(true)
                } else if no_arg {
                    *val = Value::User(false)
                } else if val.is_auto() {
                    *val = Value::Auto(!no_auto)
                }
            }
        }

        fn retiming_options(cfg: &mut Config, m: &mut ArgMatches) {
            upd!(cfg.retiming, m, Parts, ConfigRetiming);
            upd_flag!(cfg.retiming.no_linked, m, NoLinked);
        }
    }
}

macro_rules! upd_per_stream_type_streams_from_global_aliases {
    ($cfg:expr, $matches:ident; $( $ty:ident, $arg:ident, $no_arg:ident );* $(;)?) => {{ $(
        let cfg = &mut $cfg.per_stream_type[StreamType::$ty];

        if let Some(v) = cfg.streams.as_mut() {
            upd_streams!(*v, $matches, $arg, $no_arg);
        } else if let Some(v) = get_streams!($matches, $arg, $no_arg) {
            cfg.streams = Some(v);
        }
    )* }};
}
fn upd_per_stream_type_streams_from_global_aliases(cfg: &mut Config, m: &mut ArgMatches) {
    upd_per_stream_type_streams_from_global_aliases!(
        cfg, m;
        Audio, Audio, NoAudio;
        Subtitles, Subs, NoSubs;
        Video, Video, NoVideo;
        AttachedFont, Fonts, NoFonts;
        AttachedPic, Pics, NoPics;
    );
}

// Returns (locale, locale_log)
pub(super) fn update_log_language(m: &ArgMatches) -> (Option<LangCode>, Option<LangCode>) {
    let locale = m.get_one::<LangCode>(undashed!(Locale)).copied();
    let log = m.get_one::<LogLang>(undashed!(LocaleLog)).copied();

    let msg_lang = match (locale, log) {
        (_, Some(l)) => Some(l.0),
        (Some(l), None) => Some(l),
        (None, None) => None,
    };

    if let Some(l) = msg_lang {
        Msg::upd_lang_or_warn(l);
    }

    (locale, log.map(|l| l.0))
}

pub(super) fn printable_args(m: &ArgMatches) -> Result<(), Error> {
    arg(m, CliArg::ListTargets, Target::print_list_targets)?;
    arg(m, CliArg::ListLangs, LangCode::print_list_langs)?;

    arg(m, CliArg::Version, || {
        println!("{}", VERSION);

        // do not print ffmpeg version if not verbose
        if m.get_count(undashed!(Verbose)).is_default() {
            return;
        }

        unsafe {
            use ffmpeg_next::sys;

            let ver = CStr::from_ptr(sys::av_version_info()).to_string_lossy();

            #[allow(unused_mut)]
            let mut config = CStr::from_ptr(sys::avcodec_configuration()).to_string_lossy();
            #[cfg(feature = "ffmpeg-static")]
            {
                config = config
                    .split_whitespace()
                    .filter(|arg| !arg.starts_with("--prefix"))
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .into();
            }

            println!("\nffmpeg version {}", ver);
            println!("  configuration: {}", config);
            println!(
                "  libavutil    {}. {}.{}",
                sys::LIBAVUTIL_VERSION_MAJOR,
                sys::LIBAVUTIL_VERSION_MINOR,
                sys::LIBAVUTIL_VERSION_MICRO
            );
            println!(
                "  libavcodec   {}. {}.{}",
                sys::LIBAVCODEC_VERSION_MAJOR,
                sys::LIBAVCODEC_VERSION_MINOR,
                sys::LIBAVCODEC_VERSION_MICRO
            );
            println!(
                "  libavformat  {}. {}.{}",
                sys::LIBAVFORMAT_VERSION_MAJOR,
                sys::LIBAVFORMAT_VERSION_MINOR,
                sys::LIBAVFORMAT_VERSION_MICRO
            );
        }
    })?;

    arg(m, CliArg::Help, || {
        let mut cmd = Config::command();
        if let Err(_) = cmd.print_help() {
            println!("{}", cmd.render_help());
        }
    })?;

    return Ok(());

    fn arg<F>(m: &ArgMatches, arg: CliArg, print: F) -> Result<(), Error>
    where
        F: FnOnce(),
    {
        if m.get_flag(arg.undashed()) {
            print();
            Err(MuxError::new_ok().into())
        } else {
            Ok(())
        }
    }
}

fn try_input(m: &mut ArgMatches) -> Result<ConfigInput, Error> {
    let ty = match try_input_ty(m) {
        Some(res) => res?,
        None => InputType::Dir(ConfigInput::try_default_dir()?),
    };

    Ok(ConfigInput {
        ty,
        overwrite: flag!(m, Overwrite),
        range: rm!(m, Range, RangeUsize),
        skip: rm!(m, Skip, GlobSetPattern),
        depth: rm_or!(m, Depth, u8, || ConfigInput::DEPTH_DEFAULT),
        orphans_too: flag!(m, OrphansToo),
        file_dirs: Default::default(),
    })
}

fn try_input_ty(m: &mut ArgMatches) -> Option<Result<InputType, Error>> {
    let mut paths: Vec<_> = m.remove_many::<PathBuf>(undashed!(Input))?.collect();

    if paths.is_empty() {
        None
    } else if paths.len() > 1 && paths.iter().any(|x| x.is_dir()) {
        Some(Err(err!("must be only 1 directory").into()))
    } else if paths.len() == 1 && paths[0].is_dir() {
        Some(Ok(InputType::Dir(paths.pop().unwrap())))
    } else if !paths
        .iter()
        .any(|p| Extension::from_path(p).is_some_and(|ext| ext.is_media()))
    {
        Some(Err(err!("must be at least 1 media file").into()))
    } else {
        Some(Ok(InputType::Files(paths)))
    }
}

fn get_chapters(m: &mut ArgMatches) -> Option<ConfigChapters> {
    if flag!(m, NoChapters) {
        return Some(ConfigChapters {
            no_flag: true,
            ranges: None,
        });
    }

    rm!(m, Chapters, ConfigChapters)
}

fn upd_chapters(chp: &mut ConfigChapters, m: &mut ArgMatches) {
    if flag!(m, NoChapters) {
        chp.no_flag = true;
        return;
    }

    if let Some(new) = rm!(m, Chapters, ConfigChapters) {
        *chp = new;
    }
}

macro_rules! trg_upd_dispositions {
    ($field:expr, $matches:ident, $arg:ident, $lim_arg:ident) => {
        match $field.as_mut() {
            Some(f) => upd_dispositions!(*f, $matches, $arg, $lim_arg),
            None => $field = get_dispositions!($matches, $arg, $lim_arg),
        }
    };
}

impl FromArgMatches for ConfigTarget {
    fn from_arg_matches(m: &ArgMatches) -> Result<Self, Error> {
        Self::from_arg_matches_mut(&mut m.clone())
    }

    fn update_from_arg_matches(&mut self, m: &ArgMatches) -> Result<(), Error> {
        self.update_from_arg_matches_mut(&mut m.clone())
    }

    fn from_arg_matches_mut(m: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            streams: get_streams!(m, Streams, NoStreams),
            chapters: get_chapters(m),
            order: rm!(m, Order, ConfigStreamsOrder),
            defaults: get_dispositions!(m, Defaults, MaxDefaults),
            forceds: get_dispositions!(m, Forceds, MaxForceds),
            titles: rm!(m, Titles, ConfigTitleMetadata),
            langs: rm!(m, Langs, ConfigLangMetadata),
            subs_encodings: rm!(m, SubsEncodings, ConfigSubsEncodings),
            ts_offsets: rm!(m, TsOffsets, ConfigTsOffsets),
        })
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        trg_upd_streams(&mut self.streams, m);
        trg_upd_chapters(&mut self.chapters, m);

        upd!(self.order, m, Order, ConfigStreamsOrder, @opt);

        trg_upd_dispositions!(self.defaults, m, Defaults, MaxDefaults);
        trg_upd_dispositions!(self.forceds, m, Forceds, MaxForceds);

        upd!(self.titles, m, Titles, ConfigTitleMetadata, @opt);
        upd!(self.langs, m, Langs, ConfigLangMetadata, @opt);
        upd!(self.subs_encodings, m, SubsEncodings, ConfigSubsEncodings, @opt);
        upd!(self.ts_offsets, m, TsOffsets, ConfigTsOffsets, @opt);

        return Ok(());

        fn trg_upd_streams(streams: &mut Option<ConfigStreams>, m: &mut ArgMatches) {
            match streams {
                Some(xs) => upd_streams!(*xs, m, Streams, NoStreams),
                None => *streams = get_streams!(m, Streams, NoStreams),
            }
        }

        fn trg_upd_chapters(chp: &mut Option<ConfigChapters>, m: &mut ArgMatches) {
            match chp.as_mut() {
                Some(chp) => upd_chapters(chp, m),
                None => *chp = get_chapters(m),
            }
        }
    }
}
