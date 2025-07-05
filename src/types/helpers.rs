use crate::MuxError;
use std::{
    ffi::{OsStr, OsString},
    fs::{File, canonicalize},
    io::BufWriter,
    path::{Path, PathBuf},
};

#[inline]
pub(crate) fn try_write_args_to_json<I, T>(args: I, json: &Path) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = T> + Clone,
    T: AsRef<OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| {
            arg.as_ref()
                .to_str()
                .ok_or("Unsupported UTF-8 symbol.".to_string())
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let file = File::create(json).map_err(|e| format!("Create error: {}", e))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &args).map_err(|e| format!("JSON write error: {}", e))?;

    Ok(args)
}

#[inline(always)]
pub(crate) fn try_canonicalize_and_open(path: impl AsRef<Path>) -> Result<PathBuf, MuxError> {
    let path = canonicalize(path)?;
    if !path.is_file() {
        return Err("Is not a file".into());
    }
    File::open(&path)?;
    Ok(path)
}

#[inline]
pub(crate) fn os_str_starts_with(prefix: &OsStr, longer: &OsStr) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        longer.as_bytes().starts_with(prefix.as_bytes())
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        longer
            .encode_wide()
            .zip(prefix.encode_wide())
            .all(|(a, b)| a == b)
    }
}

#[inline]
pub(crate) fn os_str_tail(prefix: &OsStr, longer: &OsStr) -> Result<OsString, MuxError> {
    if !os_str_starts_with(prefix, longer) {
        return Err(format!("Longer {:?} is not starts with {:?}", longer, prefix).into());
    }

    #[cfg(unix)]
    {
        use std::os::unix::ffi::{OsStrExt, OsStringExt};
        let full_bytes = longer.as_bytes();
        let prefix_bytes = prefix.as_bytes();
        Ok(OsString::from_vec(
            full_bytes[prefix_bytes.len()..].to_vec(),
        ))
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        let full_wide: Vec<u16> = longer.encode_wide().collect();
        let prefix_wide: Vec<u16> = prefix.encode_wide().collect();
        Ok(OsString::from_wide(&full_wide[prefix_wide.len()..]))
    }
}
