#[macro_export]
macro_rules! from_arg_matches {
    // Case 1: Return Option<T>
    ($matches:ident, $typ:ty, $arg:ident, @no_default) => {{
        use $crate::CLIArg;

        $matches
            .try_remove_one::<$typ>(<$crate::MuxConfig as $crate::CLIArgs>::Arg::$arg.as_long())
            .map_err($crate::MuxError::from)?
    }};

    // Case 2: Default function returns plain value
    ($matches:ident, $typ:ty, $arg:ident, $default_fn:expr) => {
        match $crate::from_arg_matches!($matches, $typ, $arg, @no_default)
        {
            Some(val) => val,
            None => $default_fn(),
        }
    };

    // Case 3: Default function returns Result
    ($matches:ident, $typ:ty, $arg:ident, $default_fn:expr, @try_default) => {
        match $crate::from_arg_matches!($matches, $typ, $arg, @no_default)
        {
            Some(val) => val,
            None => $default_fn().map_err($crate::MuxError::from)?,
        }
    };

    // Case 4: Off on pro flag logic
    ($matches:ident, $arg:ident, $no_arg:ident, $pro:expr, @off_on_pro) => {
        match $crate::from_arg_matches!($matches, bool, $arg, @no_default)
        {
            Some(true) => true,
            _ => {
                match $crate::from_arg_matches!($matches, bool, $no_arg, @no_default)
                {
                    Some(true) => false,
                    _ => !$pro,
                }
            }
        }
    };

    // Case 5: Targets opt logic in MuxConfig.targets
    ($matches:ident, $type:ident, @target) => {{
        use $crate::IsDefault;

        let value = $type::from_arg_matches_mut($matches)?;
        if value.is_default() {
            None
        } else {
            Some(value)
        }
    }};

    (@fn) => {
        fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
            let mut matches = matches.clone();
            Self::from_arg_matches_mut(&mut matches)
        }
    };

    (@fn_update) => {
        fn update_from_arg_matches(
            &mut self,
            matches: &clap::ArgMatches,
        ) -> Result<(), clap::Error> {
            let mut matches = matches.clone();
            self.update_from_arg_matches_mut(&mut matches)
        }
    };

    (@fn_update_mut) => {
        fn update_from_arg_matches_mut(
            &mut self,
            matches: &mut clap::ArgMatches,
        ) -> Result<(), clap::Error> {
            *self = Self::from_arg_matches_mut(matches)?;
            Ok(())
        }
    };

    (@unrealized_fns) => {
        $crate::from_arg_matches!(@fn);
        $crate::from_arg_matches!(@fn_update);
        $crate::from_arg_matches!(@fn_update_mut);
    };

    (@fn_mut, $arg:ident, $no_arg:ident) => {
        fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
            if $crate::from_arg_matches!(matches, bool, $no_arg, || false) {
                Ok(Self::default().no_flag(true))
            } else {
                Ok($crate::from_arg_matches!(matches, Self, $arg, Self::default))
            }
        }
    };

    (@impl, $ty:ty, $arg:ident, $no_arg:ident) => {
        impl clap::FromArgMatches for $ty {
            $crate::from_arg_matches!(@fn_mut, $arg, $no_arg);
            $crate::from_arg_matches!(@unrealized_fns);
        }
    };

    // Update case 1: field is not Option
    (@upd, $self:ident, $matches:ident; $( $field:ident, $ty:ty, $arg:ident ),* ) => {{
        $(
            if let Some(val) = from_arg_matches!($matches, $ty, $arg, @no_default) {
                $self.$field = val;
            }
        )*
    }};

    // Update case 2: field is Option
    (@upd, $self:ident, $matches:ident, @opt_field; $( $field:ident, $ty:ty, $arg:ident ),* ) => {{
        $(
            if let Some(val) = from_arg_matches!($matches, $ty, $arg, @no_default) {
                $self.$field = Some(val);
            }
        )*
    }};
}
