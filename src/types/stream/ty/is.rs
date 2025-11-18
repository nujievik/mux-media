use super::StreamType;

impl StreamType {
    /// Returns `true` if stream is Audio.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Audio.is_audio());
    /// assert!(!StreamType::Sub.is_audio());
    /// assert!(!StreamType::Video.is_audio());
    /// assert!(!StreamType::Other.is_audio());
    /// assert!(!StreamType::Font.is_audio());
    /// assert!(!StreamType::Attach.is_audio());
    /// ```
    pub const fn is_audio(&self) -> bool {
        matches!(self, Self::Audio)
    }

    /// Returns `true` if stream is Sub.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Sub.is_sub());
    /// assert!(!StreamType::Audio.is_sub());
    /// assert!(!StreamType::Video.is_sub());
    /// assert!(!StreamType::Other.is_sub());
    /// assert!(!StreamType::Font.is_sub());
    /// assert!(!StreamType::Attach.is_sub());
    /// ```
    pub const fn is_sub(&self) -> bool {
        matches!(self, Self::Sub)
    }

    /// Returns `true` if stream is Video.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Video.is_video());
    /// assert!(!StreamType::Audio.is_video());
    /// assert!(!StreamType::Sub.is_video());
    /// assert!(!StreamType::Other.is_video());
    /// assert!(!StreamType::Font.is_video());
    /// assert!(!StreamType::Attach.is_video());
    /// ```
    pub const fn is_video(&self) -> bool {
        matches!(self, Self::Video)
    }

    /// Returns `true` if stream is Other.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Other.is_other());
    /// assert!(!StreamType::Audio.is_other());
    /// assert!(!StreamType::Sub.is_other());
    /// assert!(!StreamType::Video.is_other());
    /// assert!(!StreamType::Font.is_other());
    /// assert!(!StreamType::Attach.is_other());
    /// ```
    pub const fn is_other(&self) -> bool {
        matches!(self, Self::Other)
    }

    /// Returns `true` if stream is Font.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Font.is_font());
    /// assert!(!StreamType::Audio.is_font());
    /// assert!(!StreamType::Sub.is_font());
    /// assert!(!StreamType::Video.is_font());
    /// assert!(!StreamType::Other.is_font());
    /// assert!(!StreamType::Attach.is_font());
    /// ```
    pub const fn is_font(&self) -> bool {
        matches!(self, Self::Font)
    }

    /// Returns `true` if stream is Attach.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Attach.is_attach());
    /// assert!(!StreamType::Audio.is_attach());
    /// assert!(!StreamType::Sub.is_attach());
    /// assert!(!StreamType::Video.is_attach());
    /// assert!(!StreamType::Other.is_attach());
    /// assert!(!StreamType::Font.is_attach());
    /// ```
    pub const fn is_attach(&self) -> bool {
        matches!(self, Self::Attach)
    }

    /// Returns `true` if stream is Audio, Sub, or Video.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Audio.is_track());
    /// assert!(StreamType::Sub.is_track());
    /// assert!(StreamType::Video.is_track());
    /// assert!(!StreamType::Other.is_track());
    /// assert!(!StreamType::Font.is_track());
    /// assert!(!StreamType::Attach.is_track());
    /// ```
    pub const fn is_track(&self) -> bool {
        matches!(self, Self::Audio | Self::Sub | Self::Video)
    }

    /// Returns `true` if stream is Font or Attach.
    /// ```
    /// use mux_media::StreamType;
    /// assert!(StreamType::Font.is_an_attach());
    /// assert!(StreamType::Attach.is_an_attach());
    /// assert!(!StreamType::Audio.is_an_attach());
    /// assert!(!StreamType::Sub.is_an_attach());
    /// assert!(!StreamType::Video.is_an_attach());
    /// assert!(!StreamType::Other.is_an_attach());
    /// ```
    pub const fn is_an_attach(&self) -> bool {
        matches!(self, Self::Font | Self::Attach)
    }
}
