use super::MediaInfo;
use crate::{
    ArcPathBuf, MCAudioTracks, MCButtonTracks, MCChapters, MCDefaultTFlags, MCEnabledTFlags,
    MCFontAttachs, MCForcedTFlags, MCSpecials, MCSubTracks, MCTrackLangs, MCTrackNames,
    MCVideoTracks, MISubCharset, MITargets, MuxError, TFlags, ToMkvmergeArgs, TrackOrder,
};
use std::{ffi::OsString, path::Path};

impl MediaInfo<'_> {
    /// Collects mkvmerge args of all appended media.
    pub fn collect_os_mkvmerge_args(&mut self) -> Vec<OsString> {
        let mut args = Vec::<OsString>::new();
        self.append_os_mkvmerge_args(&mut args);
        args
    }

    /// Appends mkvmerge args of all appended media to [`Vec<OsString>`].
    pub fn append_os_mkvmerge_args(&mut self, args: &mut Vec<OsString>) {
        match self.init_track_order() {
            Ok(order) => {
                // self and Path unused, just trait requirements
                args.append(&mut order.to_os_mkvmerge_args(self, Path::new("")));

                let (i_route, paths, mut path_args) =
                    TFlags::track_order_to_os_mkvmerge_args(self, order);

                i_route.into_iter().for_each(|i| {
                    append_target_args(args, self, &paths[i]);
                    args.append(&mut std::mem::take(&mut path_args[i]));
                    args.push(paths[i].as_ref().into());
                })
            }

            Err(e) => {
                log::warn!("{}", e);
                let paths: Vec<ArcPathBuf> = self.cache.of_files.keys().cloned().collect();
                for path in paths {
                    append_target_args(args, self, &path);
                    fallback_append_target_flags(args, self, &path);
                    args.push(path.as_ref().into());
                }
            }
        };
    }

    #[inline(always)]
    fn init_track_order(&mut self) -> Result<TrackOrder, MuxError> {
        TrackOrder::try_from(self)
    }
}

macro_rules! append_target_args {
    ($args:expr, $mi:ident, $path:expr; $( $field:ident ),*) => {
        // Cache MITargets if need. Immediate return if None
        if let None = $mi.get::<MITargets>($path) {
            return;
        }

        $(
            if let Some(targets) = $mi.unmut_get::<MITargets>($path) {
                let mut args = $mi.mc.get_trg::<$field>(targets).to_os_mkvmerge_args($mi, $path);
                $args.append(&mut args);
            }
        )*
    };
}

fn append_target_args(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    append_target_args!(
        args, mi, path;
        MCAudioTracks, MCSubTracks, MCVideoTracks, MCButtonTracks,
        MCChapters, MCFontAttachs, MCTrackNames, MCTrackLangs,
        MCSpecials
    );

    if let Some(Ok(cs)) = mi
        .pro_flags
        .add_charsets
        .then(|| mi.try_get::<MISubCharset>(path).map(|c| c.clone()))
    {
        args.append(&mut cs.to_os_mkvmerge_args(mi, path))
    }
}

#[inline(always)]
fn fallback_append_target_flags(args: &mut Vec<OsString>, mi: &mut MediaInfo, path: &Path) {
    append_target_args!(
        args, mi, path;
        MCDefaultTFlags, MCForcedTFlags, MCEnabledTFlags
    );
}
