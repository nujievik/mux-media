use super::Output;
use crate::{ToJsonArgs, json_arg};
use std::path::MAIN_SEPARATOR;

macro_rules! push_or_return_new {
    ($out:ident, $( $as_os_str:expr ),*) => {{
        $(
            match $as_os_str.as_os_str().to_str() {
                Some(s) => $out.push_str(s),
                None => return Vec::new(),
            }
        )*
    }};
}

impl ToJsonArgs for Output {
    fn to_json_args(&self) -> Vec<String> {
        let mut out = String::new();

        push_or_return_new!(out, self.dir);
        out.push(MAIN_SEPARATOR);
        push_or_return_new!(out, self.name_begin, self.name_tail);

        if self.ext != Self::DEFAULT_EXT {
            out.push('.');
            push_or_return_new!(out, self.ext);
        }

        vec![json_arg!(Output), out]
    }
}
