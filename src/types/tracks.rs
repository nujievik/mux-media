pub(crate) mod id;
mod impls;
mod to_json_args;
mod to_mkvmerge_args;
pub(crate) mod track_type;

use crate::{IsDefault, deref_singleton_tuple_struct};
use id::TrackID;
use std::collections::HashSet;

/// Settings for saving media audio tracks.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct AudioTracks(pub Tracks);

/// Settings for saving media subtitle tracks.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct SubTracks(pub Tracks);

/// Settings for saving media video tracks.
#[derive(Clone, Debug, PartialEq, IsDefault)]
pub struct VideoTracks(pub Tracks);

/// Common interface for savings of tracks by type.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Tracks {
    pub no_flag: bool,
    pub inverse: bool,
    pub ids_hashed: Option<HashSet<TrackID>>,
    pub ids_unhashed: Option<Vec<TrackID>>,
}

deref_singleton_tuple_struct!(AudioTracks, Tracks, @all);
deref_singleton_tuple_struct!(SubTracks, Tracks, @all);
deref_singleton_tuple_struct!(VideoTracks, Tracks, @all);
