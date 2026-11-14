#![allow(unused)]

use clap::Parser;
use mux_media::*;
use std::{
    ffi::{OsStr, OsString},
    path::{MAIN_SEPARATOR, Path, PathBuf},
    sync::LazyLock,
};

pub fn p<OS: AsRef<OsStr> + ?Sized>(oss: &OS) -> &Path {
    Path::new(oss.as_ref())
}

pub fn data(add: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    path_with_add(p, add)
}

pub fn temp(add: &str) -> PathBuf {
    let p = data("temp");
    path_with_add(p, add)
}

fn path_with_add(mut p: PathBuf, add: &str) -> PathBuf {
    for s in add.split('/') {
        p.push(s);
    }
    extended_length_path(p)
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| cfg::<_, &str>([]));

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
            let dir = $crate::common::temp(&format!("to_args/{}", $txt_dir));

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

                // test that config with --target global is eq config without target
                let trg_args = $crate::common::append_str_vecs([add_args.clone(), vec!["-t", "global"], $args]);
                assert_eq!(&$crate::common::cfg(trg_args), &cfg);

                let double_trg_args = $crate::common::append_str_vecs([
                    add_args.clone(),
                    vec!["-t", "global"],
                    vec!["-t", "global"],
                    $args
                ]);
                assert_eq!($crate::common::cfg(double_trg_args), cfg);
            )*
        }
    };
}
