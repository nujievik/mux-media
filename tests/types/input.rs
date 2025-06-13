use crate::test_cli_args;
use mux_media::{CLIArg, Input};

#[test]
fn test_cli_args() {
    test_cli_args!(Input; Input => "input", Range => "range", Up => "up", Check => "check",
                   Down => "down", Skip => "skip");
}
