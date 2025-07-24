use super::Output;
use crate::{ToJsonArgs, json_arg};

macro_rules! push_or_return_vec_new {
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

        push_or_return_vec_new!(out, &self.dir);

        push_or_return_vec_new!(out, self.name_begin);
        out.push(',');
        push_or_return_vec_new!(out, self.name_tail);

        out.push('.');
        push_or_return_vec_new!(out, self.ext);

        vec![json_arg!(Output), out]
    }
}
