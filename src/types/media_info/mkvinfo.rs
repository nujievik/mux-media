use super::cache::CacheState;
use crate::{LangCode, MuxError, SetGetField};

macro_rules! mkvinfo_get_opt_fields {
    ($( $field:ident, $field_ty:ty, $builder:ident => $marker:ident ),* $(,)?) => { $(
        pub(super) struct $marker;

        impl SetGetField<$marker> for Mkvinfo {
            type FieldType = $field_ty;

            fn try_set(&mut self) -> Result<(), MuxError> {
                let (state, result) = match self.$builder() {
                    Ok(val) => (CacheState::Cached(val), Ok(())),
                    Err(e) => (CacheState::Failed, Err(e)),
                };

                self.$field = state;
                result
            }

            fn try_get(&mut self) -> Result<&Self::FieldType, MuxError> {
                if let CacheState::NotCached = self.$field {
                    <Self as SetGetField::<$marker>>::try_set(self)?;
                }

                match &self.$field {
                    CacheState::Cached(val) => Ok(val),
                    CacheState::Failed => Err("Previously failed to load".into()),
                    CacheState::NotCached => Err("Unexpected NotCached state".into()),
                }
            }

            fn get(&mut self) -> Option<&Self::FieldType> {
                match <Self as SetGetField::<$marker>>::try_get(self) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        log::trace!("{}", e.to_str_localized());
                        None
                    }
                }
            }

            fn unmut_get(&self) -> Option<&Self::FieldType> {
                match &self.$field {
                    CacheState::Cached(val) => Some(val),
                    _ => None,
                }
            }
        }
    )* };
}

#[derive(Clone, Default)]
pub struct Mkvinfo {
    lines: Vec<String>,
    name: CacheState<String>,
    lang: CacheState<LangCode>,
}

mkvinfo_get_opt_fields!(
    name, String, build_name => MKVIName,
    lang, LangCode, build_lang => MKVILang,
);

impl Mkvinfo {
    pub(super) fn get<F>(&mut self) -> Option<&<Self as SetGetField<F>>::FieldType>
    where
        Self: SetGetField<F>,
    {
        <Self as SetGetField<F>>::get(self)
    }
}

impl std::ops::Deref for Mkvinfo {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl From<Vec<String>> for Mkvinfo {
    fn from(lines: Vec<String>) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl Mkvinfo {
    fn build_name(&self) -> Result<String, MuxError> {
        self.build_any_str_val("Name:")
    }

    fn build_lang(&self) -> Result<LangCode, MuxError> {
        // Default Matroska state is LangCode::Eng
        Ok(self
            .build_any_str_val("Language:")
            .and_then(|s| s.parse::<LangCode>())
            .unwrap_or(LangCode::Eng))
    }

    fn build_any_str_val(&self, key: &str) -> Result<String, MuxError> {
        self.lines
            .iter()
            .find_map(|line| {
                line.split_once(key)
                    .map(|(_, right)| right.trim().to_string())
            })
            .ok_or_else(|| format!("Not found line with '{}'", key).into())
    }
}
