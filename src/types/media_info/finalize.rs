use super::MediaInfo;
use crate::{
    ArcPathBuf, IsDefault, LangCode, Result, Stream, Target, TryFinalizeInit, Value, markers::*,
};
use std::path::Path;

impl TryFinalizeInit for MediaInfo<'_> {
    fn try_finalize_init(&mut self) -> Result<()> {
        self.try_init_cmn(MICmnStem)?;

        let sources: Vec<ArcPathBuf> = self.cache.of_files.keys().cloned().collect();
        for src in sources.iter() {
            let src = src.as_path();
            self.try_finalize_init_streams_src(src)?;
            self.try_init(MIPathTail, src)?;
            self.try_init(MIRelativeUpmost, src)?;
            self.try_init(MISubCharEncoding, src)?;
            self.try_init(MITargetPaths, src)?;
            self.try_init(MIPlayableDuration, src)?;
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
        let mut streams = self.try_take(MIStreams, src)?;
        let ts = self.try_take(MITargetPaths, src)?;

        for stream in streams.iter_mut() {
            if let Some(n) = self.get_name(src, &ts, stream) {
                stream.name = Some(n);
            }
            if let Some(l) = self.get_lang(src, &ts, stream) {
                stream.lang = l;
            }
        }

        self.set(MIStreams, src, streams);
        self.set(MITargetPaths, src, ts);

        Ok(())
    }

    fn get_name(&mut self, src: &Path, ts: &Vec<Target>, stream: &Stream) -> Option<Value<String>> {
        let (i, names) = self.cfg.stream_val(CfgNames, ts, stream);

        if let Some(n) = names.get(&i, &stream.lang) {
            return Some(Value::User(n.clone()));
        }

        if stream.name.as_ref().is_some_and(|n| !n.is_empty()) || !*self.cfg.auto_flags.names {
            return None;
        }

        if let Some(n) = self.get(MIPathTail, src).and_then(|tail| {
            let s = tail.trim_matches(&['.', ' ']);
            (s.len() > 2).then(|| s.to_owned())
        }) {
            return Some(Value::Auto(n));
        }

        // From parent
        if let Some(n) = src
            .parent()
            .filter(|p| p.as_os_str().len() != self.cfg.input.dir.as_os_str().len())
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
        ts: &Vec<Target>,
        stream: &Stream,
    ) -> Option<Value<LangCode>> {
        let (i, langs) = self.cfg.stream_val(CfgLangs, ts, stream);

        if let Some(l) = langs.get(&i, &stream.lang) {
            return Some(Value::User(*l));
        }

        if !stream.lang.is_default() || !*self.cfg.auto_flags.langs {
            return None;
        }

        let parse = |opt_s: Option<&String>| {
            opt_s
                .and_then(|s| s.parse::<LangCode>().ok())
                .filter(|l| !l.is_default() && l.has_duo_code())
        };

        parse(stream.name.as_ref().map(|v| &**v))
            .or_else(|| parse(self.get(MIPathTail, src)))
            .or_else(|| parse(self.get(MIRelativeUpmost, src)))
            .map(|l| Value::Auto(l))
    }
}
