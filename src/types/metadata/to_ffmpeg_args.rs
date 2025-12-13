use super::{LangMetadata, NameMetadata};
use crate::{Lang, MediaInfo, Result, Stream, ToFfmpegArgs, Value, markers::*};
use std::ffi::OsString;

macro_rules! to_ffmpeg_args_impl {
    ($ty:ty, $auto_field:ident) => {
        impl ToFfmpegArgs for $ty {
            fn append_ffmpeg_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<()> {
                let auto = *mi.cfg.auto_flags.$auto_field;
                let order = mi.try_take_cmn(MICmnStreamsOrder)?;

                for (i, m) in order.iter_track().enumerate() {
                    let stream = &mi.try_get(MIStreams, &m.key)?[m.key_i_stream];
                    if let Some(val) = <$ty>::get_meta_val(stream) {
                        if auto || val.is_user() {
                            args.push(format!("-metadata:s:{}", i).into());
                            args.push(format!("{}={}", <$ty>::META_KEY, val).into());
                        }
                    }
                }
                mi.set_cmn(MICmnStreamsOrder, order);
                Ok(())
            }
        }
    };
}

to_ffmpeg_args_impl!(NameMetadata, names);
to_ffmpeg_args_impl!(LangMetadata, langs);

impl NameMetadata {
    const META_KEY: &str = "title";

    fn get_meta_val(stream: &Stream) -> Option<Value<&String>> {
        stream.name.as_ref().map(|n| n.as_ref())
    }
}

impl LangMetadata {
    const META_KEY: &str = "language";

    fn get_meta_val(stream: &Stream) -> Option<Value<&Lang>> {
        Some(stream.lang.as_ref())
    }
}
