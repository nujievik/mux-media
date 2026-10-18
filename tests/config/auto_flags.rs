use super::*;

fn new(args: &[&str]) -> ConfigAutoFlags {
    cfg::<_, &&str>(args).auto_flags
}

#[test]
fn parse_empty() {
    let f = new(&[]);
    assert_eq!(false, f.no_auto);
    assert_eq!(Value::Auto(true), f.defaults);
    assert_eq!(Value::Auto(true), f.forceds);
    assert_eq!(Value::Auto(true), f.titles);
    assert_eq!(Value::Auto(true), f.langs);
    assert_eq!(Value::Auto(true), f.encs);
}

#[test]
fn parse_no_auto() {
    let f = new(&["--no-auto"]);
    assert_eq!(true, f.no_auto);
    assert_eq!(Value::Auto(false), f.defaults);
    assert_eq!(Value::Auto(false), f.forceds);
    assert_eq!(Value::Auto(false), f.titles);
    assert_eq!(Value::Auto(false), f.langs);
    assert_eq!(Value::Auto(false), f.encs);
}

macro_rules! parse_manual_test {
    ($fn:ident, $v:expr, $auto_v:expr; $( $args:expr, $v_field:ident );* $(;)?) => {
        #[test]
        fn $fn() {
            let v = Value::User($v);
            let auto_v = Value::Auto($auto_v);
            $(
                let mut flags = new($args);

                assert_eq!(v, flags.$v_field);
                // replace to auto
                flags.$v_field = auto_v;

                assert_eq!(auto_v, flags.defaults);
                assert_eq!(auto_v, flags.forceds);
                assert_eq!(auto_v, flags.titles);
                assert_eq!(auto_v, flags.langs);
                assert_eq!(auto_v, flags.encs);
            )*
        }
    }
}

parse_manual_test!(
    parse_manual_on, true, true;
    &["--auto-defaults"], defaults;
    &["--auto-forceds"], forceds;
    &["--auto-titles"], titles;
    &["--auto-langs"], langs;
    &["--auto-encs"], encs;
);

parse_manual_test!(
    parse_manual_off, false, true;
    &["--no-auto-defaults"], defaults;
    &["--no-auto-forceds"], forceds;
    &["--no-auto-titles"], titles;
    &["--no-auto-langs"], langs;
    &["--no-auto-encs"], encs;
);

parse_manual_test!(
    parse_manual_on_with_no_auto, true, false;
    &["--no-auto", "--auto-defaults"], defaults;
    &["--no-auto", "--auto-forceds"], forceds;
    &["--no-auto", "--auto-titles"], titles;
    &["--no-auto", "--auto-langs"], langs;
    &["--no-auto", "--auto-encs"], encs;
);

parse_manual_test!(
    parse_manual_off_with_no_auto, false, false;
    &["--no-auto", "--no-auto-defaults"], defaults;
    &["--no-auto", "--no-auto-forceds"], forceds;
    &["--no-auto", "--no-auto-titles"], titles;
    &["--no-auto", "--no-auto-langs"], langs;
    &["--no-auto", "--no-auto-encs"], encs;
);

macro_rules! update_test {
    ( $( $arg:expr, $alt_arg:expr );* $(;)?) => {
        #[test]
        fn parse_update() {
            assert_eq!(new(&[]), new(&["-t", "g"]));
            assert_eq!(new(&["--no-auto"]), new(&["-t", "g", "--no-auto"]));

            $(
                assert_eq!(new(&[$arg]), new(&[$alt_arg, "-t", "g", $arg]));

                assert_eq!(new(&[$alt_arg]), new(&[$arg, "-t", "g", $alt_arg]));
            )*
        }
    };
}

update_test!(
    "--auto-defaults", "--no-auto-defaults";
    "--auto-forceds", "--no-auto-forceds";
    "--auto-titles", "--no-auto-titles";
    "--auto-langs", "--no-auto-langs";
    "--auto-encs", "--no-auto-encs";
);

crate::build_test_to_args!(
    to_args, "auto_flags";
    vec![],
    vec!["--no-auto-defaults"],
    vec!["--no-auto-forceds"],
    vec!["--no-auto-titles"],
    vec!["--no-auto-langs"],
    vec!["--no-auto-encs"],
    vec!["--no-auto", "--auto-defaults"],
    vec!["--no-auto", "--auto-forceds"],
    vec!["--no-auto", "--auto-titles"],
    vec!["--no-auto", "--auto-langs"],
    vec!["--no-auto", "--auto-encs"],
);
