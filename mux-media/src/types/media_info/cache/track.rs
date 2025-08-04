use crate::{CacheState, LangCode, MuxError, TrackID, TrackType, Value};

/// Cache of [`crate::MediaInfo`] is separate for each track in media.
#[derive(Clone, Debug, Default)]
pub struct CacheMIOfFileTrack {
    pub raw: RawTrackCache,
    pub track_type: TrackType,

    pub lang: CacheState<Value<LangCode>>,
    pub name: CacheState<Value<String>>,
    pub words_name: CacheState<Vec<String>>,
    pub track_ids: CacheState<[TrackID; 2]>,
    pub codec: CacheState<String>,
    pub it_signs: CacheState<bool>,
}

#[derive(Clone, Debug)]
pub enum RawTrackCache {
    Matroska(matroska::Track),
    Mkvmerge(String),
}

impl Default for RawTrackCache {
    fn default() -> Self {
        RawTrackCache::Mkvmerge(String::new())
    }
}

impl TryFrom<&matroska::Track> for CacheMIOfFileTrack {
    type Error = MuxError;

    fn try_from(mt: &matroska::Track) -> Result<Self, Self::Error> {
        let track_type = TrackType::from(mt.tracktype);

        if let TrackType::NonCustomizable = track_type {
            return Err("Unsupported track type".into());
        }

        Ok(Self {
            raw: RawTrackCache::Matroska(mt.clone()),
            track_type,
            ..Default::default()
        })
    }
}

impl TryFrom<String> for CacheMIOfFileTrack {
    type Error = MuxError;

    fn try_from(mkvmerge_id_line: String) -> Result<Self, Self::Error> {
        let track_type = [
            TrackType::Video,
            TrackType::Audio,
            TrackType::Sub,
            TrackType::Button,
        ]
        .into_iter()
        .find(|tt| mkvmerge_id_line.contains(tt.as_str_mkvtoolnix()))
        .ok_or_else(|| MuxError::from("Unsupported track type"))?;

        Ok(Self {
            raw: RawTrackCache::Mkvmerge(mkvmerge_id_line),
            track_type,
            ..Default::default()
        })
    }
}
