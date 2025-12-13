use super::Streams;
use crate::{IsDefault, ToJsonArgs};

impl ToJsonArgs for Streams {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if self.is_default() {
            return;
        }

        if self.no_flag {
            args.push(to_json_args!(NoStreams));
            return;
        }

        let arg = match arg(self) {
            s if s.is_empty() => return,
            s => s,
        };

        args.push(to_json_args!(Streams));
        args.push(arg);
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
