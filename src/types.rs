macro_rules! deref_singleton_tuple_struct {
    ($wrapper:ty, $inner:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($wrapper:ty, @default) => {
        impl Default for $wrapper {
            fn default() -> Self {
                Self(Default::default())
            }
        }
    };

    ($wrapper:ty, $inner:ty, @from_str) => {
        impl std::str::FromStr for $wrapper {
            type Err = $crate::MuxError;

            fn from_str(s: &str) -> $crate::Result<Self> {
                s.parse::<$inner>().map(Self).map_err(Into::into)
            }
        }
    };

    ($wrapper:ty, $inner:ty, @all) => {
        deref_singleton_tuple_struct!($wrapper, $inner);
        deref_singleton_tuple_struct!($wrapper, @default);
        deref_singleton_tuple_struct!($wrapper, $inner, @from_str);
    };
}

macro_rules! to_ffmpeg_args {
    (@names_or_langs, $ty:ty, $arg:ident, $auto:ident, $marker:ident) => {
        impl $crate::ToFfmpegArgs for $ty {
            fn try_append_ffmpeg_args(
                args: &mut Vec<std::ffi::OsString>,
                mi: &mut $crate::MediaInfo,
            ) -> $crate::Result<()> {
                use $crate::{markers::MICmnTrackOrder, undashed};

                let order = mi.try_take_cmn::<MICmnTrackOrder>()?;

                let auto = mi.auto_flags.$auto;
                let metadata = undashed!(Metadata);
                let mtd_marker = undashed!($arg);

                order.iter().enumerate().for_each(|(i, m)| {
                    let val =
                        unwrap_or_return!(mi.get_ti::<$crate::markers::$marker>(&m.media, m.track));

                    if *auto || val.is_user() {
                        args.push(format!("-{}:s:{}", metadata, i).into());
                        args.push(format!("{}={}", mtd_marker, val).into());
                    }
                });

                mi.set_cmn::<MICmnTrackOrder>(order);

                Ok(())
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

    (@collect_id_map, $self:ident) => {{
        let mut id_map = std::collections::BTreeSet::<String>::new();

        if let Some(id_vals) = $self.map_hashed.as_ref() {
            id_vals.iter().for_each(|(tid, val)| {
                id_map.insert(format!("{}:{}", tid, val));
            });
        }

        if let Some(id_vals) = $self.map_unhashed.as_ref() {
            id_vals.iter().for_each(|(tid, val)| {
                id_map.insert(format!("{}:{}", tid, val));
            });
        }

        id_map
    }};

    (@tracks_or_attachs, $ty:ty, $arg:ident, $no_arg:ident) => {
        impl $crate::ToJsonArgs for $ty {
            fn append_json_args(&self, args: &mut Vec<String>) {
                use $crate::IsDefault;

                if self.is_default() {
                    return;
                }

                if self.no_flag {
                    args.push(to_json_args!($no_arg));
                    return;
                }

                let mut s_ids = std::collections::BTreeSet::<String>::new();

                if let Some(ids) = &self.ids_hashed {
                    ids.iter().for_each(|id| {
                        s_ids.insert(id.to_string());
                    });
                }

                if let Some(ids) = &self.ids_unhashed {
                    ids.iter().for_each(|id| {
                        s_ids.insert(id.to_string());
                    });
                }

                if s_ids.is_empty() {
                    return;
                }

                let mut s_ids = s_ids.into_iter().collect::<Vec<String>>().join(",");

                if self.inverse {
                    s_ids.insert(0, '!');
                }

                args.push(to_json_args!($arg));
                args.push(s_ids);
            }
        }
    };

    (@names_or_langs, $ty:ty, $arg:ident) => {
        impl $crate::ToJsonArgs for $ty {
            fn append_json_args(&self, args: &mut Vec<String>) {
                use $crate::IsDefault;

                if self.is_default() {
                    return;
                }

                if let Some(val) = &self.unmapped {
                    args.push(to_json_args!($arg));
                    args.push(val.to_string());
                    return;
                }

                let id_map = to_json_args!(@collect_id_map, self);

                if id_map.is_empty() {
                    return;
                }

                let id_map = id_map.into_iter().collect::<Vec<_>>().join(",");

                args.push(to_json_args!($arg));
                args.push(id_map);
            }
        }
    };
}

macro_rules! unwrap_or_return {
    ($x:expr) => {
        match $x {
            Some(x) => x,
            None => {
                return;
            }
        }
    };
}

pub(crate) mod arc_path_buf;
pub(crate) mod attachs;
pub(crate) mod auto_flags;
pub(crate) mod chapters;
pub(crate) mod char_encoding;
pub(crate) mod cli_arg;
pub(crate) mod codec_id;
pub(crate) mod duration;
pub(crate) mod extensions;
pub(crate) mod ffmpeg_stream;
pub(crate) mod file_type;
pub(crate) mod globset_pattern;
mod helpers;
pub(crate) mod input;
pub(crate) mod lang_code;
pub(crate) mod media_info;
pub(crate) mod media_number;
pub(crate) mod mux_config;
pub(crate) mod mux_current;
pub(crate) mod mux_error;
pub(crate) mod mux_logger;
pub(crate) mod muxer;
pub(crate) mod output;
pub(crate) mod range;
pub(crate) mod retiming;
pub(crate) mod target;
pub(crate) mod tools;
pub(crate) mod track_flags;
pub(crate) mod track_langs;
pub(crate) mod track_names;
pub(crate) mod track_order;
pub(crate) mod tracks;
pub(crate) mod value;
pub(crate) mod verbosity;
