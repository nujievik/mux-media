use crate::{
    LangCode, MediaInfo, MuxError, TrackID, TrackType, Tracks, immut,
    markers::{
        MCAudioTracks, MCButtonTracks, MCSubTracks, MCVideoTracks, MITILang, MITargets,
        MITracksInfo,
    },
};
use enum_map::EnumMap;
use std::collections::BTreeSet;
use std::path::Path;

impl MediaInfo<'_> {
    pub(in super::super) fn build_saved_tracks(
        &mut self,
        media: &Path,
    ) -> Result<EnumMap<TrackType, BTreeSet<u64>>, MuxError> {
        let num_types: Vec<(u64, TrackType)> = self
            .try_get::<MITracksInfo>(media)?
            .into_iter()
            .map(|(num, cache)| (*num, cache.track_type))
            .collect();

        let num_lang_types: Vec<(u64, LangCode, TrackType)> = num_types
            .into_iter()
            .map(|(num, ttype)| {
                let lang = *self
                    .try_get_ti::<MITILang>(media, num)
                    .map(|lang| lang.inner())
                    .unwrap_or(&LangCode::default());
                (num, lang, ttype)
            })
            .collect();

        let targets = immut!(@try, self, MITargets, media)?;

        let mut audio_nums: BTreeSet<u64> = BTreeSet::new();
        let mut sub_nums: BTreeSet<u64> = BTreeSet::new();
        let mut video_nums: BTreeSet<u64> = BTreeSet::new();
        let mut button_nums: BTreeSet<u64> = BTreeSet::new();

        let mut a_tracks: Option<&Tracks> = None;
        let mut s_tracks: Option<&Tracks> = None;
        let mut v_tracks: Option<&Tracks> = None;
        let mut b_tracks: Option<&Tracks> = None;

        num_lang_types.into_iter().for_each(|(num, lang, tt)| {
            let tracks = match tt {
                TrackType::Audio => a_tracks.get_or_insert_with(|| {
                    self.mux_config.trg_field::<MCAudioTracks>(targets).inner()
                }),
                TrackType::Sub => s_tracks.get_or_insert_with(|| {
                    self.mux_config.trg_field::<MCSubTracks>(targets).inner()
                }),
                TrackType::Video => v_tracks.get_or_insert_with(|| {
                    self.mux_config.trg_field::<MCVideoTracks>(targets).inner()
                }),
                TrackType::Button => b_tracks.get_or_insert_with(|| {
                    self.mux_config.trg_field::<MCButtonTracks>(targets).inner()
                }),
                _ => return,
            };

            if tracks.save_track(&TrackID::Num(num), &TrackID::Lang(lang)) {
                let nums = match tt {
                    TrackType::Audio => &mut audio_nums,
                    TrackType::Sub => &mut sub_nums,
                    TrackType::Video => &mut video_nums,
                    TrackType::Button => &mut button_nums,
                    _ => return,
                };
                nums.insert(num);
            }
        });

        let mut map = TrackType::map::<BTreeSet<u64>>();
        map[TrackType::Audio] = audio_nums;
        map[TrackType::Sub] = sub_nums;
        map[TrackType::Video] = video_nums;
        map[TrackType::Button] = button_nums;

        Ok(map)
    }
}
