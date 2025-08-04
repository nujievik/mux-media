use crate::{TrackLangs, to_ffmpeg_args, to_json_args, to_mkvmerge_args};

to_ffmpeg_args!(@names_or_langs, TrackLangs, Language, auto_langs, MITILang);
to_json_args!(@names_or_langs, TrackLangs, Langs);
to_mkvmerge_args!(@names_or_langs, TrackLangs, Language, auto_langs, MITILang);
