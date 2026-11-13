use super::*;
use crate::config::{MarkConfigLangMetadata, MarkConfigTitleMetadata, PathConfigs};
use crate::{IsDefault, Lang, LangCode, Result, Stream, TryFinalizeInit, Value};
use std::{ffi::OsStr, path::Path};

impl TryFinalizeInit for MediaInfo<'_> {
    fn try_finalize_init(&mut self) -> Result<()> {
        let cfg = self.cfg;
        let shortest_stem = self.try_take_cmn(MarkMediaInfoShortestFileStem)?;

        for (src, cache) in self.cache.of_files.iter_mut() {
            let path_configs = cache.caching_path_configs(cfg, src);
            try_finalize_init_streams_src(cfg, &shortest_stem, src, cache, &path_configs)?;
            cache.caching_durations(src)?;
            cache.caching_path_tail(src, &shortest_stem);
            cache.caching_relative_upmost(cfg, src);
        }

        self.cache.of_group.shortest_file_stem = Cached(shortest_stem);
        Ok(())
    }
}

impl MediaInfo<'_> {
    pub fn try_finalize_init_streams(&mut self) -> Result<()> {
        let cfg = self.cfg;
        let shortest_stem = self.try_take_cmn(MarkMediaInfoShortestFileStem)?;

        for (src, cache) in self.cache.of_files.iter_mut() {
            let path_configs = cache.caching_path_configs(cfg, src);
            try_finalize_init_streams_src(cfg, &shortest_stem, src, cache, &path_configs)?;
        }

        self.cache.of_group.shortest_file_stem = Cached(shortest_stem);
        Ok(())
    }
}

fn try_finalize_init_streams_src<'a>(
    cfg: &'a Config,
    shortest_stem: &OsStr,
    src: &Path,
    cache: &mut MediaInfoCacheOfFile<'a>,
    path_configs: &PathConfigs,
) -> Result<()> {
    cache.caching_streams(src)?;
    let mut streams = cache.streams.try_take().unwrap();

    for stream in streams.iter_mut() {
        if let Some(n) = get_title(cfg, &shortest_stem, src, cache, &path_configs, stream) {
            stream.title = Some(n);
        }
        if let Some(l) = get_lang(cfg, &shortest_stem, src, cache, &path_configs, stream) {
            stream.lang = l;
        }
    }

    cache.streams = Cached(streams);
    Ok(())
}

fn get_title<'a>(
    cfg: &'a Config,
    shortest_stem: &OsStr,
    src: &Path,
    cache: &mut MediaInfoCacheOfFile<'a>,
    path_configs: &PathConfigs,
    stream: &Stream,
) -> Option<Value<String>> {
    let (i, titles) = cfg.stream_value(MarkConfigTitleMetadata, path_configs, stream);

    if let Some(n) = titles.get(&i, &stream.lang) {
        return Some(Value::User(n.clone()));
    }

    if stream.title.as_ref().is_some_and(|n| !n.is_empty()) || !*cfg.auto_flags.titles {
        return None;
    }

    let path_tail = cache.caching_path_tail(src, shortest_stem);
    let trimmed = path_tail.trim_matches(&['.', ' ']);
    if trimmed.len() > 2 {
        return Some(Value::Auto(trimmed.to_owned()));
    }

    // From parent
    if let Some(n) = src
        .parent()
        .filter(|p| p.as_os_str().len() != cfg.input.dir().as_os_str().len())
        .and_then(|p| p.file_name())
        .map(|p| p.to_string_lossy().into_owned())
    {
        return Some(Value::Auto(n));
    }

    None
}

fn get_lang<'a>(
    cfg: &'a Config,
    shortest_stem: &OsStr,
    src: &Path,
    cache: &mut MediaInfoCacheOfFile<'a>,
    path_configs: &PathConfigs,
    stream: &Stream,
) -> Option<Value<Lang>> {
    let (i, langs) = cfg.stream_value(MarkConfigLangMetadata, path_configs, stream);

    if let Some(l) = langs.get(&i, &stream.lang) {
        return Some(Value::User(l.clone()));
    }

    if !stream.lang.is_default() || !*cfg.auto_flags.langs {
        return None;
    }

    let parse = |s: &String| LangCode::get(s).filter(|code| !code.is_default());

    stream
        .title
        .as_ref()
        .and_then(|t| parse(t))
        .or_else(|| parse(cache.caching_path_tail(src, shortest_stem)))
        .or_else(|| parse(cache.caching_relative_upmost(cfg, src)))
        .map(|code| Value::Auto(Lang::Code(code)))
}
