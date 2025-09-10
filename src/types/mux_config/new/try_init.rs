use super::from_arg_matches::{get_locale, printable_args, tool_args};
use crate::{Input, Msg, MuxConfig, MuxConfigArg, ParseableArg, Result};
use clap::{ArgMatches, Command, CommandFactory, FromArgMatches, Parser};
use std::{
    env::args_os,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

impl MuxConfig {
    pub(crate) fn try_init() -> Result<MuxConfig> {
        let cmd = MuxConfig::command();
        let mut cli_matches = cli_matches(&cmd)?;
        let mut json_matches = get_json_matches(cmd, &cli_matches)?;

        let mut cfg = get_cfg_from_input_json(&cli_matches, &json_matches)?;

        if let Some(m) = json_matches.as_mut() {
            upd_cfg(&mut cfg, m)?;
        }
        upd_cfg(&mut cfg, &mut cli_matches)?;

        // unwrap is safe: Option is Some.
        return Ok(cfg.unwrap());

        fn cli_matches(cmd: &Command) -> Result<ArgMatches> {
            let m = cmd.clone().try_get_matches_from(args_os().skip(1))?;
            let _ = get_locale(&m);
            printable_args(&m)?;
            tool_args(&m)?;
            Ok(m)
        }

        fn get_json_matches(cmd: Command, cli_matches: &ArgMatches) -> Result<Option<ArgMatches>> {
            let m = match cli_matches.get_one::<PathBuf>(MuxConfigArg::Json.undashed()) {
                Some(j) => {
                    let args = try_read_json_args(j)?;
                    let m = cmd.try_get_matches_from(args)?;
                    Some(m)
                }
                None => None,
            };
            Ok(m)
        }

        fn get_cfg_from_input_json(
            cli_matches: &ArgMatches,
            json_matches: &Option<ArgMatches>,
        ) -> Result<Option<MuxConfig>> {
            let input_json = json_matches
                .as_ref()
                .and_then(|m| {
                    m.get_one::<PathBuf>(MuxConfigArg::Input.undashed())
                        .map(|d| d.join(MuxConfig::JSON_NAME))
                })
                .or_else(|| {
                    cli_matches
                        .get_one::<PathBuf>(MuxConfigArg::Input.undashed())
                        .map(|d| d.join(MuxConfig::JSON_NAME))
                })
                .or_else(|| {
                    Input::try_default_dir()
                        .map(|d| d.join(MuxConfig::JSON_NAME))
                        .ok()
                });

            let opt = match input_json.and_then(|j| try_read_json_args(&j).ok()) {
                Some(args) => Some(MuxConfig::try_parse_from(args)?),
                None => None,
            };

            Ok(opt)
        }

        fn upd_cfg(cfg: &mut Option<MuxConfig>, m: &mut ArgMatches) -> Result<()> {
            match cfg.as_mut() {
                Some(cfg) => cfg.update_from_arg_matches_mut(m)?,
                None => *cfg = Some(MuxConfig::from_arg_matches_mut(m)?),
            };
            Ok(())
        }

        fn try_read_json_args(json: &Path) -> Result<Vec<String>> {
            let file = File::open(json)?;
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(vec) => {
                    println!("{} '{}'...", Msg::LoadingJson, json.display());
                    Ok(vec)
                }
                Err(e) => Err(e.into()),
            }
        }
    }
}
