use super::super::{Config, ConfigTarget};
use crate::{
    AutoFlags, Chapters, CliArg, DefaultDispositions, Dispositions, ForcedDispositions,
    GlobSetPattern, Input, LangCode, LangMetadata, LogLevel, Msg, MuxError, NameMetadata, Output,
    RangeUsize, RetimingOptions, StreamType, Streams, Target, Tool, ToolPaths, Tools,
    TryFinalizeInit, Value, undashed,
};
use clap::{ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser};
use log::LevelFilter;
use std::{collections::HashMap, path::PathBuf};

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
            let mut new = Streams::default();
            new.no_flag = true;
            Some(new)
        } else {
            rm!($matches, $arg, Streams)
        }
    };
}

macro_rules! streams {
    ($matches:ident, $arg:ident, $no_arg:ident) => {
        get_streams!($matches, $arg, $no_arg).unwrap_or_default()
    };
}

macro_rules! get_dispositions {
    ($matches:ident, $arg:ident, $max_arg:ident, $ty:ident) => {{
        let max = rm!($matches, $max_arg, usize);
        if let Some(mut val) = rm!($matches, $arg, $ty) {
            val.0.max_in_auto = max;
            Some(val)
        } else if let Some(max) = max {
            Some($ty(Dispositions {
                max_in_auto: Some(max),
                ..Default::default()
            }))
        } else {
            None
        }
    }};
}

