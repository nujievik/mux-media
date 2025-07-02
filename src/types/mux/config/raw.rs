use super::RawMuxConfig;
use crate::{
    LangCode, Msg, MuxError, Target, TargetGroup, Tool, Tools, TryInit, os_helpers::os_str_tail,
    types::mux::logger::get_stderr_color_prefix,
};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

impl TryInit for RawMuxConfig {
    fn try_init() -> Result<Self, MuxError> {
        let args: Vec<OsString> = std::env::args_os().skip(1).collect();
        let cfg = Self::try_from(args)?;

        if let Some(lang) = cfg.locale {
            Msg::try_upd_lang(lang).unwrap_or_else(|e| {
                eprintln!(
                    "{}{}: {}. {} '{}'",
                    get_stderr_color_prefix(log::Level::Warn),
                    Msg::ErrUpdLangCode,
                    e,
                    Msg::Using,
                    Msg::get_lang_code()
                );
            });
        }

        if cfg.list_langs {
            LangCode::print_list_langs();
            return Err(MuxError::new_ok());
        }

        if cfg.list_targets {
            Target::print_list_targets();
            return Err(MuxError::new_ok());
        }

        if let Some((tool, args)) = cfg.call_tool {
            let tools = Tools::try_from([tool])?;
            let msg = tools.run(tool, args, None)?;
            return Err(MuxError::new_ok().message(msg));
        }

        Ok(cfg)
    }
}

impl TryFrom<Vec<OsString>> for RawMuxConfig {
    type Error = MuxError;

    fn try_from(args: Vec<OsString>) -> Result<Self, Self::Error> {
        Self::try_from_args(args)
    }
}

impl RawMuxConfig {
    fn parse_target(arg: &OsString) -> Result<Target, MuxError> {
        let s = arg.to_string_lossy();
        let target = if let Ok(group) = s.parse::<TargetGroup>() {
            Target::Group(group)
        } else {
            let path = PathBuf::from(arg.clone())
                .canonicalize()
                .map_err(|e| MuxError::from(format!("Incorrect path target '{}': {}", s, e)));
            Target::Path(path?)
        };

        Ok(target)
    }

    #[inline]
    fn try_from_args(in_args: Vec<OsString>) -> Result<Self, MuxError> {
        let mut locale: Option<LangCode> = None;
        let mut list_langs = false;
        let mut list_targets = false;

        let mut call_tool: Option<(Tool, Vec<OsString>)> = None;
        let mut args: Vec<OsString> = Vec::new();
        let mut trg_args: Option<HashMap<Target, Vec<OsString>>> = None;
        let mut current_target: Option<Target> = None;

        let mut iter = in_args.into_iter();

        while let Some(arg) = iter.next() {
            if arg == "--locale" {
                let lang = iter
                    .next()
                    .ok_or_else(|| {
                        "a value is required for '--locale <lng>' but none was supplied".into()
                    })
                    .and_then(|arg| arg.to_string_lossy().parse::<LangCode>())?;
                locale = Some(lang);
                continue;
            }

            if arg == "--list-langs" || arg == "--list-languages" {
                list_langs = true;
                break;
            }

            if arg == "--list-targets" {
                list_targets = true;
                break;
            }

            if let Ok(maybe_tool) = os_str_tail(OsStr::new("--"), arg.as_ref()) {
                if let Some(tool) = Tool::iter().find(|tool| maybe_tool == tool.as_ref()) {
                    let remaining_args: Vec<OsString> = iter.collect();
                    call_tool = Some((tool, remaining_args));
                    break;
                }
            }

            match arg {
                arg if arg == OsStr::new("--target") => {
                    if let Some(trg_arg) = iter.next() {
                        let trg_str = trg_arg.to_string_lossy();
                        if trg_str == "global" || trg_str == "g" {
                            current_target = None;
                        } else {
                            let target = Self::parse_target(&trg_arg)?;
                            current_target = Some(target.clone());

                            trg_args
                                .get_or_insert_with(HashMap::new)
                                .entry(target)
                                .or_insert_with(Vec::new);
                        }
                    }
                }
                _ => {
                    if let Some(target) = &current_target {
                        if let Some(map) = trg_args.as_mut() {
                            map.get_mut(target).unwrap().push(arg);
                        }
                    } else {
                        args.push(arg);
                    }
                }
            }
        }

        Ok(Self {
            locale,
            list_langs,
            list_targets,
            call_tool,
            args,
            trg_args,
        })
    }
}
