use clap::Parser;
use mux_media::*;
use std::{
    ffi::{OsStr, OsString},
    path::{MAIN_SEPARATOR, Path, PathBuf},
};

pub fn p<OS: AsRef<OsStr> + ?Sized>(oss: &OS) -> &Path {
    Path::new(oss.as_ref())
}

pub fn new_dir(subdir: impl AsRef<OsStr>) -> PathBuf {
    let subdir = ensure_platform_seps(subdir);
    let sep = has_trailing_sep(&subdir);

    let mut dir = std::env::current_dir().unwrap();
    dir.push(subdir);

    if sep {
        dir = ensure_trailing_sep(dir);
    }

    ensure_long_path_prefix(dir)
}

pub fn data(add: impl AsRef<OsStr>) -> PathBuf {
    let add = ensure_platform_seps(add);
    let sep = has_trailing_sep(&add);

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_data")
        .join(add);

    let mut path = ensure_long_path_prefix(path);

    if sep {
        path = ensure_trailing_sep(path);
    }

    path
}

pub fn cfg<I, OS>(args: I) -> Config
where
    I: IntoIterator<Item = OS>,
    OS: Into<OsString> + Clone,
{
    Config::try_parse_from(args).unwrap()
}

pub fn to_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter().map(|s| s_sep(s.as_ref())).collect()
}

pub fn to_os_args<I, OS>(args: I) -> Vec<OsString>
where
    I: IntoIterator<Item = OS>,
    OS: AsRef<OsStr>,
{
    args.into_iter()
        .map(|oss| ensure_platform_seps(oss).into_os_string())
        .collect()
}

pub fn append_str_vecs<I, S>(vecs: I) -> Vec<String>
where
    I: IntoIterator<Item = Vec<S>>,
    S: AsRef<str>,
{
    vecs.into_iter()
        .flatten()
        .map(|s| s_sep(s.as_ref()))
        .collect()
}

pub fn read_json_args(path: &Path) -> Vec<String> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn iter_i_lang() -> impl Iterator<Item = (&'static usize, &'static LangCode)> {
    static I: [usize; 4] = [0, 1, 8, usize::MAX - 1];
    static LANGS: [LangCode; 3] = [LangCode::Eng, LangCode::Rus, LangCode::Und];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

pub fn iter_alt_i_lang() -> impl Iterator<Item = (&'static usize, &'static LangCode)> {
    static I: [usize; 4] = [5, 10, 11, usize::MAX - 2];
    static LANGS: [LangCode; 3] = [LangCode::Abk, LangCode::Aar, LangCode::Afr];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

#[cfg(unix)]
const ALT_SEP_BYTE: u8 = b'\\';
#[cfg(windows)]
const ALT_SEP_BYTE: u8 = b'/';

const ALT_SEP_STR: &str = unsafe { str::from_utf8_unchecked(&[ALT_SEP_BYTE]) };

pub fn s_sep(s: &str) -> String {
    s.replace(ALT_SEP_STR, SEP_STR)
}

fn ensure_platform_seps(oss: impl AsRef<OsStr>) -> PathBuf {
    const SEP_BYTE: u8 = MAIN_SEPARATOR as u8;

    let bytes: Vec<u8> = oss
        .as_ref()
        .as_encoded_bytes()
        .into_iter()
        .map(|&b| if b == ALT_SEP_BYTE { SEP_BYTE } else { b })
        .collect();

    let oss = unsafe { OsString::from_encoded_bytes_unchecked(bytes) };

    PathBuf::from(oss)
}

fn has_trailing_sep(oss: impl AsRef<OsStr>) -> bool {
    let bytes = oss.as_ref().as_encoded_bytes();
    bytes.ends_with(SEP_BYTES) || bytes.ends_with(&[ALT_SEP_BYTE])
}
