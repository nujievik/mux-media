use crate::MuxError;
use std::path::Path;

/// Associates a lazily initialized field with the marker type `F`.
pub trait LazyField<F> {
    /// The type of the associated field.
    type FieldType;

    /// Initializes the field if it hasn't been initialized yet.
    fn try_init(&mut self) -> Result<(), MuxError>;

    /// Initializes the field if it hasn't been initialized yet.
    ///
    /// Returns `Some(())` on success, or `None` on error.
    ///
    /// Default implementation delegates to [`LazyField::try_init`] and performs no allocation.
    fn init(&mut self) -> Option<()> {
        self.try_init().ok()
    }

    /// Returns a mutable reference to the field value, initializing it if necessary.
    fn try_mut(&mut self) -> Result<&mut Self::FieldType, MuxError>;

    /// Returns a mutable reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyField::try_mut`] and performs no allocation.
    fn get_mut(&mut self) -> Option<&mut Self::FieldType> {
        self.try_mut().ok()
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Default implementation delegates to [`LazyField::try_mut`] and performs no allocation.
    fn try_get(&mut self) -> Result<&Self::FieldType, MuxError> {
        self.try_mut().map(|r| &*r)
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyField::get_mut`] and performs no allocation.
    fn get(&mut self) -> Option<&Self::FieldType> {
        self.get_mut().map(|r| &*r)
    }

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns an error if the field is uninitialized or an error occurred.
    fn try_immut(&self) -> Result<&Self::FieldType, MuxError>;

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns `None` if the field is uninitialized or an error occurred.
    ///
    /// Default implementation delegates to [`LazyField::try_immut`] and performs no allocation.
    fn immut(&self) -> Option<&Self::FieldType> {
        self.try_immut().ok()
    }

    /// Takes the field value, initializing it if necessary, and replaces it with a default.
    fn try_take(&mut self) -> Result<Self::FieldType, MuxError>;

    /// Takes the field value, initializing it if needed, and replaces it with a default.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyField::try_take`] and performs no allocation.
    fn take(&mut self) -> Option<Self::FieldType> {
        self.try_take().ok()
    }

    /// Sets the field value manually, replacing an existing value.
    fn set(&mut self, value: Self::FieldType);
}

/// Associates a lazily initialized field with the marker type `F` and [`Path`].
pub trait LazyPathField<F> {
    /// The type of the associated field.
    type FieldType;

    /// Initializes the field if it hasn't been initialized yet.
    fn try_init(&mut self, path: &Path) -> Result<(), MuxError>;

    /// Initializes the field if it hasn't been initialized yet.
    ///
    /// Returns `Some(())` on success, or `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathField::try_init`] and performs no allocation.
    fn init(&mut self, path: &Path) -> Option<()> {
        self.try_init(path).ok()
    }

    /// Returns a mutable reference to the field value, initializing it if necessary.
    fn try_mut(&mut self, path: &Path) -> Result<&mut Self::FieldType, MuxError>;

    /// Returns a mutable reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathField::try_mut`] and performs no allocation.
    fn get_mut(&mut self, path: &Path) -> Option<&mut Self::FieldType> {
        self.try_mut(path).ok()
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Default implementation delegates to [`LazyPathField::try_mut`] and performs no allocation.
    fn try_get(&mut self, path: &Path) -> Result<&Self::FieldType, MuxError> {
        self.try_mut(path).map(|r| &*r)
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathField::get_mut`] and performs no allocation.
    fn get(&mut self, path: &Path) -> Option<&Self::FieldType> {
        self.get_mut(path).map(|r| &*r)
    }

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns an error if the field is uninitialized or an error occurred.
    fn try_immut(&self, path: &Path) -> Result<&Self::FieldType, MuxError>;

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns `None` if the field is uninitialized or an error occurred.
    ///
    /// Default implementation delegates to [`LazyPathField::try_immut`] and performs no allocation.
    fn immut(&self, path: &Path) -> Option<&Self::FieldType> {
        self.try_immut(path).ok()
    }

    /// Takes the field value, initializing it if necessary, and replaces it with a default.
    fn try_take(&mut self, path: &Path) -> Result<Self::FieldType, MuxError>;

    /// Takes the field value, initializing it if needed, and replaces it with a default.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathField::try_take`] and performs no allocation.
    fn take(&mut self, path: &Path) -> Option<Self::FieldType> {
        self.try_take(path).ok()
    }

    /// Sets the field value manually, replacing an existing value.
    fn set(&mut self, path: &Path, value: Self::FieldType);
}

/// Associates a lazily initialized field with the marker type `F`, [`Path`] and [`u64`].
pub trait LazyPathNumField<F> {
    /// The type of the associated field.
    type FieldType;

    /// Initializes the field if it hasn't been initialized yet.
    fn try_init(&mut self, path: &Path, num: u64) -> Result<(), MuxError>;

    /// Initializes the field if it hasn't been initialized yet.
    ///
    /// Returns `Some(())` on success, or `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathNumField::try_init`] and performs no allocation.
    fn init(&mut self, path: &Path, num: u64) -> Option<()> {
        self.try_init(path, num).ok()
    }

    /// Returns a mutable reference to the field value, initializing it if necessary.
    fn try_mut(&mut self, path: &Path, num: u64) -> Result<&mut Self::FieldType, MuxError>;

    /// Returns a mutable reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathNumField::try_mut`] and performs no allocation.
    fn get_mut(&mut self, path: &Path, num: u64) -> Option<&mut Self::FieldType> {
        self.try_mut(path, num).ok()
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Default implementation delegates to [`LazyPathNumField::try_mut`] and performs no allocation.
    fn try_get(&mut self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError> {
        self.try_mut(path, num).map(|r| &*r)
    }

    /// Returns a reference to the field value, initializing it if necessary.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathNumField::get_mut`] and performs no allocation.
    fn get(&mut self, path: &Path, num: u64) -> Option<&Self::FieldType> {
        self.get_mut(path, num).map(|r| &*r)
    }

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns an error if the field is uninitialized or an error occurred.
    fn try_immut(&self, path: &Path, num: u64) -> Result<&Self::FieldType, MuxError>;

    /// Returns a reference to the field value if it has already been initialized.
    ///
    /// Returns `None` if the field is uninitialized or an error occurred.
    ///
    /// Default implementation delegates to [`LazyPathNumField::try_immut`] and performs no allocation.
    fn immut(&self, path: &Path, num: u64) -> Option<&Self::FieldType> {
        self.try_immut(path, num).ok()
    }

    /// Takes the field value, initializing it if necessary, and replaces it with a default.
    fn try_take(&mut self, path: &Path, num: u64) -> Result<Self::FieldType, MuxError>;

    /// Takes the field value, initializing it if needed, and replaces it with a default.
    ///
    /// Returns `None` on error.
    ///
    /// Default implementation delegates to [`LazyPathNumField::try_take`] and performs no allocation.
    fn take(&mut self, path: &Path, num: u64) -> Option<Self::FieldType> {
        self.try_take(path, num).ok()
    }

    /// Sets the field value manually, replacing an existing value.
    fn set(&mut self, path: &Path, num: u64, value: Self::FieldType);
}
