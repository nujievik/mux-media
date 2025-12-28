use crate::{Container, Msg, MuxError};
use log::{debug, warn};
use std::{ffi::OsStr, path::Path};

pub(crate) fn warn_container_does_not_support(cont: Container, src: &Path, i_stream: usize) {
    warn!(
        "{} {} {}. {} '{}' stream {}",
        cont,
        Msg::ContainerDoesNotSupport,
        Msg::LMultipleTracksOrTypeTrack,
        Msg::Skipping,
        src.display(),
        i_stream
    );
}

#[inline(always)]
pub(crate) fn warn_file_is_already_exists(path: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::FileIsAlreadyExists,
        Msg::Skipping,
        path.display()
    )
}

#[inline(always)]
pub(crate) fn warn_no_ext_media(stem: &OsStr) {
    warn!(
        "{}. {} '{}'",
        Msg::NoExtMediaFound,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display()
    )
}

#[inline(always)]
pub(crate) fn warn_not_out_save_any(out: &Path) {
    warn!(
        "{} '{}'. {}",
        Msg::NotOutSaveAny,
        out.display(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn warn_not_recognized_media(path: &Path, e: MuxError) {
    warn!(
        "{} '{}': {}. {}",
        Msg::NotRecognizedMedia,
        path.display(),
        e.as_str_localized(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn debug_found_repeat(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::FoundRepeat,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}

pub(crate) fn debug_media_out_of_range(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::MediaNumOutOfRange,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}
