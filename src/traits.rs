pub(crate) mod lazy_fields;

use crate::Result;

/// Provides a delayed initialization for expensive operations.
pub trait TryFinalizeInit {
    /// Finalizes initialization.
    fn try_finalize_init(&mut self) -> Result<()>;
}

/// Converts a value to JSON-compatible arguments.
pub trait ToJsonArgs {
    /// Appends arguments to the given `args` vector.
    fn append_json_args(&self, args: &mut Vec<String>);

    /// Returns arguments.
    fn to_json_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        self.append_json_args(&mut args);
        args
    }
}

/// Associates a field with the marker type `F`.
pub trait Field<F> {
    type FieldType;

    /// Returns a reference to the field value.
    fn field(&self) -> &Self::FieldType;
}
