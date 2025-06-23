use super::{MuxConfig, MuxConfigTarget, RawMuxConfig};
use crate::{MuxError, Target, TryFinalizeInit, TryInit};
use clap::{CommandFactory, FromArgMatches};
use std::collections::HashMap;
use std::ffi::OsString;

impl TryInit for MuxConfig {
    fn try_init() -> Result<Self, MuxError> {
        let raw = RawMuxConfig::try_init()?;
        let mut cfg = Self::try_from(raw)?;
        cfg.try_finalize_init()?;
        Ok(cfg)
    }
}

impl TryFinalizeInit for MuxConfig {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        self.input.try_finalize_init()?;
        self.output.try_finalize_init()?;
        Ok(())
    }
}

impl TryFrom<RawMuxConfig> for MuxConfig {
    type Error = MuxError;

    fn try_from(raw: RawMuxConfig) -> Result<Self, Self::Error> {
        let mut matches = Self::command().try_get_matches_from(raw.args)?;
        let mut cfg = Self::from_arg_matches_mut(&mut matches)?;

        if let Some(trg_args) = raw.trg_args {
            cfg.try_set_targets(trg_args)?;
        }

        Ok(cfg)
    }
}

impl MuxConfig {
    fn try_set_targets(
        &mut self,
        trg_args: HashMap<Target, Vec<OsString>>,
    ) -> Result<(), MuxError> {
        let cmd = MuxConfigTarget::command();
        let len_trg_args = trg_args.len();
        let mut targets: HashMap<Target, MuxConfigTarget> = HashMap::with_capacity(len_trg_args);

        let mut count = 1;
        for (trg, args) in trg_args {
            if count == len_trg_args {
                let mut matches = cmd.try_get_matches_from(args)?;
                let cfg = MuxConfigTarget::from_arg_matches_mut(&mut matches)?;
                targets.insert(trg, cfg);
                break; // break because is last
            } else {
                // Diff is cmd.clone() and not break
                let mut matches = cmd.clone().try_get_matches_from(args)?;
                let cfg = MuxConfigTarget::from_arg_matches_mut(&mut matches)?;
                targets.insert(trg, cfg);
            }
            count += 1;
        }

        if !targets.is_empty() {
            self.targets = Some(targets);
        }

        Ok(())
    }
}
