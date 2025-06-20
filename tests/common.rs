use mux_media::*;
use std::collections::HashMap;
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
    T: Into<OsString>,
{
    let args: Vec<OsString> = args.into_iter().map(|x| x.into()).collect();
    RawMuxConfig::try_from(args).unwrap().try_into().unwrap()
}

pub fn from_cfg<F>(args: Vec<&str>) -> <MuxConfig as GetField<F>>::FieldType
where
    MuxConfig: GetField<F>,
    <MuxConfig as GetField<F>>::FieldType: Clone,
{
    cfg(args).get::<F>().clone()
}

pub fn to_args(args: Vec<&str>) -> Vec<String> {
    args.into_iter().map(|s| s.to_string()).collect()
}

pub fn cfg_args<F>(args: Vec<String>, path: &Path, cache: HashMap<PathBuf, MICache>) -> Vec<String>
where
    MuxConfig: GetField<F>,
    <MuxConfig as GetField<F>>::FieldType: ToMkvmergeArgs,
{
    let mc = cfg(args);
    let mut mi = MediaInfo::from(&mc);
    mi.upd_cache(cache);
    mc.get::<F>().to_mkvmerge_args(&mut mi, path)
}
