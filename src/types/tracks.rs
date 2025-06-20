pub(crate) mod flags;
mod from_arg_matches;
pub(crate) mod id;
mod impls;
pub(crate) mod langs;
pub(crate) mod names;
pub(crate) mod order;
mod to_mkvmerge_args;
pub(crate) mod track_type;

use id::TrackID;
use std::collections::HashSet;

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
