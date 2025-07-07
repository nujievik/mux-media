use super::{MuxConfig, MuxConfigTarget};
use crate::{
    Target, ToJsonArgs, json_arg, push_true_json_args, types::helpers::try_write_args_to_json,
};

impl MuxConfig {
    pub fn write_args_to_json_or_log(&self) {
        if self.no_json {
            return;
        }

        let args = self.to_json_args();

        if args.is_empty() {
            return;
        }

        let json = self.input.get_dir().join(Self::JSON_NAME);
        match try_write_args_to_json(args, &json) {
            Ok(_) => {}
            Err(e) => log::warn!("Fail save current config to json: {}", e),
        }
    }
}

macro_rules! append_args_from_fields {
    ($args:ident, $self:ident; $( $field:ident ),*) => {{
        $(
            $args.append(&mut $self.$field.to_json_args());
        )*
    }};
}

impl ToJsonArgs for MuxConfig {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        append_args_from_fields!(args, self; input, output);

        args.push(json_arg!(Locale));
        args.push(self.locale.to_string());

        append_args_from_fields!(args, self; verbosity);
        push_true_json_args!(args, self; no_json, NoConfig, exit_on_err, ExitOnErr);

        append_args_from_fields!(
            args, self;
            off_on_pro,
            retiming,
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

                args.push(json_arg!(Target));
                args.push(target);
                args.append(&mut trg_args);
            }
        }

        args
    }
}

macro_rules! append_args_from_opt_fields {
    ($args:ident, $self:ident; $( $field:ident ),*) => {{
        $(
            if let Some(val) = $self.$field.as_ref() {
                $args.append(&mut val.to_json_args());
            }
        )*
    }};
}

impl ToJsonArgs for MuxConfigTarget {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        append_args_from_opt_fields!(
            args, self;
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

        args
    }
}
