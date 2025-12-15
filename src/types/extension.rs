mod is;
mod new;

use strum_macros::{AsRefStr, EnumIter};

/// A supported file extension.
#[derive(Copy, Clone, Debug, PartialEq, AsRefStr, EnumIter)]
#[non_exhaustive]
#[strum(serialize_all = "kebab-case")]
pub enum Extension {
    /// The `264` extension.
    #[strum(serialize = "264")]
    Ext264,
    /// The `265` extension.
    #[strum(serialize = "265")]
    Ext265,
    /// The `3gp` extension.
    #[strum(serialize = "3gp")]
    Ext3gp,
    Aac,
    Ac3,
    Ass,
    Av1,
    Avc,
    Avi,
    Caf,
    Dts,
    Dtshd,
    Eac3,
    Ec3,
    F4v,
    Flac,
    Flv,
    H264,
    H265,
    Hevc,
    Ivf,
    M2ts,
    M2v,
    M4a,
    M4v,
    Mka,
    Mks,
    Mkv,
    Mlp,
    Mov,
    Mp2,
    Mp3,
    Mp4,
    Mpa,
    Mpeg,
    Mpg,
    Mpv,
    Obu,
    Ogg,
    Ogm,
    Ogv,
    Opus,
    Otf,
    Ra,
    Srt,
    Ssa,
    Sub,
    Sup,
    Thd,
    Truehd,
    Ts,
    Tta,
    Ttf,
    Vc1,
    Vtt,
    Wav,
    Weba,
    Webm,
    Webma,
    Wma,
    Wmv,
    X264,
    X265,
}
