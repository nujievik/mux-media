use crate::{
    CacheState, LangCode, MCAudioTracks, MCButtonTracks, MCSubTracks, MCVideoTracks, MITILang,
    MITargets, MITracksInfo, MediaInfo, MuxError, TrackID, TrackType, Tracks,
};
use std::collections::BTreeSet;
use std::path::Path;

impl MediaInfo<'_> {
    pub(in super::super) fn build_saved_audio_u32_ids(
        &mut self,
        path: &Path,
    ) -> Result<BTreeSet<u32>, MuxError> {
        let tids = self.try_set_build_saved_u32_ids(path, TrackType::Audio)?;
        Ok(tids)
    }

    pub(in super::super) fn build_saved_sub_u32_ids(
        &mut self,
        path: &Path,
    ) -> Result<BTreeSet<u32>, MuxError> {
        let tids = self.try_set_build_saved_u32_ids(path, TrackType::Sub)?;
        Ok(tids)
    }

    pub(in super::super) fn build_saved_video_u32_ids(
        &mut self,
        path: &Path,
    ) -> Result<BTreeSet<u32>, MuxError> {
        let tids = self.try_set_build_saved_u32_ids(path, TrackType::Video)?;
        Ok(tids)
    }

    pub(in super::super) fn build_saved_button_u32_ids(
        &mut self,
        path: &Path,
    ) -> Result<BTreeSet<u32>, MuxError> {
        let tids = self.try_set_build_saved_u32_ids(path, TrackType::Button)?;
        Ok(tids)
    }

    fn try_set_build_saved_u32_ids(
        &mut self,
        path: &Path,
        target_tt: TrackType,
    ) -> Result<BTreeSet<u32>, MuxError> {
        let tid_u32_type: Vec<(TrackID, u32, TrackType)> = self
            .try_get::<MITracksInfo>(path)?
            .into_iter()
            .map(|(id, cache)| (*id, cache.id_u32, cache.track_type))
            .collect();

        let targets = self.try_get::<MITargets>(path)?.clone();

        let u32_lang_type: Vec<(u32, LangCode, TrackType)> = tid_u32_type
            .into_iter()
            .map(|(id, u32, tt)| {
                let lang = *self
                    .try_get_ti::<MITILang>(path, id)
                    .unwrap_or(&LangCode::default());
                (u32, lang, tt)
            })
            .collect();

        let mut rm: Vec<TrackID> = Vec::new();
        let mut audio: BTreeSet<u32> = BTreeSet::new();
        let mut subs: BTreeSet<u32> = BTreeSet::new();
        let mut videos: BTreeSet<u32> = BTreeSet::new();
        let mut buttons: BTreeSet<u32> = BTreeSet::new();

        let mut a_tracks: Option<&Tracks> = None;
        let mut s_tracks: Option<&Tracks> = None;
        let mut v_tracks: Option<&Tracks> = None;
        let mut b_tracks: Option<&Tracks> = None;

        u32_lang_type.into_iter().for_each(|(u32, lang, tt)| {
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

            if tracks.save_track(u32, lang) {
                let tids = match tt {
                    TrackType::Audio => &mut audio,
                    TrackType::Sub => &mut subs,
                    TrackType::Video => &mut videos,
                    TrackType::Button => &mut buttons,
                };
                tids.insert(u32);
            } else {
                rm.push(TrackID::U32(u32));
            }
        });

        // Remove unused tracks_info
        if !rm.is_empty() {
            let ti = self
                .get_mut_tracks_info(path)
                .expect("Unexpected None tracks_info");
            ti.retain(|tid, _| !rm.contains(tid));
        }

        // Write in cache
        let cache = self
            .cache
            .get_mut(path)
            .expect("Unexpected None cache entry");
        match target_tt {
            TrackType::Audio => {
                cache.saved_sub_u32_ids = CacheState::Cached(subs);
                cache.saved_video_u32_ids = CacheState::Cached(videos);
                cache.saved_button_u32_ids = CacheState::Cached(buttons);
                Ok(audio)
            }
            TrackType::Sub => {
                cache.saved_audio_u32_ids = CacheState::Cached(audio);
                cache.saved_video_u32_ids = CacheState::Cached(videos);
                cache.saved_button_u32_ids = CacheState::Cached(buttons);
                Ok(subs)
            }
            TrackType::Video => {
                cache.saved_audio_u32_ids = CacheState::Cached(audio);
                cache.saved_sub_u32_ids = CacheState::Cached(subs);
                cache.saved_button_u32_ids = CacheState::Cached(buttons);
                Ok(videos)
            }
            TrackType::Button => {
                cache.saved_audio_u32_ids = CacheState::Cached(audio);
                cache.saved_sub_u32_ids = CacheState::Cached(subs);
                cache.saved_video_u32_ids = CacheState::Cached(videos);
                Ok(buttons)
            }
        }
    }
}
