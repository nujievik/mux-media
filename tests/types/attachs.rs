use crate::common::cfg;
use crate::{compare_arg_cases, fn_variants_of_args, range, test_cli_args, test_from_str};
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
    test_cli_args!(FontAttachs; Fonts => "fonts", NoFonts => "no-fonts");
    test_cli_args!(OtherAttachs; Attachs => "attachs", NoAttachs => "no-attachs");
}

#[test]
fn test_mkvmerge_args() {
    assert_eq!("-m", Attachs::MKVMERGE_ARG);
    assert_eq!("-M", Attachs::MKVMERGE_NO_ARG);
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
    AttachID::Range(s.parse::<Range<u64>>().unwrap())
}

fn id(num: u64) -> AttachID {
    AttachID::Num(num)
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
    assert!(id(1).contains(&id(1)));
    assert!(!id(1).contains(&id(2)));

    let id_rng = id_range("1-");
    assert!(id_rng.contains(&id(1)));
    assert!(id_rng.contains(&id(2)));
    assert!(id_rng.contains(&id(u64::MAX)));
    assert!(id_rng.contains(&id_range("1-")));

    let id_rng = id_range("2-16");
    assert!(!id_rng.contains(&id(1)));
    assert!(id_rng.contains(&id(2)));
    assert!(id_rng.contains(&id(16)));
    assert!(!id_rng.contains(&id(u64::MAX)));
    assert!(!id_rng.contains(&id_range("1-")));
    assert!(!id_rng.contains(&id_range("1-8")));
    assert!(id_rng.contains(&id_range("2-8")));
}

test_from_str!(
    AttachID, test_id_from_str,
    [
        (AttachID::Num(1), "1"),
        (AttachID::Num(16), "16"),
        (AttachID::Range(range::new("1-")), "1-"),
        (AttachID::Range(range::new("1-8")), "1-8"),
    ],
    ["0", "2-1", "", "x", "eng"],
    @ok_compare
);

fn fonts_str(s: &str) -> FontAttachs {
    s.parse::<FontAttachs>().unwrap()
}

fn other_str(s: &str) -> OtherAttachs {
    s.parse::<OtherAttachs>().unwrap()
}

const FROM_STR_CASES: [&'static str; 5] = ["1-", "1", "1-1", "!1", "1,3,4"];
const FROM_STR_ERR_CASES: [&'static str; 5] = ["0", "2-1", "", "x", "eng"];

test_from_str!(
    FontAttachs,
    test_fonts_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);
test_from_str!(
    OtherAttachs,
    test_other_from_str,
    FROM_STR_CASES,
    FROM_STR_ERR_CASES
);

macro_rules! test_save_attach {
    ($test_name:ident, $from_str:ident) => {
        #[test]
        fn $test_name() {
            let cases = [
                (vec![1, 16, u64::MAX], "1-"),
                (vec![1], "1"),
                (vec![1], "1-1"),
                (vec![2, 16, u64::MAX], "!1"),
                (vec![1, 3, 4], "1,3,4"),
            ];

            for (check_nums, s) in cases {
                let attachs = $from_str(s);
                check_nums
                    .into_iter()
                    .for_each(|num| assert!(attachs.save_attach(&AttachID::Num(num))));
            }

            let bad_cases = [
                (vec![2, 16, u64::MAX], "1"),
                (vec![2, 16, u64::MAX], "1-1"),
                (vec![1], "!1"),
                (vec![2, 5, u64::MAX], "1,3,4"),
            ];

            for (check_nums, s) in bad_cases {
                let attachs = $from_str(s);
                check_nums
                    .into_iter()
                    .for_each(|num| assert!(!attachs.save_attach(&AttachID::Num(num))));
            }
        }
    };
}

test_save_attach!(test_fonts_save_attach, fonts_str);
test_save_attach!(test_other_save_attach, other_str);

fn_variants_of_args!(
    "-f" => vec!["--fonts"],
    "-F" => vec!["--no-fonts"],
    "-m" => vec!["--attachs"],
    "-M" => vec!["--no-attachs"],
);

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
        variants_of_args,
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
        variants_of_args,
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
