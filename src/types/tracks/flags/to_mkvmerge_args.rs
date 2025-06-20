use super::{DefaultTFlags, EnabledTFlags, ForcedTFlags};
use crate::{
    MISavedTracks, MediaInfo, ToMkvmergeArgs, mkvmerge_arg, to_mkvmerge_args, unwrap_or_return_vec,
};
use std::path::Path;

mkvmerge_arg!(DefaultTFlags, "--default-track-flag");
mkvmerge_arg!(ForcedTFlags, "--forced-display-flag");
mkvmerge_arg!(EnabledTFlags, "--track-enabled-flag");

impl ToMkvmergeArgs for DefaultTFlags {
    fn to_mkvmerge_args(&self, mi: &mut MediaInfo, path: &Path) -> Vec<String> {
        let nums = unwrap_or_return_vec!(mi.get::<MISavedTracks>(path));

        for _num in nums.values().flatten() {}

        Vec::new()
    }

    to_mkvmerge_args!(@fn_os);
}
