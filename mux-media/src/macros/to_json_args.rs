#[doc(hidden)]
#[macro_export]
macro_rules! to_json_args {
    ($arg:ident) => {{
        use $crate::{MuxConfigArg, ParseableArg};
        MuxConfigArg::$arg.dashed().to_owned()
    }};

    (@push_true, $self:ident, $args:ident; $( $field:ident, $arg:ident ),*) => {{
        $(
            if $self.$field {
                $args.push($crate::to_json_args!($arg));
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
                    args.push($crate::to_json_args!($no_arg));
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

                args.push($crate::to_json_args!($arg));
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
                    args.push($crate::to_json_args!($arg));
                    args.push(val.to_string());
                    return;
                }

                let id_map = $crate::to_json_args!(@collect_id_map, self);

                if id_map.is_empty() {
                    return;
                }

                let id_map = id_map.into_iter().collect::<Vec<_>>().join(",");

                args.push($crate::to_json_args!($arg));
                args.push(id_map);
            }
        }
    };
}