macro_rules! dispositions {
    ($matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {
        get_dispositions!($matches, $arg, $lim_arg, $ty).unwrap_or_else(|| $ty::default())
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
        } else if let Some(val) = rm!($matches, $arg, Streams) {
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
    ($field:expr, $matches:ident, $arg:ident, $max_arg:ident, $ty:ident) => {{
        if let Some(val) = rm!($matches, $arg, $ty) {
            $field = val;
        }
        if let Some(max) = rm!($matches, $max_arg, usize) {
            $field.0.max_in_auto = Some(max);
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
        tool_args(&m)?;
        let mut cfg = cfg(m, locale)?;

        if m.contains_id(undashed!(Target)) {
            cfg.update_from_arg_matches_mut(m)?;
        }

        return Ok(cfg);

        fn cfg(m: &mut ArgMatches, locale: LangCode) -> Result<Config, Error> {
            let input = try_input(m)?;

            let (output, is_output_constructed_from_input) = match rm!(m, Output, Output) {
                Some(out) => (out, false),
                None => (Output::try_from(&input)?, true),
            };

            Ok(Config {
                input,
                output,
                locale,
                log_level: log_level(m),
                exit_on_err: flag!(m, ExitOnErr),
                save_config: flag!(m, SaveConfig),
                reencode: flag!(m, Reencode),
                jobs: rm_or!(m, Jobs, u8, || Config::JOBS_DEFAULT),
                auto_flags: auto_flags(m),
                streams: streams!(m, Streams, NoStreams),
                chapters: get_chapters(m).unwrap_or_else(|| Chapters::default()),
                defaults: dispositions!(m, Defaults, MaxDefaults, DefaultDispositions),
                forceds: dispositions!(m, Forceds, MaxForceds, ForcedDispositions),
                names: rm_or!(m, Names, NameMetadata, NameMetadata::default),
                langs: rm_or!(m, Langs, LangMetadata, LangMetadata::default),
                retiming_options: retiming_options(m),
                targets: targets(m),
                tool_paths: tool_paths(m),
                muxer: Default::default(),
                is_output_constructed_from_input,
            })
        }

        fn log_level(m: &mut ArgMatches) -> LogLevel {
            if flag!(m, Quiet) {
                LogLevel(LevelFilter::Error)
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                LogLevel::from_count(cnt)
            } else {
                LogLevel::default()
            }
        }

        fn auto_flags(m: &mut ArgMatches) -> AutoFlags {
            let mut new = AutoFlags::default();

            if flag!(m, Pro) {
                new.pro = Value::User(true);
            }
            let pro = *new.pro;

            new.defaults = val(flag!(m, AutoDefaults), flag!(m, NoAutoDefaults), pro);
            new.forceds = val(flag!(m, AutoForceds), flag!(m, NoAutoForceds), pro);
            new.names = val(flag!(m, AutoNames), flag!(m, NoAutoNames), pro);
            new.langs = val(flag!(m, AutoLangs), flag!(m, NoAutoLangs), pro);
            new.encs = val(flag!(m, AutoEncs), flag!(m, NoAutoEncs), pro);

            return new;

            fn val(arg: bool, no_arg: bool, pro: bool) -> Value<bool> {
                if arg {
                    Value::User(true)
                } else if no_arg {
                    Value::User(false)
                } else {
                    Value::Auto(!pro)
                }
            }
        }

        fn retiming_options(m: &mut ArgMatches) -> RetimingOptions {
            let mut opts = rm_or!(m, Parts, RetimingOptions, RetimingOptions::default);
            opts.no_linked = flag!(m, NoLinked);
            opts
        }

        fn tool_paths(m: &mut ArgMatches) -> ToolPaths {
            ToolPaths {
                sys: flag!(m, Sys),
                ..Default::default()
            }
        }

        fn targets(m: &mut ArgMatches) -> Option<HashMap<Target, ConfigTarget>> {
            let mut map: Option<HashMap<Target, ConfigTarget>> = None;

            let mut insert_some = |k, v: Option<Streams>| {
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
        upd_flag!(self.reencode, m, Reencode);
        upd!(self.jobs, m, Jobs, u8);

        auto_flags(self, m);
        if !m.contains_id(undashed!(Target)) {
            upd_streams!(self.streams, m, Streams, NoStreams);
        }
        upd_chapters(&mut self.chapters, m);

        upd_dispositions!(self.defaults, m, Defaults, MaxDefaults, DefaultDispositions);
        upd_dispositions!(self.forceds, m, Forceds, MaxForceds, ForcedDispositions);

        retiming_options(self, m);
        targets(self, m);
        upd_flag!(self.tool_paths.sys, m, Sys);

        let mut m: &mut ArgMatches = m;
        let mut _owned_m: Option<ArgMatches> = None;
        let mut cmd: Option<Command> = None;

        while let Some(mut t_args) = m.get_raw(undashed!(Target)) {
            // unwrap is safe: target require as min 1 argument.
            let t = t_args.next().unwrap();
            let t = match self.get_key(t) {
                Some(t) => t,
                None => Target::from_os_str(t)?,
            };

            if let Target::Global = t {
                if flag!(m, NoStreams) {
                    self.streams.no_flag = true;
                } else if let Some(val) = m.get_one::<Streams>(undashed!(Streams)) {
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
            upd!(input.dir, m, Input, PathBuf);
            upd!(input.range, m, Range, RangeUsize, @opt);
            upd!(input.skip, m, Skip, GlobSetPattern, @opt);
            upd!(input.depth, m, Depth, u8);

            upd_flag!(input.solo, m, Solo);

            input.need_num = input.range.is_some();
            input.out_need_num = false;

            if input.dirs.values().any(|v| !v.is_empty()) {
                input.dirs = Default::default();
            }
        }

        fn output(cfg: &mut Config, m: &mut ArgMatches) {
            if let Some(output) = rm!(m, Output, Output) {
                cfg.output = output;
                cfg.is_output_constructed_from_input = false;
            }
        }

        fn log_level(cfg: &mut Config, m: &mut ArgMatches) {
            if flag!(m, Quiet) {
                cfg.log_level = LogLevel(LevelFilter::Error);
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                cfg.log_level = LogLevel::from_count(cnt);
            }
        }

        fn auto_flags(cfg: &mut Config, m: &mut ArgMatches) {
            let auto = &mut cfg.auto_flags;

            if flag!(m, Pro) {
                auto.pro = Value::User(true);
            }
            let pro = *auto.pro;

            upd(
                flag!(m, AutoDefaults),
                flag!(m, NoAutoDefaults),
                pro,
                &mut auto.defaults,
            );
            upd(
                flag!(m, AutoForceds),
                flag!(m, NoAutoForceds),
                pro,
                &mut auto.forceds,
            );

            upd(
                flag!(m, AutoNames),
                flag!(m, NoAutoNames),
                pro,
                &mut auto.names,
            );
            upd(
                flag!(m, AutoLangs),
                flag!(m, NoAutoLangs),
                pro,
                &mut auto.langs,
            );
            upd(
                flag!(m, AutoEncs),
                flag!(m, NoAutoEncs),
                pro,
                &mut auto.encs,
            );

            fn upd(arg: bool, no_arg: bool, pro: bool, val: &mut Value<bool>) {
                if arg {
                    *val = Value::User(true)
                } else if no_arg {
                    *val = Value::User(false)
                } else if val.is_auto() {
                    *val = Value::User(!pro)
                }
            }
        }

        fn retiming_options(cfg: &mut Config, m: &mut ArgMatches) {
            upd!(cfg.retiming_options, m, Parts, RetimingOptions);
            upd_flag!(cfg.retiming_options.no_linked, m, NoLinked);
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
    arg(m, CliArg::ListContainers, || {
        println!("{}", Msg::ListContainers)
    })?;
    arg(m, CliArg::ListLangs, LangCode::print_list_langs)?;
    arg(m, CliArg::ListLangsFull, LangCode::print_list_langs_full)?;

    arg(m, CliArg::Version, || {
        let v = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));
        println!("{}", v);
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

pub(super) fn tool_args(m: &ArgMatches) -> Result<(), Error> {
    for t in Tool::iter() {
        let args = match m.get_raw(t.as_ref()) {
            Some(args) => args,
            None => continue,
        };
        let input = try_input(&mut m.clone())?;
        let mut output = Output::try_from(&input)?;
        output.try_finalize_init()?;

        let sys = m.get_flag(undashed!(Sys));
        let paths = ToolPaths {
            sys,
            ..Default::default()
        };
        paths.try_resolve(t, &output.temp_dir)?;
        let tools = Tools::from(&paths);

        let t_out = tools.run(t, args)?;
        println!("{}", &t_out);
        return Err(MuxError::from(t_out).into());
    }
    Ok(())
}

fn try_input(m: &mut ArgMatches) -> Result<Input, Error> {
    let dir = match rm!(m, Input, PathBuf) {
        Some(dir) => dir,
        None => Input::try_default_dir()?,
    };
    let range = rm!(m, Range, RangeUsize);

    Ok(Input {
        need_num: range.is_some(),
        dir,
        range,
        skip: rm!(m, Skip, GlobSetPattern),
        depth: rm_or!(m, Depth, u8, || Input::DEPTH_DEFAULT),
        solo: flag!(m, Solo),
        out_need_num: Default::default(),
        dirs: Default::default(),
    })
}

fn get_chapters(m: &mut ArgMatches) -> Option<Chapters> {
    if flag!(m, NoChapters) {
        Some(Chapters {
            no_flag: true,
            file: None,
        })
    } else {
        rm!(m, Chapters, Chapters)
    }
}

fn upd_chapters(chp: &mut Chapters, m: &mut ArgMatches) {
    if flag!(m, NoChapters) {
        chp.no_flag = true;
    } else {
        upd!(*chp, m, Chapters, Chapters);
    }
}

macro_rules! trg_upd_dispositions {
    ($field:expr, $matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {
        match $field.as_mut() {
            Some(f) => upd_dispositions!(*f, $matches, $arg, $lim_arg, $ty),
            None => $field = get_dispositions!($matches, $arg, $lim_arg, $ty),
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
            defaults: get_dispositions!(m, Defaults, MaxDefaults, DefaultDispositions),
            forceds: get_dispositions!(m, Forceds, MaxForceds, ForcedDispositions),
            names: rm!(m, Names, NameMetadata),
            langs: rm!(m, Langs, LangMetadata),
        })
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        trg_upd_streams(&mut self.streams, m);
        trg_upd_chapters(&mut self.chapters, m);

        trg_upd_dispositions!(self.defaults, m, Defaults, MaxDefaults, DefaultDispositions);
        trg_upd_dispositions!(self.forceds, m, Forceds, MaxForceds, ForcedDispositions);

        upd!(self.names, m, Names, NameMetadata, @opt);
        upd!(self.langs, m, Langs, LangMetadata, @opt);

        return Ok(());

        fn trg_upd_streams(streams: &mut Option<Streams>, m: &mut ArgMatches) {
            match streams {
                Some(xs) => upd_streams!(*xs, m, Streams, NoStreams),
                None => *streams = get_streams!(m, Streams, NoStreams),
            }
        }

        fn trg_upd_chapters(chp: &mut Option<Chapters>, m: &mut ArgMatches) {
            match chp.as_mut() {
                Some(chp) => upd_chapters(chp, m),
                None => *chp = get_chapters(m),
            }
        }
    }
}
