use super::{Retiming, SubType};
use crate::Extension;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Destination {
    pub src_ext: Extension,
    pub ty: SubType,
    pub path: PathBuf,
}

impl Retiming<'_, '_> {
    pub(super) fn new_destination(
        &self,
        i: usize,
        src: &Path,
        i_stream: usize,
        is_base: bool,
    ) -> Destination {
        let src_ext = Extension::new_from_path(src).unwrap_or(Extension::Mkv);

        let (ty, path) = if is_base {
            let ty = SubType::from_codec_id(self.media_info, src, i_stream);
            let path = self.temp_dir.join(format!(
                "{}-sub-base-{}.{}",
                self.job,
                i_stream,
                ty.as_ext()
            ));
            (ty, path)
        } else {
            let ty = SubType::new_from_path(src)
                .unwrap_or_else(|| SubType::from_codec_id(self.media_info, src, i_stream));
            let path = self
                .temp_dir
                .join(format!("{}-sub-{}.{}", self.job, i, ty.as_ext()));
            (ty, path)
        };

        Destination { src_ext, ty, path }
    }
}
