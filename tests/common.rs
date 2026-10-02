#![allow(unused)]

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
        .join("data")
        .join(add);

    let mut path = ensure_long_path_prefix(path);

    if sep {
        path = ensure_trailing_sep(path);
    }

    path
}

pub fn temp(add: impl AsRef<OsStr>) -> PathBuf {
    let mut p = OsString::from("temp/");
    p.push(add);
    data(p)
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

pub fn append_str_vecs<I, T, S>(vecs: I) -> Vec<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<[S]>,
    S: AsRef<str>,
{
    let mut out = Vec::new();

    for vec in vecs {
        for s in vec.as_ref() {
            out.push(s_sep(s.as_ref()));
        }
    }

    out
}

pub fn read_txt_args(path: &Path) -> Vec<String> {
    use std::io::BufRead;
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().collect::<std::io::Result<Vec<_>>>().unwrap()
}

pub fn iter_i_lang() -> impl Iterator<Item = (&'static usize, &'static Lang)> {
    static I: [usize; 4] = [0, 1, 8, usize::MAX - 1];
    static LANGS: [Lang; 3] = [lang!(Eng), lang!(Rus), lang!(Und)];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

pub fn iter_alt_i_lang() -> impl Iterator<Item = (&'static usize, &'static Lang)> {
    static I: [usize; 4] = [5, 10, 11, usize::MAX - 2];
    static LANGS: [Lang; 3] = [lang!(Abk), lang!(Aar), lang!(Afr)];

    I.iter()
        .flat_map(|i| LANGS.iter().map(move |lang| (i, lang)))
}

const SEP_BYTE: u8 = MAIN_SEPARATOR as u8;

#[cfg(unix)]
const ALT_SEP_BYTE: u8 = b'\\';
#[cfg(windows)]
const ALT_SEP_BYTE: u8 = b'/';

const SEP_STR: &str = unsafe { str::from_utf8_unchecked(&[SEP_BYTE]) };

pub fn s_sep(s: &str) -> String {
    const ALT_SEP_STR: &str = unsafe { str::from_utf8_unchecked(&[ALT_SEP_BYTE]) };

    s.replace(ALT_SEP_STR, SEP_STR)
}

fn ensure_platform_seps(oss: impl AsRef<OsStr>) -> PathBuf {
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
    bytes.ends_with(&[SEP_BYTE]) || bytes.ends_with(&[ALT_SEP_BYTE])
}

fn ensure_trailing_sep(path: impl Into<PathBuf>) -> PathBuf {
    let path = path.into();

    if path.as_os_str().as_encoded_bytes().ends_with(&[SEP_BYTE]) {
        return path;
    }

    let mut path = path.into_os_string();
    path.push(SEP_STR);
    path.into()
}

#[macro_export]
macro_rules! test_from_str {
    ($type:ty, $err_cases:expr, @err) => {{
        for s in $err_cases {
            assert!(s.parse::<$type>().is_err(), "Fail is_err() parse '{}'", s);
        }
    }};

    ($test_fn:ident, $type:ty; $cases:expr; $err_cases:expr) => {
        #[test]
        fn $test_fn() {
            for s in $cases {
                assert!(s.parse::<$type>().is_ok(), "Fail is_ok() parse '{}'", s);
            }

            $crate::test_from_str!($type, $err_cases, @err);
        }
    };

    ($test_fn:ident, $type:ty; $cases:expr; $err_cases:expr, @ok_compare) => {
        #[test]
        fn $test_fn() {
            for (exp, s) in $cases {
                assert!(exp == s.parse::<$type>().unwrap(), "Fail == parse '{}'", s);
            }

            $crate::test_from_str!($type, $err_cases, @err);
        }
    };
}

#[macro_export]
macro_rules! build_test_to_args {
    ( $fn:ident, $txt_dir:expr; $( $args:expr ),* $(,)? ) => {
        #[test]
        fn $fn() {
            let dir = std::path::Path::new("to_args").join($txt_dir);
            let dir = $crate::common::temp(&dir);

            let in_dir = dir.to_str().unwrap();
            let out_dir = dir.join("muxed").to_str().unwrap().to_string();

            let _ = std::fs::remove_dir_all(&dir);
            let _ = std::fs::create_dir_all(&dir);

            let add_args = vec!["--locale", "eng", "--input", in_dir, "--output", &out_dir, "--save-config"];
            let txt = dir.clone().join(".mux-media").join("config.txt");

            $(
                let cfg_args = $crate::common::append_str_vecs([add_args.clone(), $args.clone()]);
                let cfg = $crate::common::cfg(cfg_args);
                let left = $crate::common::append_str_vecs([&add_args[..add_args.len() - 1], $args.as_slice()]);

                assert_eq!(&left, &cfg.to_args().unwrap(), "from config struct err");

                cfg.try_save_config().unwrap();
                assert_eq!(left, $crate::common::read_txt_args(&txt), "from txt err");
            )*
        }
    };
}
