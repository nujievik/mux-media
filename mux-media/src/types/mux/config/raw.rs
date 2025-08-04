use super::RawMuxConfig;
use crate::{LangCode, Msg, MuxError, Target, Tool, types::helpers::os_str_tail};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
};

impl RawMuxConfig {
    pub(super) fn try_parse<I, T>(args: I) -> Result<Self, MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString>,
    {
        let raw = Self::try_from_args(args)?;

        if let Some(lang) = raw.locale {
            Msg::upd_lang_or_warn(lang);
        }

        if raw.list_targets {
            Target::print_list_targets();
            return Err(MuxError::new_ok());
        }

        if raw.list_containers {
            println!("{}", Msg::ListContainers);
            return Err(MuxError::new_ok());
        }

        if raw.list_langs {
            LangCode::print_list_langs();
            return Err(MuxError::new_ok());
        }

        if let Some((tool, args)) = raw.run_command {
            return Err(run_command_to_err(raw.args, tool, args)?);
        }

        Ok(raw)
    }

    pub fn try_from_args<I, T>(input_args: I) -> Result<Self, MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString>,
    {
        let mut locale: Option<LangCode> = None;
        let mut list_targets = false;
        let mut list_containers = false;
        let mut list_langs = false;

        let mut run_command: Option<(Tool, Vec<OsString>)> = None;
        let mut args: Vec<OsString> = Vec::new();
        let mut trg_args: Option<HashMap<Target, Vec<OsString>>> = None;
        let mut target: Option<Target> = None;

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

            if arg == "--list-targets" {
                list_targets = true;
                continue;
            }

            if arg == "--list-containers" {
                list_containers = true;
                continue;
            }

            if arg == "--list-langs" || arg == "--list-languages" {
                list_langs = true;
                continue;
            }

            if let Ok(maybe_tool) = os_str_tail(OsStr::new("--"), arg.as_ref()) {
                if let Some(tool) = Tool::iter().find(|tool| maybe_tool == tool.as_ref()) {
                    let remaining_args: Vec<OsString> = iter.collect();
                    run_command = Some((tool, remaining_args));
                    break;
                }
            }

            if arg == "--target" {
                if let Some(trg_arg) = iter.next() {
                    if trg_arg == "global" || trg_arg == "g" {
                        target = None;
                    } else {
                        target = Some(Target::try_from(&trg_arg)?);
                    }
                }
                continue;
            }

            match &target {
                Some(target) => {
                    let map = trg_args.get_or_insert_with(HashMap::new);
                    if let Some(vec) = map.get_mut(target) {
                        vec.push(arg);
                    } else {
                        map.insert(target.clone(), vec![arg]);
                    }
                }
                None => args.push(arg),
            }
        }

        Ok(Self {
            locale,
            list_targets,
            list_containers,
            list_langs,
            run_command,
            args,
            trg_args,
        })
    }
}

#[cfg(not(all(
    feature = "with_embedded_bins",
    windows,
    any(target_arch = "x86", target_arch = "x86_64")
)))]
#[inline(always)]
fn run_command_to_err(
    _: Vec<OsString>,
    tool: Tool,
    run_args: Vec<OsString>,
) -> Result<MuxError, MuxError> {
    let tools = crate::Tools::try_from_tools([tool])?;
    let out = tools.run(tool, run_args)?;
    return Err(out.into());
}

#[cfg(all(
    feature = "with_embedded_bins",
    windows,
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[inline(always)]
fn run_command_to_err(
    cfg_args: Vec<OsString>,
    tool: Tool,
    run_args: Vec<OsString>,
) -> Result<MuxError, MuxError> {
    use crate::{
        MuxConfig, TryFinalizeInit,
        markers::{MCOutput, MCTools},
    };

    let mut mc = MuxConfig::try_from_args(cfg_args)?;
    mc.try_finalize_init()?;
    let result = mc.field::<MCTools>().run(tool, run_args);
    mc.field::<MCOutput>().remove_created_dirs();

    return Err(result?.into());
}
