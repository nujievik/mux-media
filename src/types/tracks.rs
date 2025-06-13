mod fields;
pub(crate) mod flags;
mod from_arg_matches;
pub(crate) mod id;
mod langs;
mod names;
mod to_mkvmerge_args;
pub(crate) mod track_type;

use crate::LangCode;
use id::TrackID;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct AudioTracks(Tracks);
#[derive(Clone)]
pub struct SubTracks(Tracks);
#[derive(Clone)]
pub struct VideoTracks(Tracks);
#[derive(Clone)]
pub struct ButtonTracks(Tracks);

#[derive(Clone, Default)]
pub struct Tracks {
    no_flag: bool,
    inverse: bool,
    ids_hashed: Option<HashSet<TrackID>>,
    ids_unhashed: Option<Vec<TrackID>>,
}

#[derive(Clone)]
pub struct DefaultTFlags(TFlags);
#[derive(Clone)]
pub struct ForcedTFlags(TFlags);
#[derive(Clone)]
pub struct EnabledTFlags(TFlags);

#[derive(Clone, Default)]
pub struct TFlags {
    lim_for_unset: Option<u32>,
    unmapped: Option<bool>,
    map_hashed: Option<HashMap<TrackID, bool>>,
    map_unhashed: Option<Vec<(TrackID, bool)>>,
}

#[derive(Clone, Default)]
pub struct TrackNames {
    unmapped: Option<String>,
    map_hashed: Option<HashMap<TrackID, String>>,
    map_unhashed: Option<Vec<(TrackID, String)>>,
}

#[derive(Clone, Default)]
pub struct TrackLangs {
    unmapped: Option<LangCode>,
    map_hashed: Option<HashMap<TrackID, LangCode>>,
    map_unhashed: Option<Vec<(TrackID, LangCode)>>,
}
