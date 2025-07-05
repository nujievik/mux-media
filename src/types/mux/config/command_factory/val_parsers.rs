use crate::{Chapters, Input, Output, types::helpers::try_canonicalize_and_open};
use clap::{
    Arg, Command, Error,
    builder::TypedValueParser,
    error::{ContextKind, ContextValue, ErrorKind},
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{ffi::OsStr, path::PathBuf};

macro_rules! typed_value_parser {
    ($parser:ident, $val_ty:ty, $try_from_os_str:path) => {
        #[derive(Clone)]
        pub(super) struct $parser;

        impl TypedValueParser for $parser {
            type Value = $val_ty;

            fn parse_ref(
                &self,
                cmd: &Command,
                arg: Option<&Arg>,
                value: &OsStr,
            ) -> Result<Self::Value, Error> {
                $try_from_os_str(value).map_err(|e| {
                    let mut err = Error::new(ErrorKind::InvalidValue).with_cmd(&cmd);

                    if let Some(arg) = arg {
                        err.insert(
                            ContextKind::InvalidArg,
                            ContextValue::String(arg.to_string()),
                        );
                    }
                    err.insert(
                        ContextKind::InvalidValue,
                        ContextValue::String(format!(
                            "'{}' (reason: {})",
                            value.to_string_lossy(),
                            e
                        )),
                    );

                    err
                })
            }
        }
    };
}

typed_value_parser!(InputDirParser, PathBuf, Input::normalize_dir);
typed_value_parser!(OutputParser, Output, Output::try_from_path);
typed_value_parser!(ConfigParser, PathBuf, try_canonicalize_and_open);
typed_value_parser!(ChaptersParser, Chapters, Chapters::try_from_path);

pub(super) fn patterns_parser(s: &str) -> Result<GlobSet, String> {
    let mut builder = GlobSetBuilder::new();

    for pattern in s.split(',') {
        let glob =
            Glob::new(pattern).map_err(|e| format!("Invalid pattern '{}': {}", pattern, e))?;
        builder.add(glob);
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build patterns: {}", e))
}
