use super::{MuxCurrent, Muxer};
use crate::{MediaInfo, Msg, MuxError, Tool, ToolOutput, Tools, TrackOrder, TrackType, i18n::logs};
use log::warn;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    path::Path,
};

impl Muxer {
    #[inline(always)]
    pub(super) fn mux_current_avi(
        tools: &Tools,
        mi: &mut MediaInfo,
        out: &Path,
    ) -> MuxCurrent<ToolOutput> {
        let mut args = Vec::<OsString>::new();

        if let Err(e) = try_append_args(&mut args, mi) {
            return MuxCurrent::Err(e);
        }

        if args.len() < 3 {
            logs::warn_not_out_save_any(out);
            mi.clear_current();
            return MuxCurrent::Continue;
        }

        args.push(out.into());

        tools.run(Tool::Ffmpeg, &args).into()
    }
}

#[inline(always)]
fn try_append_args(args: &mut Vec<OsString>, mi: &mut MediaInfo) -> Result<(), MuxError> {
    let (media, i_track_type) = match TrackOrder::try_from(mi) {
        Ok(order) => (order.media, order.i_track_type),
        Err(e) => return Err(e),
    };

    let mut out_args = Vec::<OsString>::new();
    let mut pusheds: HashMap<usize, HashSet<u64>> = HashMap::new();
    let mut cnt = 0;

    let mut push = |track_type: TrackType, s_type: char| {
        if let Some((i, track, _)) = i_track_type
            .iter()
            .filter(|(_, _, tt)| tt == &track_type)
            .next()
        {
            let i = *i;
            args.push("-i".into());
            args.push(media[i].as_path().into());
            out_args.push("-map".into());
            out_args.push(format!("{}:{}:{}", cnt, s_type, track).into());

            pusheds.entry(i).or_insert_with(HashSet::new).insert(*track);
            cnt += 1;
        } else {
            warn!("{} {}", Msg::NotFoundTrack, track_type.as_str_mkvtoolnix());
        }
    };

    push(TrackType::Video, 'v');
    push(TrackType::Audio, 'a');

    i_track_type
        .into_iter()
        .filter(|(i, t, _)| !pusheds.get(i).map_or(false, |tracks| tracks.contains(t)))
        .for_each(|(i, t, _)| logs::avi_container_does_not_support(&media[i], t));

    args.append(&mut out_args);

    Ok(())
}
