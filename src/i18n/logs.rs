use crate::{Msg, MuxError, display};
use log::{debug, warn};
use std::{ffi::OsStr, path::Path};

#[inline(always)]
pub(crate) fn warn_file_is_already_exists(path: &Path) {
    warn!(
        "{}. {} '{}'",
        Msg::FileIsAlreadyExists,
        Msg::Skipping,
        display(path)
    )
}

#[inline(always)]
pub(crate) fn warn_no_ext_media(stem: &OsStr) {
    warn!(
        "{}. {} '{}'",
        Msg::NoExternalMediaFound,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display()
    )
}

#[inline(always)]
pub(crate) fn warn_not_out_save_any(out: &Path) {
    warn!(
        "{} '{}'. {}",
        Msg::NotOutSaveAny,
        display(out),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn warn_not_recognized_media(path: &Path, e: MuxError) {
    warn!(
        "{} '{}': {}. {}",
        Msg::NotRecognizedMedia,
        display(path),
        e,
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
        Msg::MediaNumberIsOutOfRange,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}
