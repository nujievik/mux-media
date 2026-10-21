use super::*;

build_test_to_args!(
    to_args, "config_locales";
    vec![],
    vec!["--locale-streams", "rus"],
    /*
    vec!["--ts-offsets", "1:0,2:1,8:4"],
    vec!["--ts-offsets", "1:0,2:1,8:4"],
    vec!["--ts-offsets", "1:0,2-4:1,eng:4"],
    vec!["--ts-offsets", "1:0,2:-1,8:-4"],
    */
);
