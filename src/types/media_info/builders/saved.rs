use crate::{
    MediaInfo, Result, TrackType, Tracks, immut,
    markers::{MCAudioTracks, MCSubTracks, MCVideoTracks, MITITrackIDs, MITargets, MITracksInfo},
};
use enum_map::EnumMap;
use std::{collections::BTreeSet, path::Path};

impl MediaInfo<'_> {
    pub(crate) fn build_saved_tracks(
        &mut self,
        media: &Path,
    ) -> Result<EnumMap<TrackType, Vec<u64>>> {
        self.try_get::<MITracksInfo>(media)?
            .iter()
            .map(|(&t, _)| t)
            .collect::<Vec<u64>>()
            .into_iter()
            .try_for_each(|t| self.try_init_ti::<MITITrackIDs>(media, t))?;

        let targets = immut!(@try, self, MITargets, media)?;

        let mut audio_nums: BTreeSet<u64> = BTreeSet::new();
        let mut sub_nums: BTreeSet<u64> = BTreeSet::new();
        let mut video_nums: BTreeSet<u64> = BTreeSet::new();
        let mut a_tracks: Option<&Tracks> = None;
        let mut s_tracks: Option<&Tracks> = None;
        let mut v_tracks: Option<&Tracks> = None;

        self.try_immut::<MITracksInfo>(media)?
            .iter()
            .for_each(|(&num, cache)| {
                let ty = cache.ty;
                let tids = unwrap_or_return!(self.immut_ti::<MITITrackIDs>(media, num));

                let tracks =
                    match ty {
                        TrackType::Audio => a_tracks
                            .get_or_insert_with(|| &self.cfg.target(MCAudioTracks, targets).0),
                        TrackType::Sub => {
                            s_tracks.get_or_insert_with(|| &self.cfg.target(MCSubTracks, targets).0)
                        }
                        TrackType::Video => v_tracks
                            .get_or_insert_with(|| &self.cfg.target(MCVideoTracks, targets).0),
                        _ => return,
                    };

                if tracks.save_track(&tids[0], &tids[1]) {
                    let nums = match ty {
                        TrackType::Audio => &mut audio_nums,
                        TrackType::Sub => &mut sub_nums,
                        TrackType::Video => &mut video_nums,
                        _ => return,
                    };
                    nums.insert(num);
                }
            });

        let mut map = TrackType::map::<Vec<u64>>();
        map[TrackType::Audio] = audio_nums.into_iter().collect();
        map[TrackType::Sub] = sub_nums.into_iter().collect();
        map[TrackType::Video] = video_nums.into_iter().collect();

        Ok(map)
    }
}
