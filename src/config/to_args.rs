use super::{Config, ConfigTarget};
use crate::{Msg, Result, ToTxtConfig};
use std::{ffi::OsString, fs};

impl Config {
    /// Tries save config to .txt in the input directory.
    ///
    /// Does nothing if [`Config::save_config`] is `false`, returning Ok().
    ///
    /// # Errors
    ///
    /// Returns an error if write args to .txt fails.
    pub fn try_save_config(&self) -> Result<()> {
        if !self.save_config {
            return Ok(());
        }

        let txt = Config::txt_path(self.input.dir());

        if let Some(dir) = txt.parent() {
            if !dir.exists() {
                fs::create_dir(dir)?;
            }
        }

        self.write(txt)?;
        Ok(())
    }

    pub(crate) fn save_config_or_warn(&self) {
        if let Err(e) = self.try_save_config() {
            log::warn!("{}: {}", Msg::FailSaveConfig, e);
        }
    }
}

macro_rules! append_args_from_fields {
    ($self:ident, $args:ident; $( $field:ident ),* $(,)?) => {{
        $(
            $self.$field.append_args($args);
        )*
    }};
}

macro_rules! append_dispositions_to_args {
    ($args:ident; $( $disps:expr, $arg:ident, $max_arg:ident );* $(;)?) => {{
        $(
        if let Some(values) = to_args!(@get_values, $disps) {
            $args.push(to_args!($arg));
            $args.push(values.into());
        }
        if let Some(max) = $disps.max_in_auto {
            $args.push(to_args!($max_arg));
            $args.push(max.to_string().into());
        }
        )*
    }};
}

impl ToTxtConfig for Config {
    fn append_args(&self, args: &mut Vec<OsString>) {
        args.push(to_args!(Locale));
        args.push(self.locale.to_string().into());

        append_args_from_fields!(self, args; input, output, log_level);

        to_args!(@push_true, self, args; exit_on_err, ExitOnErr);

        if self.jobs != Self::JOBS_DEFAULT {
            args.push(to_args!(Jobs));
            args.push(format!("{}", self.jobs).into());
        }

        append_args_from_fields!(
            self, args;
            auto_flags,
            streams,
            chapters,
        );

        append_dispositions_to_args!(
            args;
            self.defaults, Defaults, MaxDefaults;
            self.forceds, Forceds, MaxForceds;
        );

        append_args_from_fields!(
            self, args;
            titles,
            langs,
            retiming,
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
                args.push(to_args!(Target));
                args.push(t.to_string().into());
                let len = args.len();
                t_cfg.append_args(args);

                // if nothing appended removes target.
                if args.len() == len {
                    let _ = args.drain(len - 2..len);
                }
            }
        }
    }
}

macro_rules! append_args_from_opt_fields {
    ($self:ident, $args:ident; $( $field:ident ),* $(,)?) => {{
        $(
            if let Some(val) = $self.$field.as_ref() {
                val.append_args($args);
            }
        )*
    }};
}

impl ToTxtConfig for ConfigTarget {
    fn append_args(&self, args: &mut Vec<OsString>) {
        append_args_from_opt_fields!(
            self, args;
            streams,
            chapters,
        );

        if let Some(v) = self.defaults.as_ref() {
            append_dispositions_to_args!(args; v, Defaults, MaxDefaults);
        }
        if let Some(v) = self.forceds.as_ref() {
            append_dispositions_to_args!(args; v, Forceds, MaxForceds);
        }

        append_args_from_opt_fields!(
            self, args;
            titles,
            langs,
        );
    }
}
