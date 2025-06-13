use crate::common::{cfg, data_file};
use crate::{compare_arg_cases, test_cli_args};
use mux_media::*;

fn new_fonts(args: &[&str]) -> FontAttachs {
    let mc = cfg::<_, &str>(args.iter().copied());
    mc.get::<MCFontAttachs>().clone()
}

fn new_other(args: &[&str]) -> OtherAttachs {
    let mc = cfg::<_, &str>(args.iter().copied());
    mc.get::<MCOtherAttachs>().clone()
}

#[test]
fn test_cli_args() {
    test_cli_args!(Attachs; Attachs => "", "-m", NoAttachs => "", "-M");
    test_cli_args!(FontAttachs; Fonts => "fonts", NoFonts => "no-fonts");
    test_cli_args!(OtherAttachs; Attachs => "attachs", NoAttachs => "no-attachs");
}

#[test]
fn test_is_default() {
    assert!(FontAttachs::default().is_default());
    assert!(new_fonts(&[]).is_default());
    assert!(!new_fonts(&["--no-fonts"]).is_default());

    assert!(OtherAttachs::default().is_default());
    assert!(new_other(&[]).is_default());
    assert!(!new_other(&["--no-attachs"]).is_default());
}

#[test]
fn test_at_iter() {
    let ats: Vec<AttachType> = AttachType::iter().collect();
    assert!(ats.len() == 2);

    let mut has_font = false;
    let mut has_other = false;
    for at in ats {
        match at {
            AttachType::Font => has_font = true,
            AttachType::Other => has_other = true,
        }
    }
    assert!(has_font);
    assert!(has_other);
}

#[test]
fn test_at_as_str_mkvtoolnix() {
    assert_eq!("font", AttachType::Font.as_str_mkvtoolnix());
    assert_eq!("", AttachType::Other.as_str_mkvtoolnix());
}

fn id_range(s: &str) -> AttachID {
    AttachID::Range(s.parse::<Range<u32>>().unwrap())
}

fn id(u: u32) -> AttachID {
    AttachID::U32(u)
}

#[test]
fn test_id_to_mkvmerge_arg() {
    let cases = [
        ("1", id(1)),
        ("16", id(16)),
        ("1,2", id_range("1-2")),
        ("4,5,6,7,8", id_range("4-8")),
    ];

    for (expected, id) in cases {
        assert_eq!(expected, &id.to_mkvmerge_arg());
    }
}

#[test]
fn test_id_contains() {
    assert!(id(1).contains(id(1)));
    assert!(!id(1).contains(id(2)));

    let id_rng = id_range("1-");
    assert!(id_rng.contains(id(1)));
    assert!(id_rng.contains(id(2)));
    assert!(id_rng.contains(id(u32::MAX)));
    assert!(id_rng.contains(id_range("1-")));

    let id_rng = id_range("2-16");
    assert!(!id_rng.contains(id(1)));
    assert!(id_rng.contains(id(2)));
    assert!(id_rng.contains(id(16)));
    assert!(!id_rng.contains(id(u32::MAX)));
    assert!(!id_rng.contains(id_range("1-")));
    assert!(!id_rng.contains(id_range("1-8")));
    assert!(id_rng.contains(id_range("2-8")));
}

#[test]
fn test_id_from_str() {
    let cases = [
        (id(1), "1"),
        (id(16), "16"),
        (id_range("1-"), "1-"),
        (id_range("1-8"), "1-8"),
    ];
    for (id, s) in cases {
        assert!(id == s.parse::<AttachID>().unwrap());
    }

    let bad_cases = ["0", "2-1", "", "x", "eng"];
    for s in bad_cases {
        assert!(s.parse::<AttachID>().is_err());
    }
}

fn try_fonts_str(s: &str) -> Result<FontAttachs, MuxError> {
    s.parse::<FontAttachs>()
}

fn try_other_str(s: &str) -> Result<OtherAttachs, MuxError> {
    s.parse::<OtherAttachs>()
}

fn fonts_str(s: &str) -> FontAttachs {
    try_fonts_str(s).unwrap()
}

fn other_str(s: &str) -> OtherAttachs {
    try_other_str(s).unwrap()
}

macro_rules! test_from_str {
    ($test_name:ident, $try_from_str:ident) => {
        #[test]
        fn $test_name() {
            let cases = ["1-", "1", "1-1", "!1", "1,3,4"];
            cases
                .into_iter()
                .for_each(|s| assert!($try_from_str(s).is_ok()));

            let bad_cases = ["0", "2-1", "", "x", "eng"];
            bad_cases
                .into_iter()
                .for_each(|s| assert!($try_from_str(s).is_err()));
        }
    };
}

test_from_str!(test_fonts_from_str, try_fonts_str);
test_from_str!(test_other_from_str, try_other_str);

