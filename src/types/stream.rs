pub(crate) mod order;
pub(crate) mod supported;
pub(crate) mod ty;

use crate::{CodecId, LangCode, StreamType, Value};

#[derive(Clone, Debug)]
pub struct Stream {
    pub ty: StreamType,

    /// Absolute stream index in the file.
    pub i: usize,

    /// Index by stream type.
    pub i_ty: usize,

    pub codec: CodecId,

    pub lang: Value<LangCode>,

    pub name: Option<Value<String>>,

    /// Metadata `filename`.
    pub filename: Option<String>,
}
