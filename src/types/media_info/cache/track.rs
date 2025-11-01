use crate::{CacheState, CodecID, LangCode, TrackID, TrackType, Value};

/// Cache of [`crate::MediaInfo`] is separate for each track in media.
#[derive(Clone, Debug, Default)]
pub struct CacheMIOfFileTrack {
    pub stream_i: usize,
    pub codec_id: CodecID,
    pub ty: TrackType,

    pub lang: CacheState<Value<LangCode>>,
    pub name: CacheState<Value<String>>,
    pub words_name: CacheState<Vec<String>>,
    pub track_ids: CacheState<[TrackID; 2]>,
    pub it_signs: CacheState<bool>,
}
