use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, FontAttachs, ForcedTFlags,
    GetField, GetOptField, Input, LangCode, OffOnPro, OtherAttachs, Output, Retiming, Specials,
    SubTracks, TFlagType, TFlags, Target, Tools, TrackLangs, TrackNames, Verbosity, VideoTracks,
};
use std::path::Path;

impl MuxConfig {
    /// Returns a reference to the value associated with the marker type `F`.
    pub fn get<F>(&self) -> &<Self as GetField<F>>::FieldType
    where
        Self: GetField<F>,
    {
        <Self as GetField<F>>::get(self)
    }

    /// Returns a reference to the value associated with the marker type `F`
    /// from the first matching `target` in the provided list, if available;
    /// otherwise returns the common (global) value.
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

    /// Returns a cloned [`Target`] if it exists in [`MuxConfig`]; otherwise, returns `None`.
    ///
    /// This operation avoids heap allocation:
    /// internally it either copies an enum variant or increments the `Arc` reference count.
    pub fn get_clone_target(&self, path: impl AsRef<Path>) -> Option<Target> {
        self.targets
            .as_ref()
            .and_then(|map| map.get_key_value(path.as_ref()).map(|(key, _)| key.clone()))
    }

    pub(crate) fn get_trg_t_flags(&self, targets: &[Target], ft: TFlagType) -> &TFlags {
        match ft {
            TFlagType::Default => self.get_trg::<MCDefaultTFlags>(targets).inner(),
            TFlagType::Forced => self.get_trg::<MCForcedTFlags>(targets).inner(),
            TFlagType::Enabled => self.get_trg::<MCEnabledTFlags>(targets).inner(),
        }
    }
}

macro_rules! get_fields {
    // trait GetField only
    ($type:ident;
    $( $field:ident, $ty:ty => $marker:ident ),* $(,)?
    ) => {
        $(
            #[doc = concat!("Marker of `MuxConfig` fields, that stores `", stringify!($ty), "`.")]
            pub struct $marker;

            impl $crate::GetField<$marker> for $type {
                type FieldType = $ty;
                fn get(&self) -> &Self::FieldType {
                    &self.$field
                }
            }
        )*
    };

    // trait GetField + trait GetOptField
    ($type:ident, $opt_type:ident;
    $( $field:ident, $ty:ty => $marker:ident ),* $(,)?
    ) => {
        get_fields!($type; $( $field, $ty => $marker ),*);

        $(
            impl $crate::GetOptField<$marker> for $opt_type {
                type FieldType = $ty;
                fn get(&self) -> Option<&Self::FieldType> {
                    self.$field.as_ref()
                }
            }
        )*
    };
}

get_fields! {
    MuxConfig;
    input, Input => MCInput,
    output, Output => MCOutput,
    locale, LangCode => MCLocale,
    verbosity, Verbosity => MCVerbosity,
    no_json, bool => MCNoJson,
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
