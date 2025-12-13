use crate::MuxError;

/// A result of mux current files.
pub enum MuxCurrent<T> {
    Continue,
    Ok(T),
    Err(MuxError),
}

impl<T> From<Result<T, MuxError>> for MuxCurrent<T> {
    fn from(res: Result<T, MuxError>) -> MuxCurrent<T> {
        match res {
            Ok(val) => MuxCurrent::Ok(val),
            Err(e) => MuxCurrent::Err(e),
        }
    }
}
