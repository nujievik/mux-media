use super::Msg;

//
pub(super) fn eng(msg: Msg) -> &'static str {
    match msg {
        Msg::ErrUpdLangCode => "LangCode update failed",
        Msg::ErrWriteJson => "Write command to JSON failed",
        Msg::FromPackage => "From package",
        Msg::InstallTool => "Please install it, add to system PATH and re-run",
        Msg::MayFailIfCommandLong => "May fail if command long",
        Msg::NoInputMedia => "No media found in the input directory",
        Msg::NotFound => "Not found",
        Msg::RunningCommand => "Running command",
        Msg::Using => "Using",
    }
}
