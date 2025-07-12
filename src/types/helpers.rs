use crate::MuxError;
use std::{
    ffi::{OsStr, OsString},
    fs::{File, canonicalize},
    io::BufWriter,
    path::{Path, PathBuf},
};

#[inline]
pub(crate) fn try_write_args_to_json<I, T>(args: I, json: &Path) -> Result<Vec<String>, MuxError>
where
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
{
    let args = args
        .into_iter()
        .map(|arg| {
            arg.as_ref().to_str().map(|s| s.to_string()).ok_or_else(|| {
                let path = Path::new(arg.as_ref());
                format!("Unsupported UTF-8 symbol in '{}'", path.display()).into()
            })
        })
        .collect::<Result<Vec<String>, MuxError>>()?;

    let file = File::create(json)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &args)?;

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

#[inline(always)]
pub(crate) fn os_str_starts_with(prefix: &OsStr, longer: &OsStr) -> bool {
    longer
        .as_encoded_bytes()
        .starts_with(prefix.as_encoded_bytes())
}

#[inline]
pub(crate) fn os_str_tail(prefix: &OsStr, longer: &OsStr) -> Result<OsString, MuxError> {
    let prefix_b = prefix.as_encoded_bytes();
    let longer_b = longer.as_encoded_bytes();

    if !longer_b.starts_with(prefix_b) {
        return Err(format!("Longer {:?} is not starts with {:?}", longer, prefix).into());
    }

    let prefix_len = prefix_b.len();

    if longer_b.len() == prefix_len {
        return Ok(OsString::new());
    }

    // Safety: `bytes` is a suffix of `longer_b`, which was originally obtained from a valid `OsStr`.
    // Since `prefix_b` is a valid prefix, the remaining bytes (`bytes`) are also guaranteed
    // to form a valid `OsStr` on this platform.
    unsafe {
        let bytes = &longer_b[prefix_len..];
        Ok(OsStr::from_encoded_bytes_unchecked(bytes).into())
    }
}
