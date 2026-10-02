use super::{Config, ConfigTarget};
use crate::{Msg, Result, ToArgs};
use core::fmt::NumBuffer;
use std::{fs, io::Write};

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

        self.write_to_file(&txt)?;
        Ok(())
    }

    pub(crate) fn save_config_or_warn(&self) {
        if let Err(e) = self.try_save_config() {
            log::warn!("{}: {}", Msg::FailSaveConfig, e);
        }
    }
}

macro_rules! write_fields {
    ($self:ident, $w:ident, $buf:ident; $( $field:ident ),* $(,)?) => {{
        $(
            $self.$field.write_with_num_buffer($w, $buf)?;
        )*
    }};
}

macro_rules! write_dispositions {
    ($w:ident, $buf:ident; $( $values:expr, $arg:ident, $max_arg:ident );* $(;)?) => {{
    $(
        if $values.single_val.is_some() || $values.idxs.is_some() || $values.ranges.is_some() || $values.langs.is_some() {
            to_args!($w, $arg)?;
            to_args!($w, $values, @write_map, $buf);
        }

        if let Some(max) = $values.max_in_auto {
            to_args!($w, $max_arg)?;
            to_args!($w, max.format_into($buf).as_bytes(), @v)?;
        }
    )*
    }};
}

impl ToArgs for Config {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        to_args!(w, Locale)?;
        to_args!(w, self.locale.as_ref().as_bytes(), @v)?;

        write_fields!(self, w, buf; input, output, log_level);

        if self.overwrite {
            to_args!(w, Overwrite)?;
        }
        if self.exit_on_err {
            to_args!(w, ExitOnErr)?;
        }

        if self.jobs != Self::JOBS_DEFAULT {
            to_args!(w, Jobs)?;
            to_args!(w, (self.jobs as usize).format_into(buf).as_bytes(), @v)?;
        }

        write_fields!(self, w, buf; auto_flags, streams, chapters);

        write_dispositions!(
            w, buf;
            self.defaults, Defaults, MaxDefaults;
            self.forceds, Forceds, MaxForceds;
        );

        write_fields!(self, w, buf; titles, langs, subs_encoding, retiming);

        for (t, t_cfg) in &self.target_configs {
            if let Some(s) = t.to_str() {
                to_args!(w, Target)?;
                to_args!(w, s.as_bytes(), @v)?;
            } else {
                return Err(err!("invalid utf-8"));
            }

            t_cfg.write_with_num_buffer(w, buf)?;
        }

        Ok(())
    }
}

macro_rules! write_opt_fields {
    ($self:ident, $w:ident, $buf:ident; $( $field:ident ),* $(,)?) => {{
        $(
            if let Some(val) = $self.$field.as_ref() {
                val.write_with_num_buffer($w, $buf)?;
            }
        )*
    }};
}

impl ToArgs for ConfigTarget {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        write_opt_fields!(self, w, buf; streams, chapters);

        if let Some(v) = self.defaults.as_ref() {
            write_dispositions!(w, buf; v, Defaults, MaxDefaults);
        }
        if let Some(v) = self.forceds.as_ref() {
            write_dispositions!(w, buf; v, Forceds, MaxForceds);
        }

        write_opt_fields!(self, w, buf; titles, langs, subs_encoding);

        Ok(())
    }
}
