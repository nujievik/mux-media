use super::super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, AutoFlags, Chapters, DefaultTrackFlags, EnabledTrackFlags, FontAttachs,
    ForcedTrackFlags, GlobSetPattern, Input, LangCode, Msg, MuxConfigArg, MuxError,
    OtherAttachs, Output, ParseableArg, RangeU64, RetimingOptions, Specials, SubTracks, Target,
    TargetGroup, Tool, ToolPaths, Tools, TrackFlagType, TrackLangs, TrackNames, TryFinalizeInit,
    Verbosity, VideoTracks, TrackFlags,
};
use clap::{ArgMatches, Command, CommandFactory, Error, FromArgMatches, Parser};
use std::path::PathBuf;

macro_rules! rm {
    ($matches:ident, $arg:ident, $ty:ty) => {
        $matches.remove_one::<$ty>(MuxConfigArg::$arg.undashed())
    };
}

macro_rules! rm_or {
    ($matches:ident, $arg:ident, $ty:ty, $op:expr) => {
        rm!($matches, $arg, $ty).unwrap_or_else(|| $op())
    };
}

macro_rules! flag {
    ($matches:ident, $arg:ident) => { $matches.get_flag(MuxConfigArg::$arg.undashed()) };
}

macro_rules! get_tracks_or_attachs {
    ($matches:ident, $arg:ident, $no_arg:ident, $ty:ident) => {
        if flag!($matches, $no_arg) {
            let mut new = $ty::default();
            new.0.no_flag = true;
            Some(new)
        } else {
            rm!($matches, $arg, $ty)
        }
    };
}

macro_rules! tracks_or_attachs {
    ($matches:ident, $arg:ident, $no_arg:ident, $ty:ident) => {
        get_tracks_or_attachs!($matches, $arg, $no_arg, $ty).unwrap_or_else(|| $ty::default())
    };
}

