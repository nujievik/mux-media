use super::{MuxConfig, MuxConfigTarget, RawMuxConfig, cli_args::MuxConfigArg};
use crate::{
    CLIArg, Msg, MuxError, Output, Target, Tool, Tools, TryFinalizeInit, TryInit, from_arg_matches,
};
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
        let raw = RawMuxConfig::try_parse(args_os().skip(1))?;
        let mut matches = Self::command().try_get_matches_from(raw.args)?;

        let mut cfg = match matches.try_get_one::<bool>(MuxConfigArg::Json.as_long())? {
            Some(true) => match matches.try_get_one::<PathBuf>(MuxConfigArg::Input.as_long())? {
                Some(dir) => Self::try_from_json(dir.join(Self::JSON_NAME)).ok(),
                _ => None,
            },
            _ => None,
        };

        if let Some(json) = from_arg_matches!(matches, PathBuf, Load, @no_default) {
            if let Some(cfg) = &mut cfg {
                cfg.try_upd_from_json(json)?;
            } else {
                cfg = Some(Self::try_from_json(json)?);
            }
        }

        let cfg = match cfg {
            Some(mut cfg) => {
                cfg.update_from_arg_matches_mut(&mut matches)?;
                cfg.try_upd_from_trg_args(raw.trg_args)?;
                cfg
            }
            None => {
                let mut cfg = Self::from_arg_matches_mut(&mut matches)?;
                cfg.try_upd_from_trg_args(raw.trg_args)?;
                cfg
            }
        };

        Ok(cfg)
    }
}

impl TryFinalizeInit for MuxConfig {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        self.input.upd_out_need_num(self.output.need_num());

        if self.is_output_constructed_from_input
            && Some(self.input.get_dir()) != self.output.get_dir().parent()
        {
            self.output = Output::try_from(&self.input)?;
        }

        self.input.try_finalize_init()?;
        self.output.try_finalize_init()?;

        let temp_dir = self.output.get_temp_dir();

        #[cfg(not(all(windows, any(target_arch = "x86", target_arch = "x86_64"))))]
        {
            self.tools.try_upd_paths(Tool::iter_mkvtoolnix())?;
        }

        #[cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
        {
            match self.user_tools {
                true => self
                    .tools
                    .try_upd_paths(Tool::iter_mkvtoolnix())
                    .or_else(|e| {
                        self.tools
                            .try_upd_paths_from_bundled(Tool::iter_mkvtoolnix(), temp_dir)
                            .map_err(|_| e)
                    }),
                false => self
                    .tools
                    .try_upd_paths_from_bundled(Tool::iter_mkvtoolnix(), temp_dir)
                    .or_else(|_| self.tools.try_upd_paths(Tool::iter_mkvtoolnix())),
            }?
        }

        let json = Tools::make_json(temp_dir);
        self.tools.upd_json(json);

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
        let raw = RawMuxConfig::try_parse(args)?;
        Self::try_from(raw)
    }

    pub fn try_upd_from_args<I, T>(&mut self, args: I) -> Result<(), MuxError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let raw = RawMuxConfig::try_parse(args)?;
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
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(vec) => {
            println!("{} '{}'", Msg::ReadsJson, path.display());
            Ok(vec)
        }
        Err(e) => Err(e.into()),
    }
}
