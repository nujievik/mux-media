use super::Raws;
use crate::{MediaInfo, MuxError, ToJsonArgs, ToMkvmergeArgs, dashed};
use std::{ffi::OsString, path::Path};

impl ToJsonArgs for Raws {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if let Some(spls) = self.0.as_ref().filter(|spls| !spls.is_empty()) {
            args.push(dashed!(Raws).into());
            args.push(spls.join(" "));
        }
    }
}

impl ToMkvmergeArgs for Raws {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        _: &mut MediaInfo,
        _: &Path,
    ) -> Result<(), MuxError> {
        if let Some(spls) = &self.0 {
            args.extend(spls.iter().map(Into::into));
        }

        Ok(())
    }
}
