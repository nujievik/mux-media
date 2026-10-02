macro_rules! some_if_unempty {
    ($v:expr) => {
        if $v.is_empty() { None } else { Some($v) }
    };
}

pub(crate) mod auto_flags;
pub(crate) mod chapters;
pub(crate) mod dispositions;
pub(crate) mod input;
pub(crate) mod log_level;
pub(crate) mod metadata;
pub(crate) mod output;
pub(crate) mod retiming;
pub(crate) mod streams;
pub(crate) mod subs_encoding;

use super::{
    Config, ConfigChapters, ConfigDispositions, ConfigLangMetadata, ConfigStreams,
    ConfigSubsEncoding, ConfigTarget, ConfigTitleMetadata,
};
use crate::{DispositionType, Field, Stream, Target};
use std::path::Path;

impl Config {
    /// Returns a field value for marker `F`:
    ///
    /// - From the first given target contains in [`Config::target_configs`], that has a Some value.
    /// - Otherwise, from the common (global) configuration.
    pub fn target<F, K>(&self, field_mark: F, key: K) -> &<Self as Field<F>>::FieldType
    where
        Self: Field<F>,
        ConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
        K: AsRef<Path>,
    {
        self.get_target(field_mark, key)
            .unwrap_or(<Self as Field<F>>::field(self))
    }

    // Returns (index, val)
    pub(crate) fn stream_val<F, I, T>(
        &self,
        f: F,
        target_paths: I,
        stream: &Stream,
    ) -> (usize, &<Self as Field<F>>::FieldType)
    where
        F: Copy,
        Self: Field<F>,
        ConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        if let Some(v) = self.get_targets(f, target_paths) {
            (stream.i, v)
        } else if let Some(v) = self.get_target(f, Target::StreamType(stream.ty)) {
            (stream.i_ty, v)
        } else {
            (stream.i, <Self as Field<F>>::field(self))
        }
    }

    // Returns (index, val)
    pub(crate) fn stream_val_dispositions<I, T>(
        &self,
        ty: DispositionType,
        target_paths: I,
        stream: &Stream,
    ) -> (usize, &ConfigDispositions)
    where
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        match ty {
            DispositionType::Default => self.stream_val(MarkConfigDefaults, target_paths, stream),
            DispositionType::Forced => self.stream_val(MarkConfigForceds, target_paths, stream),
        }
    }

    #[inline]
    pub fn get_target<F, K>(&self, _: F, key: K) -> Option<&<Self as Field<F>>::FieldType>
    where
        Self: Field<F>,
        ConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
        K: AsRef<Path>,
    {
        self.target_configs
            .get(key.as_ref())
            .and_then(|v| <ConfigTarget as Field<F>>::field(v).as_ref())
    }

    pub fn get_targets<F, I, T>(&self, f: F, ts: I) -> Option<&<Self as Field<F>>::FieldType>
    where
        F: Copy,
        Self: Field<F>,
        ConfigTarget: Field<F, FieldType = Option<<Self as Field<F>>::FieldType>>,
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        ts.into_iter().find_map(|t| self.get_target(f, t))
    }

    /// Gets a cloned [`Target`] key if its exists in [`Self::targets`].
    ///
    /// This operation avoids heap allocation: internally it either copies an enum variant
    /// or increments the [`Arc`](std::sync::Arc) reference count.
    pub(crate) fn get_key(&self, key: impl AsRef<Path>) -> Option<Target> {
        self.target_configs
            .get_key_value(key.as_ref())
            .map(|(k, _)| k.clone())
    }
}

macro_rules! fields {
    // Base type and optional (target) type
    ($type:ident, $opt_type:ident;
    $( $field:ident, $ty:ty => $marker:ident ),* $(,)?
    ) => {
        $(
            #[doc = concat!("Marker of [`Config`] fields, that stores [`", stringify!($ty), "`].")]
            #[derive(Copy, Clone)]
            pub struct $marker;

            impl Field<$marker> for $type {
                type FieldType = $ty;

                #[inline(always)]
                fn field(&self) -> &Self::FieldType {
                    &self.$field
                }
            }

            impl Field<$marker> for $opt_type {
                type FieldType = Option<$ty>;

                #[inline(always)]
                fn field(&self) -> &Self::FieldType {
                    &self.$field
                }
            }
        )*
    };
}

fields! {
    Config, ConfigTarget;
    streams, ConfigStreams => MarkConfigStreams,
    chapters, ConfigChapters => MarkConfigChapters,
    defaults, ConfigDispositions => MarkConfigDefaults,
    forceds, ConfigDispositions => MarkConfigForceds,
    titles, ConfigTitleMetadata => MarkConfigTitleMetadata,
    langs, ConfigLangMetadata => MarkConfigLangMetadata,
    subs_encoding, ConfigSubsEncoding => MarkConfigSubsEncoding,
}
