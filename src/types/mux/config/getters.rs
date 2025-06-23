use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs, ForcedTFlags,
    GetField, GetOptField, Input, LangCode, OffOnPro, OtherAttachs, Output, Retiming, Specials,
    SubTracks, TFlagType, TFlags, Target, Tools, TrackLangs, TrackNames, Verbosity, VideoTracks,
    get_fields,
};

get_fields! {
    MuxConfig;
    input, Input => MCInput,
    output, Output => MCOutput,
    verbosity, Verbosity => MCVerbosity,
    locale, LangCode => MCLocale,
    exit_on_err, bool => MCExitOnErr,
    off_on_pro, OffOnPro => MCOffOnPro,
    retiming, Retiming => MCRetiming,
    tools, Tools => MCTools,
}

get_fields! {
    MuxConfig, MuxConfigTarget;
    audio_tracks, AudioTracks => MCAudioTracks,
    sub_tracks, SubTracks => MCSubTracks,
    video_tracks, VideoTracks => MCVideoTracks,
    button_tracks, ButtonTracks => MCButtonTracks,
    chapters, Chapters => MCChapters,
    font_attachs, FontAttachs => MCFontAttachs,
    other_attachs, OtherAttachs => MCOtherAttachs,
    default_t_flags, DefaultTFlags => MCDefaultTFlags,
    forced_t_flags, ForcedTFlags => MCForcedTFlags,
    enabled_t_flags, EnabledTFlags => MCEnabledTFlags,
    track_names, TrackNames => MCTrackNames,
    track_langs, TrackLangs => MCTrackLangs,
    specials, Specials => MCSpecials,
}

impl MuxConfig {
    pub fn get<F>(&self) -> &<Self as GetField<F>>::FieldType
    where
        Self: GetField<F>,
    {
        <Self as GetField<F>>::get(self)
    }

    pub fn get_trg<F>(&self, targets: &[Target]) -> &<Self as GetField<F>>::FieldType
    where
        Self: GetField<F>,
        MuxConfigTarget: GetOptField<F, FieldType = <Self as GetField<F>>::FieldType>,
    {
        if let Some(map) = self.targets.as_ref() {
            for target in targets {
                if let Some(tgt_cfg) = map.get(target) {
                    if let Some(value) = <MuxConfigTarget as GetOptField<F>>::get(tgt_cfg) {
                        return value;
                    }
                }
            }
        }

        self.get::<F>()
    }

    pub fn get_trg_t_flags(&self, targets: &[Target], ft: TFlagType) -> &TFlags {
        match ft {
            TFlagType::Default => self.get_trg::<MCDefaultTFlags>(targets).inner(),
            TFlagType::Forced => self.get_trg::<MCForcedTFlags>(targets).inner(),
            TFlagType::Enabled => self.get_trg::<MCEnabledTFlags>(targets).inner(),
        }
    }
}
