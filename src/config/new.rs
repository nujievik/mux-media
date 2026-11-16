mod command_factory;
mod finalize;
mod parse;
mod target;

use crate::{Config, Extension, LangCode, Msg, MuxError, Result, display};
use clap::{ArgMatches, Command, CommandFactory, FromArgMatches, Parser};
use parse::{printable_args, update_log_language};
use std::{
    env::args_os,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};
use target::Target;

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
            let _ = update_log_language(&m);
            printable_args(&m)?;
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
                    m.get_one::<InputPath>(undashed!(Input))
                        .map(|p| Config::txt_path(&p.path))
                })
                .or_else(|| {
                    cli_matches
                        .get_one::<InputPath>(undashed!(Input))
                        .map(|p| Config::txt_path(&p.path))
                })
                .unwrap_or_else(|| Config::txt_path("."));

            if let Some(args) = try_read_txt_args(&input_txt).ok() {
                let c = Config::try_parse_from(args)?;
                Ok(Some(c))
            } else {
                Ok(None)
            }
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

            println!("{} '{}'...", Msg::LoadingTxtConfig, display(txt));
            Ok(lines)
        }
    }
}

#[derive(Copy, Clone)]
struct LogLang(LangCode);

impl FromStr for LogLang {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<LogLang> {
        let code = s.parse::<LangCode>()?;
        if Msg::is_supported_lang(code) {
            Ok(Self(code))
        } else {
            Err(err!("unsupported log language"))
        }
    }
}

#[derive(Clone)]
pub(crate) struct InputPath {
    pub path: PathBuf,
    pub ext: Option<Extension>,
}
impl InputPath {
    pub fn parse(path: impl AsRef<Path>) -> crate::Result<InputPath> {
        let path = fs::canonicalize(path)?;

        let ext = if path.is_dir() {
            let _ = fs::read_dir(&path)?;
            None
        } else {
            let ext = match Extension::from_path(&path) {
                Some(ext) if ext.is_font() || ext.is_media() => ext,
                _ => return Err(err!(UnsupportedFileExtension)),
            };
            let _ = File::open(&path)?;
            Some(ext)
        };

        Ok(InputPath { path, ext })
    }
}
