pub(crate) mod order;
pub(crate) mod streams;
pub(crate) mod supported;
pub(crate) mod ty;

use crate::{CodecId, Lang, StreamType, Value};

/// A stream info.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Stream {
    pub ty: StreamType,

    /// Absolute stream index in the file.
    pub i: usize,

    /// Index by stream type.
    pub i_ty: usize,

    pub codec: CodecId,

    pub lang: Value<Lang>,

    pub name: Option<Value<String>>,

    /// Metadata `filename`.
    pub filename: Option<String>,
}
