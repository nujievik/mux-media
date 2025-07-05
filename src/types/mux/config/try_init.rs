use super::{MuxConfig, MuxConfigTarget, RawMuxConfig, cli_args::MuxConfigArg};
use crate::{CLIArg, MuxError, Target, TryFinalizeInit, TryInit};
use clap::{CommandFactory, FromArgMatches};
use std::{
    collections::HashMap,
    env::args_os,
    ffi::OsString,
    fs::File,
    io::BufReader,
    mem::take,
    path::{Path, PathBuf},
};

impl TryInit for MuxConfig {
    fn try_init() -> Result<Self, MuxError> {
        let raw = RawMuxConfig::try_from_args(args_os().skip(1))?;
        let mut matches = Self::command().try_get_matches_from(raw.args)?;

        let json = matches.try_remove_one::<PathBuf>(MuxConfigArg::Config.as_long())?;
        let no_json = *matches
            .try_get_one::<bool>(MuxConfigArg::NoConfig.as_long())?
            .unwrap_or(&false);

        let mut cfg = if no_json {
            None
        } else {
            matches
                .try_get_one::<PathBuf>(MuxConfigArg::Input.as_long())
                .ok()
                .flatten()
                .and_then(|dir| Self::try_from_json(dir.join(Self::JSON_NAME)).ok())
        };

        if let Some(json) = json {
            if let Some(cfg_val) = &mut cfg {
                cfg_val.try_upd_from_json(json)?;
            } else {
                cfg = Some(Self::try_from_json(json)?);
            }
        }

        match cfg {
            Some(mut cfg) => {
                cfg.update_from_arg_matches_mut(&mut matches)?;
                cfg.try_upd_from_trg_args(raw.trg_args)?;
                Ok(cfg)
            }
            None => {
                let mut cfg = Self::from_arg_matches_mut(&mut matches)?;
                cfg.try_upd_from_trg_args(raw.trg_args)?;
                Ok(cfg)
            }
        }
    }
}

impl TryFinalizeInit for MuxConfig {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        self.input.try_finalize_init()?;
        self.output.try_finalize_init()?;
        Ok(())
    }
}

impl MuxConfig {
    pub const JSON_NAME: &'static str = "mux-media.json";

    pub fn try_from_args<I, T>(args: I) -> Result<Self, MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let raw = RawMuxConfig::try_from_args(args)?;
        Self::try_from(raw)
    }

    pub fn try_upd_from_args<I, T>(&mut self, args: I) -> Result<(), MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let raw = RawMuxConfig::try_from_args(args)?;
        let mut matches = Self::command().try_get_matches_from(raw.args)?;

        self.update_from_arg_matches_mut(&mut matches)?;

        self.targets.as_mut().map_or(Ok(()), |targets| {
            targets
                .iter_mut()
                .try_for_each(|(_, cfg)| cfg.update_from_arg_matches_mut(&mut matches))
        })?;

        Ok(())
    }

    pub fn try_from_json(json: impl AsRef<Path>) -> Result<Self, MuxError> {
        let args = try_read_json_args(json.as_ref())?;
        Self::try_from_args(args)
    }

    pub fn try_upd_from_json(&mut self, json: impl AsRef<Path>) -> Result<(), MuxError> {
        let args = try_read_json_args(json.as_ref())?;
        self.try_upd_from_args(args)
    }

    fn try_upd_from_trg_args(
        &mut self,
        trg_args: Option<HashMap<Target, Vec<OsString>>>,
    ) -> Result<(), MuxError> {
        let trg_args = match trg_args {
            Some(trg_args) => trg_args,
            None => return Ok(()),
        };

        let cmd = MuxConfigTarget::command();
        let mut targets: HashMap<Target, MuxConfigTarget> =
            take(&mut self.targets).unwrap_or_else(|| HashMap::new());

        let mut try_insert =
            |cmd: clap::Command, trg: Target, args: Vec<OsString>| -> Result<(), MuxError> {
                let mut matches = cmd.try_get_matches_from(args)?;

                if let Some(cfg) = targets.get_mut(&trg) {
                    cfg.update_from_arg_matches_mut(&mut matches)?;
                } else {
                    let cfg = MuxConfigTarget::from_arg_matches_mut(&mut matches)?;
                    targets.insert(trg, cfg);
                }

                Ok(())
            };

        let mut iter = trg_args.into_iter();
        let first = iter.next();

        while let Some((trg, args)) = iter.next() {
            try_insert(cmd.clone(), trg, args)?;
        }

        if let Some((trg, args)) = first {
            try_insert(cmd, trg, args)?;
        }

        if !targets.is_empty() {
            self.targets = Some(targets);
        }

        Ok(())
    }
}

impl TryFrom<RawMuxConfig> for MuxConfig {
    type Error = MuxError;

    fn try_from(raw: RawMuxConfig) -> Result<Self, Self::Error> {
        let mut matches = Self::command().try_get_matches_from(raw.args)?;
        let mut cfg = Self::from_arg_matches_mut(&mut matches)?;
        cfg.try_upd_from_trg_args(raw.trg_args)?;
        Ok(cfg)
    }
}

fn try_read_json_args(path: &Path) -> Result<Vec<String>, MuxError> {
    let file = File::open(path).map_err(|e| MuxError::from(format!("Open error: {}", e)))?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| MuxError::from(format!("JSON parse error: {}", e)))
}
