use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, AutoFlags, ButtonTracks, Chapters, DefaultTFlags, EnabledTFlags, Field,
    FontAttachs, ForcedTFlags, Input, LangCode, OtherAttachs, Output, Retiming, Specials,
    SubTracks, TFlagType, TFlags, Target, Tools, TrackLangs, TrackNames, Verbosity, VideoTracks,
};
use std::path::Path;

impl MuxConfig {
    /// Returns the field value for marker `F`.
    pub fn field<F>(&self) -> &<Self as Field<F>>::FieldType
    where
        Self: Field<F>,
    {
        <Self as Field<F>>::field(self)
    }

    /// Returns a field value for marker `F`:
    ///
    ///  - From the first [`Target`] with a value.
    ///  - Otherwise, from [`Self::field`].
    pub fn trg_field<F>(&self, targets: &[Target]) -> &<Self as Field<F>>::FieldType
    where
        Self: Field<F>,
        MuxConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
    {
        self.targets
            .as_ref()
            .and_then(|map| {
                targets.into_iter().find_map(|trg| {
                    map.get(trg).and_then(|trg_vals| {
                        <MuxConfigTarget as Field<F>>::field(trg_vals).as_ref()
                    })
                })
            })
            .unwrap_or_else(|| self.field::<F>())
    }

    /// Gets a cloned [`Target`] if its exists in [`Self`].
    ///
    /// This operation avoids heap allocation: internally it either copies an enum variant
    /// or increments the [`Arc`](std::sync::Arc) reference count.
    pub fn get_clone_target(&self, path: impl AsRef<Path>) -> Option<Target> {
        self.targets
            .as_ref()
            .and_then(|map| map.get_key_value(path.as_ref()).map(|(key, _)| key.clone()))
    }

    pub(crate) fn trg_field_t_flags(&self, targets: &[Target], ft: TFlagType) -> &TFlags {
        match ft {
            TFlagType::Default => self.trg_field::<MCDefaultTFlags>(targets).inner(),
            TFlagType::Forced => self.trg_field::<MCForcedTFlags>(targets).inner(),
            TFlagType::Enabled => self.trg_field::<MCEnabledTFlags>(targets).inner(),
        }
    }
}

macro_rules! fields {
    // Only base type: defines marker + Field impl
    ($type:ident;
    $( $field:ident, $ty:ty => $marker:ident ),* $(,)?
    ) => {
        $(
            #[doc = concat!("Marker of [`MuxConfig`] fields, that stores [`", stringify!($ty), "`].")]
            pub struct $marker;

            impl Field<$marker> for $type {
                type FieldType = $ty;

                #[inline(always)]
                fn field(&self) -> &Self::FieldType {
                    &self.$field
                }
            }
        )*
    };

    // Base type and optional (target) type
    ($type:ident, $opt_type:ident;
    $( $field:ident, $ty:ty => $marker:ident ),* $(,)?
    ) => {
        fields!($type; $( $field, $ty => $marker ),*);

        $(
            impl Field<$marker> for $opt_type {
                type FieldType = Option<$ty>;

                #[inline(always)]
                fn field(&self) -> &Self::FieldType {
                    &self.$field
                }
            }
        )*
    };
}

fields! {
    MuxConfig;
    input, Input => MCInput,
    output, Output => MCOutput,
    locale, LangCode => MCLocale,
    verbosity, Verbosity => MCVerbosity,
    json, bool => MCJson,
    exit_on_err, bool => MCExitOnErr,
    auto_flags, AutoFlags => MCAutoFlags,
    retiming, Retiming => MCRetiming,
    tools, Tools => MCTools,
}

fields! {
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
