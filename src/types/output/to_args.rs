use crate::{Output, ToJsonArgs};

macro_rules! push_or_return {
    ($out:ident, $( $as_os_str:expr ),*) => {{
        $(
            match $as_os_str.as_os_str().to_str() {
                Some(s) => $out.push_str(s),
                None => return,
            }
        )*
    }};
}

impl ToJsonArgs for Output {
    fn append_json_args(&self, args: &mut Vec<String>) {
        let mut out = String::new();

        push_or_return!(out, &self.dir);
        out.push(std::path::MAIN_SEPARATOR);

        push_or_return!(out, self.name_begin);
        out.push(',');
        push_or_return!(out, self.name_tail);

        out.push('.');
        push_or_return!(out, self.ext);

        args.push(to_json_args!(Output));
        args.push(out);
    }
}
