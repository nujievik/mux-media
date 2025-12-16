use super::Extension;
use std::path::Path;

impl Extension {
    pub fn new(bytes: &[u8]) -> Option<Extension> {
        new_from_bytes(bytes)
    }

    pub(crate) fn new_from_path(path: impl AsRef<Path>) -> Option<Extension> {
        let ext = path.as_ref().extension()?;
        new_from_bytes(ext.as_encoded_bytes())
    }
}

fn new_from_bytes(bytes: &[u8]) -> Option<Extension> {
    let len = bytes.len();
    if !matches!(len, 2 | 3 | 4 | 5 | 6) {
        return None;
    }

    let mut buf = [0u8; 6];
    for (dst, src) in buf[..len].iter_mut().zip(bytes.iter()) {
        *dst = src.to_ascii_lowercase();
    }

    let ext = match &buf[..len] {
        b"264" => Extension::Ext264,
        b"265" => Extension::Ext265,
        b"3gp" => Extension::Ext3gp,
        b"aac" => Extension::Aac,
        b"ac3" => Extension::Ac3,
        b"ass" => Extension::Ass,
        b"av1" => Extension::Av1,
        b"avc" => Extension::Avc,
        b"avi" => Extension::Avi,
        b"caf" => Extension::Caf,
        b"dts" => Extension::Dts,
        b"dtshd" => Extension::Dtshd,
        b"eac3" => Extension::Eac3,
        b"ec3" => Extension::Ec3,
        b"f4v" => Extension::F4v,
        b"flac" => Extension::Flac,
        b"flv" => Extension::Flv,
        b"h264" => Extension::H264,
        b"h265" => Extension::H265,
        b"hevc" => Extension::Hevc,
        b"ivf" => Extension::Ivf,
        b"m2ts" => Extension::M2ts,
        b"m2v" => Extension::M2v,
        b"m4a" => Extension::M4a,
        b"m4v" => Extension::M4v,
        b"mka" => Extension::Mka,
        b"mks" => Extension::Mks,
        b"mkv" => Extension::Mkv,
        b"mlp" => Extension::Mlp,
        b"mov" => Extension::Mov,
        b"mp2" => Extension::Mp2,
        b"mp3" => Extension::Mp3,
        b"mp4" => Extension::Mp4,
        b"mpa" => Extension::Mpa,
        b"mpeg" => Extension::Mpeg,
        b"mpg" => Extension::Mpg,
        b"mpv" => Extension::Mpv,
        b"obu" => Extension::Obu,
        b"ogg" => Extension::Ogg,
        b"ogm" => Extension::Ogm,
        b"ogv" => Extension::Ogv,
        b"opus" => Extension::Opus,
        b"otf" => Extension::Otf,
        b"ra" => Extension::Ra,
        b"srt" => Extension::Srt,
        b"ssa" => Extension::Ssa,
        b"sub" => Extension::Sub,
        b"sup" => Extension::Sup,
        b"thd" => Extension::Thd,
        b"truehd" => Extension::Truehd,
        b"ts" => Extension::Ts,
        b"tta" => Extension::Tta,
        b"ttf" => Extension::Ttf,
        b"vc1" => Extension::Vc1,
        b"vtt" => Extension::Vtt,
        b"wav" => Extension::Wav,
        b"weba" => Extension::Weba,
        b"webm" => Extension::Webm,
        b"webma" => Extension::Webma,
        b"wma" => Extension::Wma,
        b"wmv" => Extension::Wmv,
        b"x264" => Extension::X264,
        b"x265" => Extension::X265,
        _ => return None,
    };

    Some(ext)
}
