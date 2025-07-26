#[doc(hidden)]
#[macro_export]
macro_rules! json_arg {
    ($arg:ident) => {{
        <$crate::MuxConfigArg as $crate::ParseableArg>::dashed($crate::MuxConfigArg::$arg)
            .to_owned()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! push_true_json_args {
    ($args:ident, $self:ident; $( $field:ident, $arg:ident ),*) => {{
        $(
            if $self.$field {
                $args.push($crate::json_arg!($arg));
            }
        )*
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_json_args {
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
            fn to_json_args(&self) -> Vec<String> {
                use $crate::IsDefault;

                if self.is_default() {
                    return Vec::new();
                }

                if self.no_flag {
                    return vec![$crate::json_arg!($no_arg)];
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
                    return Vec::new();
                }

                let mut s_ids = s_ids.into_iter().collect::<Vec<String>>().join(",");

                if self.inverse {
                    s_ids.insert(0, '!');
                }

                vec![$crate::json_arg!($arg), s_ids]
            }
        }
    };

    (@names_or_langs, $ty:ty, $arg:ident) => {
        impl $crate::ToJsonArgs for $ty {
            fn to_json_args(&self) -> Vec<String> {
                use $crate::IsDefault;

                if self.is_default() {
                    return Vec::new();
                }

                if let Some(val) = &self.unmapped {
                    return vec![$crate::json_arg!($arg), val.to_string()];
                }

                let id_map = $crate::to_json_args!(@collect_id_map, self);

                if id_map.is_empty() {
                    return Vec::new();
                }

                let id_map = id_map.into_iter().collect::<Vec<_>>().join(",");

                vec![$crate::json_arg!($arg), id_map]
            }
        }
    };
}
