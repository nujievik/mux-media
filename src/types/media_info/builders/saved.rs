use crate::{
    LangCode, MCAudioTracks, MCButtonTracks, MCSubTracks, MCVideoTracks, MITILang, MITargets,
    MITracksInfo, MediaInfo, MuxError, TrackID, TrackType, Tracks,
};
use enum_map::EnumMap;
use std::collections::BTreeSet;
use std::path::Path;

impl MediaInfo<'_> {
    pub(in super::super) fn build_saved_track_nums(
        &mut self,
        path: &Path,
    ) -> Result<EnumMap<TrackType, BTreeSet<u64>>, MuxError> {
        let tid_types: Vec<(TrackID, TrackType)> = self
            .try_get::<MITracksInfo>(path)?
            .into_iter()
            .map(|(id, cache)| {
                let tid = match id {
                    TrackID::Num(_) => *id,
                    _ => TrackID::Num(cache.num),
                };
                (tid, cache.track_type)
            })
            .collect();

        let targets = self.try_get::<MITargets>(path)?.clone();

        let tid_tid_types: Vec<(TrackID, TrackID, TrackType)> = tid_types
            .into_iter()
            .map(|(tid, ttype)| {
                let lang = *self
                    .try_get_ti::<MITILang>(path, &tid)
                    .unwrap_or(&LangCode::default());
                (tid, TrackID::Lang(lang), ttype)
            })
            .collect();

        let mut audio_nums: BTreeSet<u64> = BTreeSet::new();
        let mut sub_nums: BTreeSet<u64> = BTreeSet::new();
        let mut video_nums: BTreeSet<u64> = BTreeSet::new();
        let mut button_nums: BTreeSet<u64> = BTreeSet::new();

        let mut a_tracks: Option<&Tracks> = None;
        let mut s_tracks: Option<&Tracks> = None;
        let mut v_tracks: Option<&Tracks> = None;
        let mut b_tracks: Option<&Tracks> = None;

        tid_tid_types.into_iter().for_each(|(tid, l_tid, tt)| {
            let tracks = match tt {
                TrackType::Audio => a_tracks
                    .get_or_insert_with(|| self.mc.get_trg::<MCAudioTracks>(&targets).inner()),
                TrackType::Sub => {
                    s_tracks.get_or_insert_with(|| self.mc.get_trg::<MCSubTracks>(&targets).inner())
                }
                TrackType::Video => v_tracks
                    .get_or_insert_with(|| self.mc.get_trg::<MCVideoTracks>(&targets).inner()),
                TrackType::Button => b_tracks
                    .get_or_insert_with(|| self.mc.get_trg::<MCButtonTracks>(&targets).inner()),
            };

            if tracks.save_track(&tid, &l_tid) {
                let nums = match tt {
                    TrackType::Audio => &mut audio_nums,
                    TrackType::Sub => &mut sub_nums,
                    TrackType::Video => &mut video_nums,
                    TrackType::Button => &mut button_nums,
                };
                if let TrackID::Num(num) = tid {
                    nums.insert(num);
                }
            }
        });

        let mut map: EnumMap<TrackType, BTreeSet<u64>> = TrackType::new_enum_map();
        map[TrackType::Audio] = audio_nums;
        map[TrackType::Sub] = sub_nums;
        map[TrackType::Video] = video_nums;
        map[TrackType::Button] = button_nums;

        Ok(map)
    }
}
