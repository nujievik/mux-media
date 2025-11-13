use super::{MuxConfig, MuxConfigTarget};
use crate::{Result, Target, ToJsonArgs, types::helpers::try_write_args_to_json};

impl MuxConfig {
    /// Tries save mux config to JSON in the input directory.
    ///
    /// Does nothing if [`MuxConfig::save_config`] is `false`, returning Ok().
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
    ($self:ident, $args:ident; $( $field:ident ),*) => {{
        $(
            $self.$field.append_json_args($args);
        )*
    }};
}

impl ToJsonArgs for MuxConfig {
    fn append_json_args(&self, args: &mut Vec<String>) {
        args.push(to_json_args!(Locale));
        args.push(self.locale.to_string());

        append_args_from_fields!(self, args; input, output, verbosity);

        to_json_args!(@push_true, self, args; exit_on_err, ExitOnErr, save_config, SaveConfig);

        if self.threads != Self::THREADS_DEFAULT {
            args.push(to_json_args!(Threads));
            args.push(format!("{}", self.threads));
        }

        append_args_from_fields!(
            self, args;
            auto_flags,
            audio_tracks,
            sub_tracks,
            video_tracks,
            chapters,
            font_attachs,
            other_attachs,
            default_track_flags,
            forced_track_flags,
            track_names,
            track_langs
        );

        if let Some(targets) = &self.targets {
            for (target, cfg) in targets {
                let target = match target {
                    Target::Path(path) => match path.to_str() {
                        Some(path) => path.to_string(),
                        None => continue,
                    },
                    Target::Group(g) => g.to_string(),
                };

                let mut trg_args = cfg.to_json_args();

                if trg_args.is_empty() {
                    continue;
                }

                args.push(to_json_args!(Target));
                args.push(target);
                args.append(&mut trg_args);
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

impl ToJsonArgs for MuxConfigTarget {
    fn append_json_args(&self, args: &mut Vec<String>) {
        append_args_from_opt_fields!(
            self, args;
            audio_tracks,
            sub_tracks,
            video_tracks,
            chapters,
            font_attachs,
            other_attachs,
            default_track_flags,
            forced_track_flags,
            track_names,
            track_langs
        );
    }
}
