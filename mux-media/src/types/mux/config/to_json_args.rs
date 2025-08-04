use super::{MuxConfig, MuxConfigTarget};
use crate::{Target, ToJsonArgs, to_json_args, types::helpers::try_write_args_to_json};

impl MuxConfig {
    /// Attempts to write args to json in the input directory; logs a warning on failure.
    pub fn write_args_to_json_or_log(&self) {
        if !self.json {
            return;
        }

        let args = self.to_json_args();

        if args.is_empty() {
            return;
        }

        let json = self.input.dir().join(Self::JSON_NAME);

        if let Err(e) = try_write_args_to_json(args, &json) {
            log::warn!("Fail save current config to json: {}", e);
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

        to_json_args!(@push_true, self, args; json, Json, exit_on_err, ExitOnErr);

        append_args_from_fields!(
            self, args;
            auto_flags,
            //retiming,
            audio_tracks,
            sub_tracks,
            video_tracks,
            button_tracks,
            chapters,
            font_attachs,
            other_attachs,
            default_t_flags,
            forced_t_flags,
            enabled_t_flags,
            track_names,
            track_langs,
            specials
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
            button_tracks,
            chapters,
            font_attachs,
            other_attachs,
            default_t_flags,
            forced_t_flags,
            enabled_t_flags,
            track_names,
            track_langs,
            specials
        );
    }
}
