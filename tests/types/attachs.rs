use crate::{common::cfg, *};
use mux_media::{markers::*, *};

fn new_fonts(args: &[&str]) -> FontAttachs {
    let mc = cfg::<_, &str>(args.iter().copied());
    <Config as Field<MCFontAttachs>>::field(&mc).clone()
}

fn new_other(args: &[&str]) -> OtherAttachs {
    let mc = cfg::<_, &str>(args.iter().copied());
    <Config as Field<MCOtherAttachs>>::field(&mc).clone()
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

fn id_range(s: &str) -> AttachID {
    AttachID::Range(s.parse::<RangeU64>().unwrap())
}

fn id(num: u64) -> AttachID {
    AttachID::Num(num)
}

#[test]
fn test_id_contains() {
    assert!(id(1).contains(&id(1)));
    assert!(!id(1).contains(&id(2)));

    let id_rng = id_range("1-");
    assert!(id_rng.contains(&id(1)));
    assert!(id_rng.contains(&id(2)));
    assert!(id_rng.contains(&id(u64::MAX - 1)));
    assert!(id_rng.contains(&id_range("1-")));

    let id_rng = id_range("2-16");
    assert!(!id_rng.contains(&id(1)));
    assert!(id_rng.contains(&id(2)));
    assert!(id_rng.contains(&id(16)));
    assert!(!id_rng.contains(&id(u64::MAX - 1)));
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
                (vec![1, 16, u64::MAX - 1], "1-"),
                (vec![1], "1"),
                (vec![1], "1-1"),
                (vec![2, 16, u64::MAX - 1], "!1"),
                (vec![1, 3, 4], "1,3,4"),
            ];

            for (check_nums, s) in cases {
                let attachs = $from_str(s);
                check_nums
                    .into_iter()
                    .for_each(|num| assert!(attachs.save_attach(&AttachID::Num(num))));
            }

            let bad_cases = [
                (vec![2, 16, u64::MAX - 1], "1"),
                (vec![2, 16, u64::MAX - 1], "1-1"),
                (vec![1], "!1"),
                (vec![2, 5, u64::MAX - 1], "1,3,4"),
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

macro_rules! build_test_attachs_to_json_args {
    ( $( $fn:ident, $field:ident, $json_dir:expr, $arg:expr, $no_arg:expr );* ) => {
        $(
            build_test_to_json_args!(
                $fn, $field, $json_dir, @diff_in_out;
                vec![], vec![],
                vec![$no_arg], vec![$no_arg],
                vec![$arg, "1"], vec![$arg, "1"],
                vec![$arg, "1,2,3"], vec![$arg, "1,2,3"],
                vec![$arg, "1,2,3"], vec![$arg, "2,3,1"],
                vec![$arg, "1-5"], vec![$arg, "1-5"],
            );
        )*
    };
}

build_test_attachs_to_json_args!(
    test_fonts_to_json_args, font_attachs, "font_attachs", "--fonts", "--no-fonts";
    test_others_to_json_args, other_attachs, "other_tracks", "--attachs", "--no-attachs"
);
