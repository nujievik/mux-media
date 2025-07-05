use super::Output;
use crate::ToJsonArgs;

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
        if !self.need_num() && self.ext == Self::DEFAULT_EXT {
            return Vec::new();
        }

        let mut out = String::new();
        push_or_return_new!(out, self.dir, self.name_begin, self.name_tail, self.ext);

        vec!["-o".to_string(), out]
    }
}
