macro_rules! deref_singleton_tuple_struct {
    ($wrapper:ty, $inner:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($wrapper:ty, $inner:ty, @from_str) => {
        deref_singleton_tuple_struct!($wrapper, $inner);

        impl std::str::FromStr for $wrapper {
            type Err = $crate::MuxError;

            fn from_str(s: &str) -> $crate::Result<Self> {
                s.parse::<$inner>().map(Self).map_err(Into::into)
            }
        }
    };
}

macro_rules! to_json_args {
    ($arg:ident) => {
        $crate::dashed!($arg).to_owned()
    };

    (@push_true, $self:ident, $args:ident; $( $field:ident, $arg:ident ),*) => {{
        $(
            if $self.$field {
                $args.push(to_json_args!($arg));
            }
        )*
    }};

    (@get_values, $self:ident) => {{
        let mut map = std::collections::BTreeSet::<String>::new();

        if let Some(xs) = $self.idxs.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if let Some(xs) = $self.ranges.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if let Some(xs) = $self.langs.as_ref() {
            xs.iter().for_each(|(k, v)| {
                map.insert(format!("{}:{}", k, v));
            });
        }

        if map.is_empty() {
            $self.single_val.as_ref().map(|v| v.to_string())
        } else {
            Some(map.into_iter().collect::<Vec<_>>().join(","))
        }
    }};
}

macro_rules! some_or {
    ($x:expr, $or:expr) => {
        match $x {
            Some(x) => x,
            None => $or,
        }
    };
}

pub(crate) mod arc_path_buf;
pub(crate) mod auto_flags;
pub(crate) mod chapters;
pub(crate) mod char_encoding;
pub(crate) mod cli_arg;
pub(crate) mod codec_id;
pub mod config;
pub(crate) mod dispositions;
pub(crate) mod duration;
pub(crate) mod extension;
pub(crate) mod file_type;
pub(crate) mod globset_pattern;
pub(crate) mod helpers;
pub(crate) mod input;
pub(crate) mod lang;
pub(crate) mod log_level;
pub(crate) mod media_info;
pub(crate) mod media_number;
pub(crate) mod metadata;
pub(crate) mod mux_error;
pub(crate) mod mux_logger;
pub(crate) mod muxer;
pub(crate) mod output;
pub(crate) mod range;
pub(crate) mod retiming;
pub(crate) mod stream;
pub(crate) mod target;
pub(crate) mod value;
