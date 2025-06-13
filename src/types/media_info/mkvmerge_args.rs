use super::MediaInfo;
use crate::{
    MCAudioTracks, MCButtonTracks, MCChapters, MCDefaultTFlags, MCEnabledTFlags, MCFontAttachs,
    MCForcedTFlags, MCSpecials, MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks, MITargets,
    ToMkvmergeArgs,
};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

impl MediaInfo<'_> {
    pub fn collect_os_mkvmerge_args(&mut self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();
        self.extend_vec_os_mkvmerge_args(&mut args);
        args
    }

    pub fn extend_vec_os_mkvmerge_args(&mut self, args: &mut Vec<OsString>) {
        let paths: Vec<PathBuf> = self.cache.keys().cloned().collect();

        for path in paths {
            extend_file_args(args, self, &path);
            args.push(path.into_os_string());
        }
    }
}

fn extend_file_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    let targets = match mi.get::<MITargets>(path) {
        Some(targets) => targets.clone(),
        None => return,
    };

    args.extend(
        mi.mc
            .get_trg::<MCAudioTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCSubTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCVideoTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCButtonTracks>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );

    args.extend(
        mi.mc
            .get_trg::<MCChapters>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCFontAttachs>(&targets)
            .inner() // Added fonts and other attachs
            .to_os_mkvmerge_args(mi, path),
    );

    args.extend(
        mi.mc
            .get_trg::<MCDefaultTFlags>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCForcedTFlags>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCEnabledTFlags>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );

    args.extend(
        mi.mc
            .get_trg::<MCTrackNames>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCTrackLangs>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
    args.extend(
        mi.mc
            .get_trg::<MCSpecials>(&targets)
            .to_os_mkvmerge_args(mi, path),
    );
}
