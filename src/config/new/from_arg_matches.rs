use super::super::{
    Config, ConfigAutoFlags, ConfigChapters, ConfigDispositions, ConfigInput, ConfigLangMetadata,
    ConfigLogLevel, ConfigNameMetadata, ConfigOutput, ConfigRetiming, ConfigStreams, ConfigTarget,
    InputType,
};
use crate::{
    CliArg, Extension, GlobSetPattern, LangCode, Msg, MuxError, RangeUsize, StreamType, Target,
    VERSION, Value, undashed,
};
use clap::{ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser};
use log::LevelFilter;
use std::{collections::HashMap, ffi::CStr, path::PathBuf};

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

macro_rules! trg_upd_streams {
    ($targets:expr, $k:expr, $matches:ident, $arg:ident, $no_arg:ident) => {
        let k = Target::Stream($k);

        if let Some(v) = $targets
            .as_mut()
            .and_then(|xs| xs.get_mut(&k))
            .and_then(|x| x.streams.as_mut())
        {
            upd_streams!(*v, $matches, $arg, $no_arg);
        }

        if let Some(v) = get_streams!($matches, $arg, $no_arg) {
            if let Some(trg) = $targets.as_mut().and_then(|xs| xs.get_mut(&k)) {
                trg.streams = Some(v);
            } else {
                let v = ConfigTarget {
                    streams: Some(v),
                    ..Default::default()
                };
                $targets.get_or_insert_default().insert(k, v);
            }
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
        let locale = get_locale(&m).unwrap_or_else(|| Msg::lang());
        printable_args(&m)?;
        let mut cfg = cfg(m, locale)?;

        if m.contains_id(undashed!(Target)) {
            cfg.update_from_arg_matches_mut(m)?;
        }

        return Ok(cfg);

        fn cfg(m: &mut ArgMatches, locale: LangCode) -> Result<Config, Error> {
            let input = try_input(m)?;

            let (output, is_output_constructed_from_input) = match rm!(m, Output, ConfigOutput) {
                Some(out) => (out, false),
                None => (ConfigOutput::try_from(&input)?, true),
            };

            Ok(Config {
                input,
                output,
                locale,
                log_level: log_level(m),
                exit_on_err: flag!(m, ExitOnErr),
                save_config: flag!(m, SaveConfig),
                jobs: rm_or!(m, Jobs, u8, || Config::JOBS_DEFAULT),
                auto_flags: auto_flags(m),
                streams: streams!(m, Streams, NoStreams),
                chapters: get_chapters(m).unwrap_or_else(|| ConfigChapters::default()),
                defaults: dispositions!(m, Defaults, MaxDefaults),
                forceds: dispositions!(m, Forceds, MaxForceds),
                names: rm_or!(m, Names, ConfigNameMetadata, ConfigNameMetadata::default),
                langs: rm_or!(m, Langs, ConfigLangMetadata, ConfigLangMetadata::default),
                retiming: retiming(m),
                targets: targets(m),
                is_output_constructed_from_input,
            })
        }

        fn log_level(m: &mut ArgMatches) -> ConfigLogLevel {
            if flag!(m, Quiet) {
                ConfigLogLevel(LevelFilter::Error)
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                ConfigLogLevel::from_count(cnt)
            } else {
                ConfigLogLevel::default()
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
            new.names = val(flag!(m, AutoNames), flag!(m, NoAutoNames), no_auto);
            new.langs = val(flag!(m, AutoLangs), flag!(m, NoAutoLangs), no_auto);
            new.encs = val(flag!(m, AutoEncs), flag!(m, NoAutoEncs), no_auto);

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

        fn targets(m: &mut ArgMatches) -> Option<HashMap<Target, ConfigTarget>> {
            let mut map: Option<HashMap<Target, ConfigTarget>> = None;

            let mut insert_some = |k, v: Option<ConfigStreams>| {
                if v.is_none() {
                    return;
                }
                let k = Target::Stream(k);
                let v = ConfigTarget {
                    streams: v,
                    ..Default::default()
                };
                map.get_or_insert_default().insert(k, v);
            };

            insert_some(StreamType::Audio, get_streams!(m, Audio, NoAudio));
            insert_some(StreamType::Sub, get_streams!(m, Subs, NoSubs));
            insert_some(StreamType::Video, get_streams!(m, Video, NoVideo));
            insert_some(StreamType::Font, get_streams!(m, Fonts, NoFonts));
            insert_some(StreamType::Attach, get_streams!(m, Attachs, NoAttachs));

            map
        }
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        if let Some(l) = get_locale(m) {
            self.locale = l;
        }

        input(self, m);
        output(self, m);
        log_level(self, m);

        upd_flag!(self.exit_on_err, m, ExitOnErr);
        upd_flag!(self.save_config, m, SaveConfig);
        upd!(self.jobs, m, Jobs, u8);

        auto_flags(self, m);
        if !m.contains_id(undashed!(Target)) {
            upd_streams!(self.streams, m, Streams, NoStreams);
        }
        upd_chapters(&mut self.chapters, m);

        upd_dispositions!(self.defaults, m, Defaults, MaxDefaults);
        upd_dispositions!(self.forceds, m, Forceds, MaxForceds);

        retiming_options(self, m);
        targets(self, m);

        let mut m: &mut ArgMatches = m;
        let mut _owned_m: Option<ArgMatches> = None;
        let mut cmd: Option<Command> = None;

        while let Some(mut t_args) = m.get_raw(undashed!(Target)) {
            // unwrap is safe: target require as min 1 argument.
            let t = t_args.next().unwrap();
            let t = match self.get_key(t) {
                Some(t) => t,
                None => Target::new(t)?,
            };

            if let Target::Global = t {
                if flag!(m, NoStreams) {
                    self.streams.no_flag = true;
                } else if let Some(val) = m.get_one::<ConfigStreams>(undashed!(Streams)) {
                    self.streams = val.clone();
                }
                return self.try_update_from(t_args);
            }

            let matches = cmd
                .get_or_insert_with(|| ConfigTarget::command())
                .clone()
                .try_get_matches_from(t_args)?;
            _owned_m = Some(matches);
            m = _owned_m.as_mut().unwrap();

            if let Some(trg) = self.targets.as_mut().and_then(|map| map.get_mut(&t)) {
                trg.update_from_arg_matches_mut(m)?;
                continue;
            }

            let val = ConfigTarget::from_arg_matches_mut(m)?;

            match self.targets.as_mut() {
                Some(map) => {
                    map.insert(t, val);
                }
                None => {
                    let _ = self.targets.insert([(t, val)].into());
                }
            }
        }

        return Ok(());

        fn input(cfg: &mut Config, m: &mut ArgMatches) {
            let input = &mut cfg.input;

            if let Some(Ok(ty)) = try_input_ty(m) {
                input.ty = ty;
            }

            upd!(input.range, m, Range, RangeUsize, @opt);
            upd!(input.skip, m, Skip, GlobSetPattern, @opt);
            upd!(input.depth, m, Depth, u8);

            upd_flag!(input.solo, m, Solo);

            if input.file_dirs.values().any(|v| !v.is_empty()) {
                input.file_dirs = Default::default();
            }
        }

        fn output(cfg: &mut Config, m: &mut ArgMatches) {
            if let Some(output) = rm!(m, Output, ConfigOutput) {
                cfg.output = output;
                cfg.is_output_constructed_from_input = false;
            }
        }

        fn log_level(cfg: &mut Config, m: &mut ArgMatches) {
            if flag!(m, Quiet) {
                cfg.log_level = ConfigLogLevel(LevelFilter::Error);
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                cfg.log_level = ConfigLogLevel::from_count(cnt);
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
                flag!(m, AutoNames),
                flag!(m, NoAutoNames),
                no_auto,
                &mut auto.names,
            );
            upd(
                flag!(m, AutoLangs),
                flag!(m, NoAutoLangs),
                no_auto,
                &mut auto.langs,
            );
            upd(
                flag!(m, AutoEncs),
                flag!(m, NoAutoEncs),
                no_auto,
                &mut auto.encs,
            );

            fn upd(arg: bool, no_arg: bool, no_auto: bool, val: &mut Value<bool>) {
                if arg {
                    *val = Value::User(true)
                } else if no_arg {
                    *val = Value::User(false)
                } else if val.is_auto() {
                    *val = Value::User(!no_auto)
                }
            }
        }

        fn retiming_options(cfg: &mut Config, m: &mut ArgMatches) {
            upd!(cfg.retiming, m, Parts, ConfigRetiming);
            upd_flag!(cfg.retiming.no_linked, m, NoLinked);
        }

        fn targets(cfg: &mut Config, m: &mut ArgMatches) {
            let xs = &mut cfg.targets;
            trg_upd_streams!(xs, StreamType::Audio, m, Audio, NoAudio);
            trg_upd_streams!(xs, StreamType::Sub, m, Subs, NoSubs);
            trg_upd_streams!(xs, StreamType::Video, m, Video, NoVideo);
            trg_upd_streams!(xs, StreamType::Font, m, Fonts, NoFonts);
            trg_upd_streams!(xs, StreamType::Attach, m, Attachs, NoAttachs);
        }
    }
}

pub(super) fn get_locale(m: &ArgMatches) -> Option<LangCode> {
    match m.get_one::<LangCode>(undashed!(Locale)) {
        Some(&l) => {
            Msg::upd_lang_or_warn(l);
            Some(l)
        }
        None => None,
    }
}

pub(super) fn printable_args(m: &ArgMatches) -> Result<(), Error> {
    arg(m, CliArg::ListTargets, Target::print_list_targets)?;
    arg(m, CliArg::ListLangs, LangCode::print_list_langs)?;

    arg(m, CliArg::Version, || {
        println!("{}\n", VERSION);

        unsafe {
            use ffmpeg_next::sys;

            let ver = CStr::from_ptr(sys::av_version_info()).to_string_lossy();
            let config = CStr::from_ptr(sys::avcodec_configuration()).to_string_lossy();

            println!("ffmpeg version {}", ver);
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
        range: rm!(m, Range, RangeUsize),
        skip: rm!(m, Skip, GlobSetPattern),
        depth: rm_or!(m, Depth, u8, || ConfigInput::DEPTH_DEFAULT),
        solo: flag!(m, Solo),
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
        .any(|p| Extension::new_from_path(p).is_some_and(|ext| ext.is_media()))
    {
        Some(Err(err!("must be at least 1 media file").into()))
    } else {
        Some(Ok(InputType::Files(paths)))
    }
}

fn get_chapters(m: &mut ArgMatches) -> Option<ConfigChapters> {
    if flag!(m, NoChapters) {
        Some(ConfigChapters { no_flag: true })
    } else {
        None
    }
}

fn upd_chapters(chp: &mut ConfigChapters, m: &mut ArgMatches) {
    if flag!(m, NoChapters) {
        chp.no_flag = true;
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
            defaults: get_dispositions!(m, Defaults, MaxDefaults),
            forceds: get_dispositions!(m, Forceds, MaxForceds),
            names: rm!(m, Names, ConfigNameMetadata),
            langs: rm!(m, Langs, ConfigLangMetadata),
        })
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        trg_upd_streams(&mut self.streams, m);
        trg_upd_chapters(&mut self.chapters, m);

        trg_upd_dispositions!(self.defaults, m, Defaults, MaxDefaults);
        trg_upd_dispositions!(self.forceds, m, Forceds, MaxForceds);

        upd!(self.names, m, Names, ConfigNameMetadata, @opt);
        upd!(self.langs, m, Langs, ConfigLangMetadata, @opt);

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
