pub(crate) mod arg;
pub(crate) mod logger;
mod parser;

use crate::{Config, Input, Msg, Result, undashed};
use clap::{ArgMatches, Command, CommandFactory, FromArgMatches, Parser};
use std::{
    env::args_os,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

impl Config {
    pub(crate) fn try_init() -> Result<Config> {
        let cmd = Config::command();
        let mut cli_matches = cli_matches(&cmd)?;
        let mut txt_matches = get_txt_matches(cmd, &cli_matches)?;

        let mut cfg = get_cfg_from_input_txt(&cli_matches, &txt_matches)?;

        if let Some(m) = txt_matches.as_mut() {
            upd_cfg(&mut cfg, m)?;
        }
        upd_cfg(&mut cfg, &mut cli_matches)?;

        // unwrap is safe: Option is Some.
        return Ok(cfg.unwrap());

        fn cli_matches(cmd: &Command) -> Result<ArgMatches> {
            let m = cmd.clone().try_get_matches_from(args_os().skip(1))?;
            let _ = parser::get_locale(&m);
            parser::printable_args(&m)?;
            Ok(m)
        }

        fn get_txt_matches(cmd: Command, cli_matches: &ArgMatches) -> Result<Option<ArgMatches>> {
            let m = match cli_matches.get_one::<PathBuf>(undashed!(Load)) {
                Some(j) => {
                    let args = try_read_txt_args(j)?;
                    let m = cmd.try_get_matches_from(args)?;
                    Some(m)
                }
                None => None,
            };
            Ok(m)
        }

        fn get_cfg_from_input_txt(
            cli_matches: &ArgMatches,
            txt_matches: &Option<ArgMatches>,
        ) -> Result<Option<Config>> {
            let input_txt = txt_matches
                .as_ref()
                .and_then(|m| {
                    m.get_one::<PathBuf>(undashed!(Input))
                        .map(|d| d.join(Config::CONFIG_NAME))
                })
                .or_else(|| {
                    cli_matches
                        .get_one::<PathBuf>(undashed!(Input))
                        .map(|d| d.join(Config::CONFIG_NAME))
                })
                .or_else(|| {
                    Input::try_default_dir()
                        .map(|d| d.join(Config::CONFIG_NAME))
                        .ok()
                });

            let opt = match input_txt.and_then(|j| try_read_txt_args(&j).ok()) {
                Some(args) => Some(Config::try_parse_from(args)?),
                None => None,
            };

            Ok(opt)
        }

        fn upd_cfg(cfg: &mut Option<Config>, m: &mut ArgMatches) -> Result<()> {
            match cfg.as_mut() {
                Some(cfg) => cfg.update_from_arg_matches_mut(m)?,
                None => *cfg = Some(Config::from_arg_matches_mut(m)?),
            };
            Ok(())
        }

        fn try_read_txt_args(txt: &Path) -> Result<Vec<String>> {
            let file = File::open(txt)?;
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().collect::<std::io::Result<Vec<_>>>()?;

            println!("{} '{}'...", Msg::LoadingTxtConfig, txt.display());
            Ok(lines)
        }
    }
}
