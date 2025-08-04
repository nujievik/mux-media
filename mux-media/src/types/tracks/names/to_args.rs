use super::TrackNames;
use crate::{to_ffmpeg_args, to_json_args, to_mkvmerge_args};

to_ffmpeg_args!(@names_or_langs, TrackNames, Title, auto_names, MITIName);
to_json_args!(@names_or_langs, TrackNames, Names);
to_mkvmerge_args!(@names_or_langs, TrackNames, TrackName, auto_names, MITIName);
