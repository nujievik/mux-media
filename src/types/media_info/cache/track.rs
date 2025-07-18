use crate::{CacheState, LangCode, MuxError, TrackID, TrackType};

/// Cache of [`crate::MediaInfo`] is separate for each track in media.
#[derive(Clone, Debug, Default)]
pub struct CacheMIOfFileTrack {
    pub matroska: Option<matroska::Track>,
    pub mkvmerge_id_line: Option<String>,
    pub track_type: TrackType,
    pub lang: CacheState<LangCode>,
    pub name: CacheState<String>,
    pub track_ids: CacheState<[TrackID; 2]>,
}

impl TryFrom<matroska::Track> for CacheMIOfFileTrack {
    type Error = MuxError;

    fn try_from(mt: matroska::Track) -> Result<Self, Self::Error> {
        let track_type = TrackType::from(mt.tracktype);

        if let TrackType::NonCustomizable = track_type {
            return Err("Unsupported track type".into());
        }

        Ok(Self {
            matroska: Some(mt),
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
            mkvmerge_id_line: Some(mkvmerge_id_line),
            track_type,
            ..Default::default()
        })
    }
}
