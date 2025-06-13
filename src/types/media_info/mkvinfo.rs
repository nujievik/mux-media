use super::CacheState;
use crate::{GetOptField, LangCode};

macro_rules! mkvinfo_get_opt_fields {
    ($( $field:ident, $field_ty:ty, $builder:ident => $marker:ident ),* $(,)?) => {
        $(
            pub(super) struct $marker;

            impl GetOptField<$marker> for Mkvinfo {
                type FieldType = $field_ty;
                fn get(&self) -> Option<&Self::FieldType> {
                    if let CacheState::NotCached = self.$field {
                        self.$builder();
                    }
                    match &self.$field {
                        CacheState::Cached(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
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
    pub(super) fn get<F>(&self) -> Option<&<Self as GetOptField<F>>::FieldType>
    where
        Self: GetOptField<F>,
    {
        <Self as GetOptField<F>>::get(self)
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
    fn build_name(&self) -> Option<String> {
        self.build_any_str_val("Name:")
    }

    fn build_lang(&self) -> Option<LangCode> {
        // Default Matroska state is LangCode::Eng
        let lang = self
            .build_any_str_val("Language:")
            .and_then(|s| s.parse::<LangCode>().ok())
            .unwrap_or(LangCode::Eng);
        Some(lang)
    }

    fn build_any_str_val(&self, key: &str) -> Option<String> {
        for line in &self.lines {
            if line.contains(key) {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let val = if parts.len() > 1 {
                    parts[1].trim()
                } else {
                    parts[0].trim()
                };
                return Some(val.to_string());
            }
        }
        None
    }
}