macro_rules! test_save_attach {
    ($test_name:ident, $from_str:ident) => {
        #[test]
        fn $test_name() {
            let cases = [
                (vec![1, 16, u32::MAX], "1-"),
                (vec![1], "1"),
                (vec![1], "1-1"),
                (vec![2, 16, u32::MAX], "!1"),
                (vec![1, 3, 4], "1,3,4"),
            ];

            for (check_ids, s) in cases {
                let attachs = $from_str(s);
                check_ids
                    .into_iter()
                    .for_each(|id| assert!(attachs.save_attach(id)));
            }

            let bad_cases = [
                (vec![2, 16, u32::MAX], "1"),
                (vec![2, 16, u32::MAX], "1-1"),
                (vec![1], "!1"),
                (vec![2, 5, u32::MAX], "1,3,4"),
            ];

            for (check_ids, s) in bad_cases {
                let attachs = $from_str(s);
                check_ids
                    .into_iter()
                    .for_each(|id| assert!(!attachs.save_attach(id)));
            }
        }
    };
}

test_save_attach!(test_fonts_save_attach, fonts_str);
test_save_attach!(test_other_save_attach, other_str);

#[inline]
fn short_to_long(arg: &str) -> String {
    let arg = match arg {
        "-f" => "--fonts",
        "-F" => "--no-fonts",
        "-m" => "--attachs",
        "-M" => "--no-attachs",
        _ => arg,
    };
    arg.to_string()
}

fn to_long_args(args: &Vec<&str>) -> Vec<String> {
    args.into_iter().map(|arg| short_to_long(arg)).collect()
}

fn current_args(at: AttachType) -> (&'static str, &'static str, &'static str, &'static str) {
    match at {
        AttachType::Font => ("-f", "-F", "-m", "-M"),
        AttachType::Other => ("-m", "-M", "-f", "-F"),
    }
}

#[inline]
fn build_test_to_mkvmerge_args(file: &str, at: AttachType) {
    let (arg, no_arg, alt, no_alt) = current_args(at);

    let cases = [
        (vec![], vec![]),
        (vec![], vec![no_alt]),
        (vec![], vec![alt, "1"]),
        (vec!["-M"], vec![no_arg]),
        (vec!["-m", "1,8,16"], vec![arg, "1,8,16"]),
        (vec!["-m", "!1"], vec![arg, "2-"]),
        (vec!["-m", "!3,4"], vec![arg, "!3,4"]),
        (vec!["-m", "1"], vec![arg, "!2-"]),
    ];

    compare_arg_cases!(
        cases,
        to_long_args,
        file,
        MCFontAttachs,
        MIAttachsInfo,
        MITargets
    );
}

#[test]
fn test_fonts_to_mkvmerge_args() {
    build_test_to_mkvmerge_args("font_attachs_x16.mks", AttachType::Font);
}

#[test]
fn test_other_to_mkvmerge_args() {
    build_test_to_mkvmerge_args("other_attachs_x16.mks", AttachType::Other);
}

#[inline]
fn build_test_mix_to_mvkmerge_args(file: &str, at: AttachType) {
    let (arg, no_arg, alt, no_alt) = current_args(at);

    let cases = [
        (vec![], vec![]),
        (vec!["-m", "1,2,3,4,5,6,7,8"], vec![no_alt]),
        (vec!["-m", "1,2,3,4,5,6,7,8"], vec![alt, "1"]),
        (vec!["-m", "9,10,11,12,13,14,15,16"], vec![no_arg]),
        (vec!["-m", "9,10,11,12,13,14,15,16"], vec![arg, "32"]),
        (vec!["-M"], vec!["-FM"]),
        (vec!["-M"], vec![no_arg, no_alt]),
        (vec!["-M"], vec![arg, "32", alt, "32"]),
        (vec!["-M"], vec![arg, "!1-16", alt, "!1-16"]),
        (vec!["-m", "3,4,10"], vec![arg, "3,4", alt, "10"]),
        (vec!["-m", "3,4,10"], vec![arg, "!1-2,5-", alt, "!9,11-"]),
        (vec!["-m", "!3,4,10"], vec![arg, "!3,4", alt, "!10"]),
        (vec!["-m", "!3,4,10"], vec![arg, "1,2,5-", alt, "9,11-"]),
    ];

    compare_arg_cases!(
        cases,
        to_long_args,
        file,
        MCFontAttachs,
        MIAttachsInfo,
        MITargets
    );
}

#[test]
fn test_mix_fonts_to_mkvmerge_args() {
    build_test_mix_to_mvkmerge_args("font_x8_other_x8.mks", AttachType::Font);
}

#[test]
fn test_mix_other_to_mkvmerge_args() {
    build_test_mix_to_mvkmerge_args("other_x8_font_x8.mks", AttachType::Other);
}
