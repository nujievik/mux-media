// Get Field MuxConfig markers
pub use crate::types::mux_config::fields::{
    MCAudioTracks, MCChapters, MCDefaultTrackFlags, MCFontAttachs, MCForcedTrackFlags,
    MCOtherAttachs, MCSubTracks, MCTrackLangs, MCTrackNames, MCVideoTracks,
};

// Get Field MediaInfo markers
pub use crate::types::media_info::lazy_fields::{
    MIAttachsInfo, MIAudioDuration, MICache, MICmnExternalFonts, MICmnStem, MICmnTrackOrder,
    MIFfmpegStreams, MIMatroska, MIPathTail, MIPlayableDuration, MIRelativeUpmost, MISavedTracks,
    MISubCharset, MITICache, MITIItSigns, MITILang, MITIName, MITITrackIDs, MITIWordsName,
    MITargetGroup, MITargets, MITracksInfo, MIVideoDuration, MIWordsPathTail,
    MIWordsRelativeUpmost,
};
