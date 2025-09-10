use super::{MuxConfig, MuxConfigTarget};
use crate::{
    AudioTracks, Chapters, DefaultTrackFlags, EnabledTrackFlags, Field, FontAttachs,
    ForcedTrackFlags, OtherAttachs, Specials, SubTracks, Target, TrackFlagType, TrackFlags,
    TrackLangs, TrackNames, VideoTracks,
};
use std::path::Path;

impl MuxConfig {
    /// Returns a field value for marker `F`:
    ///
    /// - From the first given target contains in [`MuxConfig::targets`], that has a Some value.
    ///
    /// - Otherwise, from the common (global) configuration.
    pub fn target<F, I, K>(&self, _: F, targets: I) -> &<Self as Field<F>>::FieldType
    where
        Self: Field<F>,
        MuxConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
        I: IntoIterator<Item = K>,
        K: AsRef<Path>,
    {
        self.targets
            .as_ref()
            .and_then(|map| {
                targets.into_iter().find_map(|trg| {
                    map.get(trg.as_ref())
                        .and_then(|vals| <MuxConfigTarget as Field<F>>::field(vals).as_ref())
                })
            })
            .unwrap_or_else(|| <Self as Field<F>>::field(self))
    }

    /// Gets a cloned [`Target`] key if its exists in [`Self::targets`].
    ///
    /// This operation avoids heap allocation: internally it either copies an enum variant
    /// or increments the [`Arc`](std::sync::Arc) reference count.
    pub(crate) fn get_key(&self, target: impl AsRef<Path>) -> Option<Target> {
        self.targets.as_ref().and_then(|map| {
            map.get_key_value(target.as_ref())
                .map(|(key, _)| key.clone())
        })
    }

    pub(crate) fn target_track_flags(&self, targets: &[Target], ty: TrackFlagType) -> &TrackFlags {
        match ty {
            TrackFlagType::Default => &self.target(MCDefaultTrackFlags, targets).0,
            TrackFlagType::Forced => &self.target(MCForcedTrackFlags, targets).0,
            TrackFlagType::Enabled => &self.target(MCEnabledTrackFlags, targets).0,
        }
    }
}

macro_rules! fields {
    // Base type and optional (target) type
    ($type:ident, $opt_type:ident;
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
    MuxConfig, MuxConfigTarget;
    audio_tracks, AudioTracks => MCAudioTracks,
    sub_tracks, SubTracks => MCSubTracks,
    video_tracks, VideoTracks => MCVideoTracks,
    chapters, Chapters => MCChapters,
    font_attachs, FontAttachs => MCFontAttachs,
    other_attachs, OtherAttachs => MCOtherAttachs,
    default_track_flags, DefaultTrackFlags => MCDefaultTrackFlags,
    forced_track_flags, ForcedTrackFlags => MCForcedTrackFlags,
    enabled_track_flags, EnabledTrackFlags => MCEnabledTrackFlags,
    track_names, TrackNames => MCTrackNames,
    track_langs, TrackLangs => MCTrackLangs,
    specials, Specials => MCSpecials,
}
