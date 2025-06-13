use crate::MuxError;
use std::ffi::{OsStr, OsString};

#[inline]
pub fn os_str_starts_with(prefix: &OsStr, longer: &OsStr) -> bool {
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
pub fn os_str_tail(prefix: &OsStr, longer: &OsStr) -> Result<OsString, MuxError> {
    if !os_str_starts_with(prefix, longer) {
        return Err(format!("Longer {:?} is not starts with {:?}", prefix, longer).into());
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
