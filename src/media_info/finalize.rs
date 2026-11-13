use super::*;
use crate::config::{MarkConfigLangMetadata, MarkConfigTitleMetadata};
use crate::{ArcPathBuf, IsDefault, Lang, LangCode, Result, Stream, TryFinalizeInit, Value};
use std::path::Path;

impl TryFinalizeInit for MediaInfo<'_> {
    fn try_finalize_init(&mut self) -> Result<()> {
        self.try_init_cmn(MarkMediaInfoStem)?;

        let sources: Vec<ArcPathBuf> = self.cache.of_files.keys().cloned().collect();
        for src in sources.iter() {
            let src = src.as_path();
            self.try_finalize_init_streams_src(src)?;
            self.try_init(MarkMediaInfoPathTail, src)?;
            self.try_init(MarkMediaInfoRelativeUpmost, src)?;
            self.try_init(MarkMediaInfoVideoAndAudioDurations, src)?;
        }

        Ok(())
    }
}

impl MediaInfo<'_> {
    pub fn try_finalize_init_streams(&mut self) -> Result<()> {
        let sources: Vec<ArcPathBuf> = self.cache.of_files.keys().cloned().collect();
        for src in sources.iter() {
            self.try_finalize_init_streams_src(&src)?;
        }
        Ok(())
    }

    fn try_finalize_init_streams_src(&mut self, src: &Path) -> Result<()> {
        let mut streams = self.try_take(MarkMediaInfoStreams, src)?;
        let path_configs = self.path_configs(src);

        for stream in streams.iter_mut() {
            if let Some(n) = self.get_title(src, &path_configs, stream) {
                stream.title = Some(n);
            }
            if let Some(l) = self.get_lang(src, &path_configs, stream) {
                stream.lang = l;
            }
        }

        self.set(MarkMediaInfoStreams, src, streams);

        Ok(())
    }

    fn get_title(
        &mut self,
        src: &Path,
        path_configs: &PathConfigs,
        stream: &Stream,
    ) -> Option<Value<String>> {
        let (i, titles) = self
            .cfg
            .stream_value(MarkConfigTitleMetadata, path_configs, stream);

        if let Some(n) = titles.get(&i, &stream.lang) {
            return Some(Value::User(n.clone()));
        }

        if stream.title.as_ref().is_some_and(|n| !n.is_empty()) || !*self.cfg.auto_flags.titles {
            return None;
        }

        if let Some(n) = self.get(MarkMediaInfoPathTail, src).and_then(|tail| {
            let s = tail.trim_matches(&['.', ' ']);
            (s.len() > 2).then(|| s.to_owned())
        }) {
            return Some(Value::Auto(n));
        }

        // From parent
        if let Some(n) = src
            .parent()
            .filter(|p| p.as_os_str().len() != self.cfg.input.dir().as_os_str().len())
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
        {
            return Some(Value::Auto(n));
        }

        None
    }

    fn get_lang(
        &mut self,
        src: &Path,
        path_configs: &PathConfigs,
        stream: &Stream,
    ) -> Option<Value<Lang>> {
        let (i, langs) = self
            .cfg
            .stream_value(MarkConfigLangMetadata, path_configs, stream);

        if let Some(l) = langs.get(&i, &stream.lang) {
            return Some(Value::User(l.clone()));
        }

        if !stream.lang.is_default() || !*self.cfg.auto_flags.langs {
            return None;
        }

        let parse = |opt_s: Option<&String>| {
            opt_s
                .and_then(|s| LangCode::get(s))
                .filter(|c| !c.is_default())
        };

        parse(stream.title.as_ref().map(|v| &**v))
            .or_else(|| parse(self.get(MarkMediaInfoPathTail, src)))
            .or_else(|| parse(self.get(MarkMediaInfoRelativeUpmost, src)))
            .map(|code| Value::Auto(Lang::Code(code)))
    }
}
