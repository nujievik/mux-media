pub(crate) mod flags;
mod from_arg_matches;
pub(crate) mod id;
mod impls;
pub(crate) mod langs;
pub(crate) mod names;
pub(crate) mod order;
mod to_json_args;
mod to_mkvmerge_args;
pub(crate) mod track_type;

use id::TrackID;
use std::collections::HashSet;

/// Settings for saving media audio tracks.
#[derive(Clone)]
pub struct AudioTracks(Tracks);

/// Settings for saving media subtitle tracks.
#[derive(Clone)]
pub struct SubTracks(Tracks);

/// Settings for saving media video tracks.
#[derive(Clone)]
pub struct VideoTracks(Tracks);

/// Settings for saving media button tracks.
#[derive(Clone)]
pub struct ButtonTracks(Tracks);

/// Settings for saving media tracks.
#[derive(Clone, Default)]
pub struct Tracks {
    no_flag: bool,
    inverse: bool,
    ids_hashed: Option<HashSet<TrackID>>,
    ids_unhashed: Option<Vec<TrackID>>,
}
