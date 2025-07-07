use super::RawMuxConfig;
use crate::{
    LangCode, Msg, MuxError, Target, TargetGroup, Tool, Tools, types::helpers::os_str_tail,
};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

impl RawMuxConfig {
    pub(super) fn try_from_args<I, T>(args: I) -> Result<Self, MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let raw = Self::parse_args(args)?;

        if let Some(lang) = raw.locale {
            Msg::upd_lang_or_warn(lang);
        }

        if raw.list_langs {
            LangCode::print_list_langs();
            return Err(MuxError::new_ok());
        }

        if raw.list_targets {
            Target::print_list_targets();
            return Err(MuxError::new_ok());
        }

        if let Some((tool, args)) = raw.call_tool {
            let tools = Tools::try_from([tool])?;
            let msg = tools.run(tool, args, None)?;
            return Err(MuxError::new_ok().message(msg));
        }

        Ok(raw)
    }
}

impl RawMuxConfig {
    #[inline]
    pub fn parse_args<I, T>(input_args: I) -> Result<Self, MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut locale: Option<LangCode> = None;
        let mut list_langs = false;
        let mut list_targets = false;

        let mut call_tool: Option<(Tool, Vec<OsString>)> = None;
        let mut args: Vec<OsString> = Vec::new();
        let mut trg_args: Option<HashMap<Target, Vec<OsString>>> = None;
        let mut current_target: Option<Target> = None;

        let mut iter = input_args.into_iter().map(|arg| arg.into());

        while let Some(arg) = iter.next() {
            if arg == "--locale" {
                let next_arg = iter.next().ok_or_else(|| {
                    MuxError::from("a value is required for '--locale <lng>' but none was supplied")
                })?;
                let lang = next_arg.to_string_lossy().parse::<LangCode>()?;

                locale = Some(lang);
                args.push(arg);
                args.push(next_arg);
                continue;
            }

            if arg == "--list-langs" || arg == "--list-languages" {
                list_langs = true;
                continue;
            }

            if arg == "--list-targets" {
                list_targets = true;
                continue;
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

    #[inline(always)]
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
}
