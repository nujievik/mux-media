use super::Msg;
use log::{debug, trace, warn};
use std::{ffi::OsStr, path::Path};

#[inline(always)]
pub(crate) fn warn_no_input_dir_media(dir: &Path, stem: &OsStr) {
    warn!(
        "{} '{}'. {} '{}'",
        Msg::NoInputDirMedia,
        dir.display(),
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}

#[inline(always)]
pub(crate) fn warn_file_is_already_exists(out: &Path) {
    warn!(
        "{} '{}' {}. {}",
        Msg::File,
        out.display(),
        Msg::IsAlreadyExists,
        Msg::Skipping
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
pub(crate) fn warn_not_out_change(out: &Path) {
    warn!(
        "{} '{}'. {}",
        Msg::NotOutChange,
        out.display(),
        Msg::Skipping
    )
}

#[inline(always)]
pub(crate) fn debug_no_ext_media(stem: &OsStr) {
    debug!(
        "{}. {} '{}'",
        Msg::NoExtMediaFound,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display()
    )
}

#[inline(always)]
pub(crate) fn trace_found_repeat(stem: &OsStr) {
    trace!(
        "{}. {} '{}'",
        Msg::FoundRepeat,
        Msg::Skipping,
        AsRef::<Path>::as_ref(stem).display(),
    )
}
