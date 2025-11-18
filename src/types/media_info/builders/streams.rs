use super::MediaInfo;
use crate::{CodecId, EXTENSIONS, LangCode, Result, Stream, StreamType, Value};
use std::path::Path;

impl MediaInfo<'_> {
    pub(crate) fn build_streams(&self, src: &Path) -> Result<Vec<Stream>> {
        Self::help_build_streams(src)
    }

    pub(crate) fn help_build_streams(src: &Path) -> Result<Vec<Stream>> {
        use crate::ffmpeg::{format::input, util::media::Type};

        let ictx = input(src)?;
        let mut idxs = StreamType::map::<usize>();

        Ok(ictx
            .streams()
            .map(|stream| {
                let meta = stream.metadata();
                let lang = meta
                    .get("language")
                    .and_then(|v| v.parse::<LangCode>().ok())
                    .unwrap_or_default();
                let name = meta.get("title").map(|v| Value::Auto(v.to_owned()));
                let filename = meta.get("filename").map(|s| s.to_owned());

                let params = stream.parameters();
                let codec = CodecId(params.id());

                let is_font = || codec.is_font() || is_font_filename(&filename);

                let ty = match params.medium() {
                    Type::Audio => StreamType::Audio,
                    Type::Subtitle => StreamType::Sub,
                    Type::Video if codec.is_attach() => StreamType::Attach,
                    Type::Video => StreamType::Video,
                    Type::Attachment if is_font() => StreamType::Font,
                    Type::Attachment => StreamType::Attach,
                    _ => StreamType::Other,
                };

                let i = stream.index();
                let i_ty = idxs[ty];
                idxs[ty] += 1;

                Stream {
                    ty,
                    i,
                    i_ty,
                    codec,
                    lang: Value::Auto(lang),
                    name,
                    filename,
                }
            })
            .collect())
    }
}

fn is_font_filename(opt_s: &Option<String>) -> bool {
    opt_s.as_ref().is_some_and(|s| {
        Path::new(s)
            .extension()
            .is_some_and(|ext| EXTENSIONS.fonts.contains(ext.as_encoded_bytes()))
    })
}
