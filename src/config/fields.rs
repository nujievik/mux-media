macro_rules! some_if_unempty {
    ($v:expr) => {
        if $v.is_empty() { None } else { Some($v) }
    };
}

pub(crate) mod auto_flags;
pub(crate) mod chapters;
pub(crate) mod dispositions;
pub(crate) mod input;
pub(crate) mod locales;
pub(crate) mod log_level;
pub(crate) mod metadata;
pub(crate) mod output;
pub(crate) mod retiming;
pub(crate) mod streams;
pub(crate) mod streams_order;
pub(crate) mod subs_encodings;
pub(crate) mod ts_offsets;

use super::{
    Config, ConfigChapters, ConfigDispositions, ConfigLangMetadata, ConfigStreams,
    ConfigStreamsOrder, ConfigSubsEncodings, ConfigTarget, ConfigTitleMetadata, ConfigTsOffsets,
};
use crate::{DispositionType, Field, Stream};
use std::path::Path;

impl Config {
    #[inline]
    pub fn target_or_global<'a, F, TY>(&'a self, f: F, trg: &'a ConfigTarget) -> &'a TY
    where
        Config: Field<F, FieldType = TY>,
        ConfigTarget: Field<F, FieldType = Option<TY>>,
    {
        trg.get(f).unwrap_or(<Self as Field<F>>::field(self))
    }

    pub(crate) fn stream_value<'a, F, TY>(
        &'a self,
        _: F,
        path_configs: &'a PathConfigs<'_>,
        stream: &Stream,
    ) -> (usize, &'a TY)
    where
        Config: Field<F, FieldType = TY>,
        PathConfigs<'a>: Field<F, FieldType = Option<TY>>,
        ConfigTarget: Field<F, FieldType = Option<TY>>,
    {
        if let Some(v) = <PathConfigs as Field<F>>::field(path_configs) {
            (stream.i, v)
        } else if let Some(v) = <ConfigTarget as Field<F>>::field(&self.per_stream_type[stream.ty])
        {
            (stream.i_ty, v)
        } else {
            (stream.i, <Config as Field<F>>::field(self))
        }
    }

    pub(crate) fn stream_dispositions<'a>(
        &'a self,
        ty: DispositionType,
        path_configs: &'a PathConfigs<'_>,
        stream: &Stream,
    ) -> (usize, &'a ConfigDispositions) {
        match ty {
            DispositionType::Default => self.stream_value(MarkConfigDefaults, path_configs, stream),
            DispositionType::Forced => self.stream_value(MarkConfigForceds, path_configs, stream),
        }
    }
}

macro_rules! fields {
    ( $( $field:ident, $ty:ty => $marker:ident ),* $(,)? ) => {
        $(
            #[doc = concat!("Marker of [`Config`] fields, that stores [`", stringify!($ty), "`].")]
            pub struct $marker;

            impl Field<$marker> for Config {
                type FieldType = $ty;

                #[inline]
                fn field(&self) -> &$ty {
                    &self.$field
                }
            }

            impl Field<$marker> for ConfigTarget {
                type FieldType = Option<$ty>;

                #[inline]
                fn field(&self) -> &Option<$ty> {
                    &self.$field
                }
            }

            impl<'a> Field<$marker> for PathConfigs<'a> {
                type FieldType = Option<$ty>;

                #[inline]
                fn field(&self) -> &'a Option<$ty> {
                    self.field($marker)
                }
            }
        )*
    };
}

fields! {
    streams, ConfigStreams => MarkConfigStreams,
    chapters, ConfigChapters => MarkConfigChapters,
    order, ConfigStreamsOrder => MarkConfigStreamsOrder,
    defaults, ConfigDispositions => MarkConfigDefaults,
    forceds, ConfigDispositions => MarkConfigForceds,
    titles, ConfigTitleMetadata => MarkConfigTitleMetadata,
    langs, ConfigLangMetadata => MarkConfigLangMetadata,
    subs_encodings, ConfigSubsEncodings => MarkConfigSubsEncodings,
    ts_offsets, ConfigTsOffsets => MarkConfigTsOffsets,
}

#[derive(Copy, Clone, Debug)]
pub struct PathConfigs<'a> {
    value: PathConfigsValue<'a>,
}
#[derive(Copy, Clone, Debug)]
enum PathConfigsValue<'a> {
    FileAndParentDir((&'a ConfigTarget, &'a ConfigTarget)),
    FileOrParentDir(&'a ConfigTarget),
    Empty,
}
use PathConfigsValue::*;

impl ConfigTarget {
    #[inline]
    pub fn get<F, TY>(&self, _: F) -> Option<&TY>
    where
        ConfigTarget: Field<F, FieldType = Option<TY>>,
    {
        <ConfigTarget as Field<F>>::field(self).as_ref()
    }
}

impl<'a> PathConfigs<'a> {
    pub(crate) fn new(cfg: &'a Config, file: &Path) -> PathConfigs<'a> {
        let dir = file.parent().and_then(|p| cfg.by_path.get_key_value(p));

        let value = match (cfg.by_path.get_key_value(file), dir) {
            (Some((_, file)), Some((_, dir))) => FileAndParentDir((file, dir)),
            (Some((_, c)), None) | (None, Some((_, c))) => FileOrParentDir(c),
            (None, None) => Empty,
        };
        PathConfigs { value }
    }

    #[inline]
    pub fn get<F, TY>(&self, f: F) -> Option<&'a TY>
    where
        ConfigTarget: Field<F, FieldType = Option<TY>>,
    {
        self.field(f).as_ref()
    }

    #[inline]
    pub fn field<F, TY>(&self, _: F) -> &'a Option<TY>
    where
        ConfigTarget: Field<F, FieldType = Option<TY>>,
    {
        match self.value {
            FileAndParentDir((f, d)) => {
                let f_field = <ConfigTarget as Field<F>>::field(f);

                if f_field.is_some() {
                    f_field
                } else {
                    <ConfigTarget as Field<F>>::field(d)
                }
            }
            FileOrParentDir(x) => <ConfigTarget as Field<F>>::field(x),
            Empty => &None,
        }
    }
}
