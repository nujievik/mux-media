use super::Streams;
use crate::{IsDefault, ToTxtConfig};
use std::ffi::OsString;

impl ToTxtConfig for Streams {
    fn append_args(&self, args: &mut Vec<OsString>) {
        if self.is_default() {
            return;
        }

        if self.no_flag {
            args.push(to_args!(NoStreams));
            return;
        }

        let arg = match arg(self) {
            s if s.is_empty() => return,
            s => s,
        };

        args.push(to_args!(Streams));
        args.push(arg.into());
    }
}

fn arg(streams: &Streams) -> String {
    let mut s = std::collections::BTreeSet::<String>::new();

    if let Some(xs) = &streams.idxs {
        xs.iter().for_each(|x| {
            s.insert(x.to_string());
        });
    }

    if let Some(xs) = &streams.langs {
        xs.iter().for_each(|x| {
            s.insert(x.to_string());
        });
    }

    if let Some(xs) = &streams.ranges {
        xs.iter().for_each(|x| {
            s.insert(x.to_string());
        });
    }

    if s.is_empty() {
        return String::new();
    }

    let mut s = s.into_iter().collect::<Vec<String>>().join(",");

    if streams.inverse {
        s.insert(0, '!');
    }

    s
}