macro_rules! get_track_flags {
    ($matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {{
        let lim = rm!($matches, $lim_arg, u64);
        if let Some(mut val) = rm!($matches, $arg, $ty) {
            val.0.lim_for_unset = lim;
            Some(val)
        } else if let Some(lim) = lim {
            Some($ty( TrackFlags { lim_for_unset: Some(lim), ..Default::default() } ))
        } else {
            None
        }
    }};
}

macro_rules! track_flags {
    ($matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {
        get_track_flags!($matches, $arg, $lim_arg, $ty).unwrap_or_else(|| $ty::default())
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

macro_rules! upd_tracks_or_attachs {
    ($field:expr, $matches:ident, $arg:ident, $no_arg:ident, $ty:ident) => {
        if flag!($matches, $no_arg) {
            $field.0.no_flag = true;
        } else if let Some(val) = rm!($matches, $arg, $ty) {
            $field = val;
        }
    };
}

macro_rules! upd_track_flags {
    ($field:expr, $matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {{
        if let Some(val) = rm!($matches, $arg, $ty) {
            $field = val;
        }
        if let Some(lim) = rm!($matches, $lim_arg, u64) {
            $field.0.lim_for_unset = Some(lim);
        }
    }};
}


impl Parser for MuxConfig {}

impl FromArgMatches for MuxConfig {
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

        if m.contains_id(MuxConfigArg::Target.undashed()) {
            cfg.update_from_arg_matches_mut(m)?;
        }

        return Ok(cfg);

        fn cfg(m: &mut ArgMatches, locale: LangCode) -> Result<MuxConfig, Error> {
            let input = try_input(m)?;

            let (output, is_output_constructed_from_input) = match rm!(m, Output, Output) {
                Some(out) => (out, false),
                None => (Output::try_from(&input)?, true),
            };

            Ok(MuxConfig {
                input,
                output,
                locale,
                verbosity: verbosity(m),
                exit_on_err: flag!(m, ExitOnErr),
                save_config: flag!(m, SaveConfig),
                reencode: flag!(m, Reencode),
                threads: rm_or!(m, Threads, u8, || MuxConfig::THREADS_DEFAULT),
                auto_flags: auto_flags(m),
                audio_tracks: tracks_or_attachs!(m, Audio, NoAudio, AudioTracks),
                sub_tracks: tracks_or_attachs!(m, Subs, NoSubs, SubTracks),
                video_tracks: tracks_or_attachs!(m, Video, NoVideo, VideoTracks),
                chapters: get_chapters(m).unwrap_or_else(|| Chapters::default()),
                font_attachs: tracks_or_attachs!(m, Fonts, NoFonts, FontAttachs),
                other_attachs: tracks_or_attachs!(m, Attachs, NoAttachs, OtherAttachs),
                default_track_flags: track_flags!(m, Defaults, MaxDefaults, DefaultTrackFlags),
                forced_track_flags: track_flags!(m, Forceds, MaxForceds, ForcedTrackFlags),
                enabled_track_flags: track_flags!(m, Enableds, MaxEnableds, EnabledTrackFlags),
                track_names: rm_or!(m, Names, TrackNames, TrackNames::default),
                track_langs: rm_or!(m, Langs, TrackLangs, TrackLangs::default),
                specials: rm_or!(m, Specials, Specials, Specials::default),
                retiming: retiming(m),
                targets: None,
                tool_paths: tool_paths(m),
                muxer: Default::default(),
                is_output_constructed_from_input,
            })
        }

        fn verbosity(m: &mut ArgMatches) -> Verbosity {
            if flag!(m, Quiet) {
                Verbosity::Quiet
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                Verbosity::from(cnt)
            } else {
                Verbosity::default()
            }
        }

        fn auto_flags(m: &mut ArgMatches) -> AutoFlags {
            let mut new = AutoFlags::default();
            new.pro = flag!(m, Pro);

            new.track[TrackFlagType::Default] =
                val(flag!(m, AutoDefaults), flag!(m, NoAutoDefaults), new.pro);
            new.track[TrackFlagType::Forced] =
                val(flag!(m, AutoForceds), flag!(m, NoAutoForceds), new.pro);
            new.track[TrackFlagType::Enabled] =
                val(flag!(m, AutoEnableds), flag!(m, NoAutoEnableds), new.pro);

            new.names = val(flag!(m, AutoNames), flag!(m, NoAutoNames), new.pro);
            new.langs = val(flag!(m, AutoLangs), flag!(m, NoAutoLangs), new.pro);
            new.charsets = val(flag!(m, AutoCharsets), flag!(m, NoAutoCharsets), new.pro);

            return new;

            fn val(arg: bool, no_arg: bool, pro: bool) -> bool {
                if arg {
                    true
                } else if no_arg {
                    false
                } else {
                    !pro
                }
            }
        }

        fn retiming(m: &mut ArgMatches) -> RetimingOptions {
            RetimingOptions {
                rm_segments: rm!(m, RmSegments, GlobSetPattern),
                no_linked: flag!(m, NoLinked),
                less_retiming: flag!(m, LessRetiming),
            }
        }

        fn tool_paths(m: &mut ArgMatches) -> ToolPaths {
            ToolPaths {
                user_tools: flag!(m, UserTools),
                ..Default::default()
            }
        }
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        if let Some(l) = get_locale(m) {
            self.locale = l;
        }

        input(self, m);
        output(self, m);
        verbosity(self, m);

        upd_flag!(self.exit_on_err, m, ExitOnErr);
        upd_flag!(self.save_config, m, SaveConfig);
        upd_flag!(self.reencode, m, Reencode);
        upd!(self.threads, m, Threads, u8);

        auto_flags(self, m);
        upd_tracks_or_attachs!(self.audio_tracks, m, Audio, NoAudio, AudioTracks);
        upd_tracks_or_attachs!(self.sub_tracks, m, Subs, NoSubs, SubTracks);
        upd_tracks_or_attachs!(self.video_tracks, m, Video, NoVideo, VideoTracks);
        upd_chapters(&mut self.chapters, m);
        upd_tracks_or_attachs!(self.font_attachs, m, Fonts, NoFonts, FontAttachs);
        upd_tracks_or_attachs!(self.other_attachs, m, Attachs, NoAttachs, OtherAttachs);

        upd_track_flags!(self.default_track_flags, m, Defaults, MaxDefaults, DefaultTrackFlags);
        upd_track_flags!(self.forced_track_flags, m, Forceds, MaxForceds, ForcedTrackFlags);
        upd_track_flags!(self.enabled_track_flags, m, Enableds, MaxEnableds, EnabledTrackFlags);

        upd!(self.specials.0, m, Specials, Vec<String>, @opt);
        retiming(self, m);
        upd_flag!(self.tool_paths.user_tools, m, UserTools);

        let mut m: &mut ArgMatches = m;
        let mut _owned_m: Option<ArgMatches> = None;
        let mut cmd: Option<Command> = None;
        while let Some(mut t_args) = m.get_raw(MuxConfigArg::Target.undashed()) {
            // unwrap is safe: target require as min 1 argument.
            let t = t_args.next().unwrap();
            let t = match self.get_key(t) {
                Some(t) => t,
                None => Target::try_from(t)?,
            };

            if let Target::Group(TargetGroup::Global) = t {
                return self.try_update_from(t_args);
            }

            let matches = cmd
                .get_or_insert_with(|| MuxConfigTarget::command())
                .clone()
                .try_get_matches_from(t_args)?;
            _owned_m = Some(matches);
            m = _owned_m.as_mut().unwrap();

            if let Some(trg) = self.targets.as_mut().and_then(|map| map.get_mut(&t)) {
                trg.update_from_arg_matches_mut(m)?;
                continue;
            }

            let val = MuxConfigTarget::from_arg_matches_mut(m)?;
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

        fn input(cfg: &mut MuxConfig, m: &mut ArgMatches) {
            let input = &mut cfg.input;
            upd!(input.dir, m, Input, PathBuf);
            upd!(input.range, m, Range, RangeU64, @opt);
            upd!(input.skip, m, Skip, GlobSetPattern, @opt);
            upd!(input.depth, m, Depth, u8);

            upd_flag!(input.solo, m, Solo);

            input.need_num = input.range.is_some();
            input.out_need_num = false;

            if input.dirs.values().any(|v| !v.is_empty()) {
                input.dirs = Default::default();
            }
        }

        fn output(cfg: &mut MuxConfig, m: &mut ArgMatches) {
            if let Some(output) = rm!(m, Output, Output) {
                cfg.output = output;
                cfg.is_output_constructed_from_input = false;
            }
        }

        fn verbosity(cfg: &mut MuxConfig, m: &mut ArgMatches) {
            if flag!(m, Quiet) {
                cfg.verbosity = Verbosity::Quiet;
            } else if let Some(cnt) = rm!(m, Verbose, u8) {
                cfg.verbosity = Verbosity::from(cnt);
            }
        }

        fn auto_flags(cfg: &mut MuxConfig, m: &mut ArgMatches) {
            let auto = &mut cfg.auto_flags;
            upd_flag!(auto.pro, m, Pro);

            auto.track[TrackFlagType::Default] = val(
                flag!(m, AutoDefaults),
                flag!(m, NoAutoDefaults),
                auto.pro,
                auto.track[TrackFlagType::Default],
            );
            auto.track[TrackFlagType::Forced] = val(
                flag!(m, AutoForceds),
                flag!(m, NoAutoForceds),
                auto.pro,
                auto.track[TrackFlagType::Forced],
            );
            auto.track[TrackFlagType::Enabled] = val(
                flag!(m, AutoEnableds),
                flag!(m, NoAutoEnableds),
                auto.pro,
                auto.track[TrackFlagType::Default],
            );

            auto.names = val(
                flag!(m, AutoNames),
                flag!(m, NoAutoNames),
                auto.pro,
                auto.names,
            );
            auto.langs = val(
                flag!(m, AutoLangs),
                flag!(m, NoAutoLangs),
                auto.pro,
                auto.langs,
            );
            auto.charsets = val(
                flag!(m, AutoCharsets),
                flag!(m, NoAutoCharsets),
                auto.pro,
                auto.charsets,
            );

            return;

            fn val(arg: bool, no_arg: bool, pro: bool, old: bool) -> bool {
                if arg {
                    true
                } else if no_arg || pro {
                    false
                } else {
                    old
                }
            }
        }

        fn retiming(cfg: &mut MuxConfig, m: &mut ArgMatches) {
            upd!(cfg.retiming.rm_segments, m, RmSegments, GlobSetPattern, @opt);
            upd_flag!(cfg.retiming.no_linked, m, NoLinked);
            upd_flag!(cfg.retiming.less_retiming, m, LessRetiming);
        }
    }
}

pub(super) fn get_locale(m: &ArgMatches) -> Option<LangCode> {
    match m.get_one::<LangCode>(MuxConfigArg::Locale.undashed()) {
        Some(&l) => {
            Msg::upd_lang_or_warn(l);
            Some(l)
        }
        None => None,
    }
}

pub(super) fn printable_args(m: &ArgMatches) -> Result<(), Error> {
    arg(m, MuxConfigArg::ListTargets, Target::print_list_targets)?;
    arg(m, MuxConfigArg::ListContainers, || {
        println!("{}", Msg::ListContainers)
    })?;
    arg(m, MuxConfigArg::ListLangs, LangCode::print_list_langs)?;

    arg(m, MuxConfigArg::Version, || {
        let v = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));
        println!("{}", v);
    })?;
    arg(m, MuxConfigArg::Help, || {
        let mut cmd = MuxConfig::command();
        if let Err(_) = cmd.print_help() {
            println!("{}", cmd.render_help());
        }
    })?;

    return Ok(());

    fn arg<F>(m: &ArgMatches, arg: MuxConfigArg, print: F) -> Result<(), Error>
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

        let user_tools = m.get_flag(MuxConfigArg::UserTools.undashed());
        let paths = ToolPaths {
            user_tools,
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
    let range = rm!(m, Range, RangeU64);

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

fn trg_upd_chapters(chp: &mut Option<Chapters>, m: &mut ArgMatches) {
    match chp.as_mut() {
        Some(chp) => upd_chapters(chp, m),
        None => *chp = get_chapters(m),
    }
}

macro_rules! trg_upd_tracks_or_attachs {
    ($field:expr, $matches:ident, $arg:ident, $no_arg:ident, $ty:ident) => {
        match $field.as_mut() {
            Some(f) => upd_tracks_or_attachs!(*f, $matches, $arg, $no_arg, $ty),
            None => $field = get_tracks_or_attachs!($matches, $arg, $no_arg, $ty),
        }
    };
}

macro_rules! trg_upd_track_flags {
    ($field:expr, $matches:ident, $arg:ident, $lim_arg:ident, $ty:ident) => {
        match $field.as_mut() {
            Some(f) => upd_track_flags!(*f, $matches, $arg, $lim_arg, $ty),
            None => $field = get_track_flags!($matches, $arg, $lim_arg, $ty),
        }
    };
}

impl FromArgMatches for MuxConfigTarget {
    fn from_arg_matches(m: &ArgMatches) -> Result<Self, Error> {
        Self::from_arg_matches_mut(&mut m.clone())
    }

    fn update_from_arg_matches(&mut self, m: &ArgMatches) -> Result<(), Error> {
        self.update_from_arg_matches_mut(&mut m.clone())
    }

    fn from_arg_matches_mut(m: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            audio_tracks: get_tracks_or_attachs!(m, Audio, NoAudio, AudioTracks),
            sub_tracks: get_tracks_or_attachs!(m, Subs, NoSubs, SubTracks),
            video_tracks: get_tracks_or_attachs!(m, Video, NoVideo, VideoTracks),
            chapters: get_chapters(m),
            font_attachs: get_tracks_or_attachs!(m, Fonts, NoFonts, FontAttachs),
            other_attachs: get_tracks_or_attachs!(m, Attachs, NoAttachs, OtherAttachs),
            default_track_flags: get_track_flags!(m, Defaults, MaxDefaults, DefaultTrackFlags),
            forced_track_flags: get_track_flags!(m, Forceds, MaxForceds, ForcedTrackFlags),
            enabled_track_flags: get_track_flags!(m, Enableds, MaxEnableds, EnabledTrackFlags),
            track_names: rm!(m, Names, TrackNames),
            track_langs: rm!(m, Langs, TrackLangs),
            specials: rm!(m, Specials, Specials),
        })
    }

    fn update_from_arg_matches_mut(&mut self, m: &mut ArgMatches) -> Result<(), Error> {
        trg_upd_tracks_or_attachs!(self.audio_tracks, m, Audio, NoAudio, AudioTracks);
        trg_upd_tracks_or_attachs!(self.sub_tracks, m, Subs, NoSubs, SubTracks);
        trg_upd_tracks_or_attachs!(self.video_tracks, m, Video, NoVideo, VideoTracks);

        trg_upd_chapters(&mut self.chapters, m);
        trg_upd_tracks_or_attachs!(self.font_attachs, m, Fonts, NoFonts, FontAttachs);
        trg_upd_tracks_or_attachs!(self.other_attachs, m, Attachs, NoAttachs, OtherAttachs);

        trg_upd_track_flags!(self.default_track_flags, m, Defaults, MaxDefaults, DefaultTrackFlags);
        trg_upd_track_flags!(self.forced_track_flags, m, Forceds, MaxForceds, ForcedTrackFlags);
        trg_upd_track_flags!(self.enabled_track_flags, m, Enableds, MaxEnableds, EnabledTrackFlags);

        upd!(self.track_names, m, Names, TrackNames, @opt);
        upd!(self.track_langs, m, Langs, TrackLangs, @opt);
        upd!(self.specials, m, Specials, Specials, @opt);

        Ok(())
    }
}
