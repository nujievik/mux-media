use super::Specials;
use crate::{MediaInfo, ToMkvmergeArgs, to_mkvmerge_args};

impl ToMkvmergeArgs for Specials {
    fn to_mkvmerge_args(&self, _: &mut MediaInfo, _: &std::path::Path) -> Vec<String> {
        if let Some(vec) = &self.0 {
            vec.clone()
        } else {
            Vec::new()
        }
    }

    to_mkvmerge_args!(@fn_os);
}
