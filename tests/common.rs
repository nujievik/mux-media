use mux_media::*;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub const MAX_U64_STR: &'static str = "18446744073709551615";

#[cfg(unix)]
const SEP_B: &[u8] = b"/";
#[cfg(windows)]
const SEP_B: &[u8] = b"\\";

#[cfg(unix)]
const ALT_SEP_B: &[u8] = b"\\";
#[cfg(windows)]
const ALT_SEP_B: &[u8] = b"/";

pub fn s_sep(s: impl AsRef<str>) -> String {
    #[cfg(unix)]
    {
        s.as_ref().replace('\\', "/")
    }
    #[cfg(windows)]
    {
        s.as_ref().replace('/', "\\")
    }
}

pub fn new_dir(subdir: impl AsRef<OsStr>) -> PathBuf {
    let sep_end = is_sep_end(&subdir);

    let mut dir = std::env::current_dir().unwrap();
    dir.push(subdir.as_ref());

    if sep_end {
        dir = ensure_ends_sep(dir);
    }

    #[cfg(windows)]
    {
        let mut prf_dir = OsString::from(r"\\?\");
        prf_dir.push(dir.as_os_str());
        dir = prf_dir.into();
    }

    dir
}

pub fn data_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_data");

    new_dir(dir)
}

pub fn data_file(file: impl AsRef<OsStr>) -> PathBuf {
    let sep_end = is_sep_end(&file);

    let mut path = data_dir();
    path.push(file.as_ref());

    if sep_end {
        path = ensure_ends_sep(path);
    }

    path
}

pub fn cfg<I, T>(args: I) -> MuxConfig
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    MuxConfig::try_from_args(args).unwrap()
}

pub fn from_cfg<F>(args: Vec<&str>) -> <MuxConfig as GetField<F>>::FieldType
where
    MuxConfig: GetField<F>,
    <MuxConfig as GetField<F>>::FieldType: Clone,
{
    cfg(args).get::<F>().clone()
}

pub fn to_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    args.into_iter().map(|s| s_sep(s)).collect()
}

pub fn cfg_args<F>(args: Vec<String>, path: &Path, cache: CacheMI) -> Vec<String>
where
    MuxConfig: GetField<F>,
    <MuxConfig as GetField<F>>::FieldType: ToMkvmergeArgs,
{
    let mc = cfg(args);
    let mut mi = MediaInfo::from(&mc);
    mi.upd_cache(cache);
    mc.get::<F>().to_mkvmerge_args(&mut mi, path)
}

pub fn repeat_track_arg(arg: &str, val: &str, range: &str) -> Vec<String> {
    range
        .parse::<Range<u8>>()
        .unwrap()
        .iter()
        .map(|n| [arg.to_string(), format!("{}{}", n, val)])
        .flatten()
        .collect()
}

pub fn append_str_vecs<I, S>(vecs: I) -> Vec<String>
where
    I: IntoIterator<Item = Vec<S>>,
    S: AsRef<str>,
{
    vecs.into_iter().flatten().map(|s| s_sep(s)).collect()
}

pub fn read_json_args(path: &Path) -> Vec<String> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn is_sep_end(oss: impl AsRef<OsStr>) -> bool {
    let bytes = oss.as_ref().as_encoded_bytes();
    bytes.ends_with(SEP_B) || bytes.ends_with(ALT_SEP_B)
}

fn ensure_ends_sep(path: PathBuf) -> PathBuf {
    match path.as_os_str().as_encoded_bytes().ends_with(SEP_B) {
        true => path,
        false => {
            let mut path_sep = path.into_os_string();
            path_sep.push(s_sep("/"));
            path_sep.into()
        }
    }
}
