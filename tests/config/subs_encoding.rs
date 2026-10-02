use super::*;
use encoding_rs::Encoding;

static LABELS: &[&str] = &["cp1251", "utf-8", "utf-16"];

test_from_str!(from_str, ConfigSubsEncoding; LABELS; ["invalid"]);

#[test]
fn parse() {
    for label in LABELS {
        let enc = Encoding::for_label_no_replacement(label.as_bytes()).unwrap();
        let c = cfg(["--subs-encoding", label]);

        assert_eq!(enc, c.subs_encoding.get_encoding().unwrap());
    }
}

#[test]
fn parse_target() {
    for label in LABELS {
        let enc = Encoding::for_label_no_replacement(label.as_bytes()).unwrap();
        let c = cfg(["-t", "audio", "--subs-encoding", label]);

        let val = c.get_target(MarkConfigSubsEncoding, "audio").unwrap();
        assert_eq!(enc, val.get_encoding().unwrap());
    }
}

#[test]
fn parse_updated() {
    for label in LABELS {
        let enc = Encoding::for_label_no_replacement(b"cp1252").unwrap();
        let c = cfg([
            "--subs-encoding",
            label,
            "-t",
            "global",
            "--subs-encoding",
            "cp1252",
        ]);

        assert_eq!(enc, c.subs_encoding.get_encoding().unwrap());
    }
}

#[test]
fn parse_updated_target() {
    for label in LABELS {
        let enc = Encoding::for_label_no_replacement(b"cp1252").unwrap();
        let c = cfg([
            "-t",
            "audio",
            "--subs-encoding",
            label,
            "-t",
            "audio",
            "--subs-encoding",
            "cp1252",
        ]);

        let val = c.get_target(MarkConfigSubsEncoding, "audio").unwrap();
        assert_eq!(enc, val.get_encoding().unwrap());
    }
}

build_test_to_args!(
    to_args, "subs_encoding";
    vec!["--subs-encoding", "windows-1251"],
    vec!["--subs-encoding", "UTF-8"],
    vec!["--subs-encoding", "UTF-16LE"]
);
