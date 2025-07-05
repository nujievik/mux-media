use mux_media::*;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub const MAX_U64_STR: &str = "18446744073709551615";
const TEST_DATA: &str = "tests/test_data";

pub fn data_dir() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push(TEST_DATA);
    dir
}

pub fn data_file(file: &str) -> PathBuf {
    let mut path = data_dir();
    path.push(file);
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
    S: ToString,
{
    args.into_iter().map(|s| s.to_string()).collect()
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
    S: ToString,
{
    vecs.into_iter().flatten().map(|s| s.to_string()).collect()
}
