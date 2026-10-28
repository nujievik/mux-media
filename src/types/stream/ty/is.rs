use super::StreamType;

macro_rules! is_any_single {
    ($fn:ident, $ty:ident) => {
        #[doc = concat!("Return `true` if stream is `", stringify!($ty), "`.")]
        /// ```
        /// use mux_media::StreamType;
        /// use strum::IntoEnumIterator;
        ///
        #[doc = concat!("assert!(StreamType::", stringify!($ty), ".", stringify!($fn), "());")]
        ///
        #[doc = concat!("for ty in StreamType::iter().filter(|ty| !matches!(ty, StreamType::", stringify!($ty), ")) {")]
        #[doc = concat!("    assert!(!ty.", stringify!($fn), "());")]
        /// }
        /// ```
        pub const fn $fn(&self) -> bool {
            matches!(self, Self::$ty)
        }
    };
}

impl StreamType {
    is_any_single!(is_video, Video);
    is_any_single!(is_audio, Audio);
    is_any_single!(is_subtitles, Subtitles);
    is_any_single!(is_attached_font, AttachedFont);
    is_any_single!(is_attached_pic, AttachedPic);

    pub(crate) const fn is_attached(&self) -> bool {
        matches!(
            self,
            Self::AttachedFont | Self::AttachedPic | Self::AttachedOther
        )
    }
}
