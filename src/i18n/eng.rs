use super::Msg;

pub(super) fn eng(msg: &Msg) -> String {
    match msg {
        Msg::ErrUpdLangCode => "LangCode update failed".into(),
        Msg::FailSetPaths { s, s1 } => format!(
            "'{}' from package '{}' is not installed. Please install it, add to system PATH and re-run",
            s, s1
        ),
        Msg::FailWriteJson { s } => format!(
            "Failed to write command to JSON: {}. Using CLI (may fail if command is too long)",
            s
        ),
        Msg::NoInputFiles => "No track files found in the input directory".to_string(),
        Msg::RunningCommand => "Running command".to_string(),
        Msg::Using => "Using".into(),
    }
}
