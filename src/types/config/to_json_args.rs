use super::{Config, ConfigTarget};
use crate::{Result, ToJsonArgs, types::helpers::try_write_args_to_json};

impl Config {
    /// Tries save mux config to JSON in the input directory.
    ///
    /// Does nothing if [`Config::save_config`] is `false`, returning Ok().
    ///
    /// # Errors
    ///
    /// Returns an error if write args to JSON fails.
    pub fn try_save_config(&self) -> Result<()> {
        if !self.save_config {
            return Ok(());
        }

        let args = self.to_json_args();
        if args.is_empty() {
            return Ok(());
        }

        let json = self.input.dir.join(Self::JSON_NAME);

        match try_write_args_to_json(args, &json) {
            Ok(_) => Ok(()),
            Err(e) => Err(err!("Fail save current config to json: {}", e)),
        }
    }

    /// Tries save mux config to JSON in the input directory, logging warning on fail.
    pub(crate) fn save_config_or_warn(&self) {
        if let Err(e) = self.try_save_config() {
            log::warn!("{}", e);
        }
    }
}

macro_rules! append_args_from_fields {
    ($self:ident, $args:ident; $( $field:ident ),* $(,)?) => {{
        $(
            $self.$field.append_json_args($args);
        )*
    }};
}

impl ToJsonArgs for Config {
    fn append_json_args(&self, args: &mut Vec<String>) {
        args.push(to_json_args!(Locale));
        args.push(self.locale.to_string());

        append_args_from_fields!(self, args; input, output, log_level);

        to_json_args!(@push_true, self, args; exit_on_err, ExitOnErr, save_config, SaveConfig);

        if self.threads != Self::THREADS_DEFAULT {
            args.push(to_json_args!(Threads));
            args.push(format!("{}", self.threads));
        }

        append_args_from_fields!(
            self, args;
            auto_flags,
            streams,
            chapters,
            defaults,
            forceds,
            names,
            langs,
        );

        if let Some(targets) = &self.targets {
            for (t, t_cfg) in targets {
                let t = match t.to_str() {
                    Some(s) => s,
                    None => {
                        log::warn!(
                            "Fail save config for target '{}': unsupported UTF-8 symbol. Skipping",
                            t.as_path().display()
                        );
                        continue;
                    }
                };
                args.push(to_json_args!(Target));
                args.push(t.to_string());
                let len = args.len();
                t_cfg.append_json_args(args);

                // if nothing appended removes target.
                if args.len() == len {
                    let _ = args.drain(len - 2..len);
                }
            }
        }
    }
}

macro_rules! append_args_from_opt_fields {
    ($self:ident, $args:ident; $( $field:ident ),*) => {{
        $(
            if let Some(val) = $self.$field.as_ref() {
                val.append_json_args($args);
            }
        )*
    }};
}

impl ToJsonArgs for ConfigTarget {
    fn append_json_args(&self, args: &mut Vec<String>) {
        append_args_from_opt_fields!(
            self, args;
            streams,
            chapters,
            defaults,
            forceds,
            names,
            langs
        );
    }
}
