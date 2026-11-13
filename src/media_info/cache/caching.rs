use super::*;
use crate::media_info::MarkMediaInfoShortestFileStem;
use crate::{CodecId, Config, Extension, Lang, MediaInfo, StreamType, Value, helpers};
use std::{ffi::OsStr, path::Path};

impl<'a> MediaInfoCache<'a> {
    pub(crate) fn caching_file_cache<'b>(
        &'b mut self,
        src: &Path,
    ) -> &'b mut MediaInfoCacheOfFile<'a> {
        if !self.of_files.contains_key(src) {
            self.of_files.insert(src.into(), Default::default());
        }
        match self.of_files.get_mut(src) {
            Some(cache) => cache,
            _ => unreachable!(),
        }
    }
}

macro_rules! cached_or_unreachable {
    ($v:expr) => {
        match $v {
            Cached(v) => v,
            _ => unreachable!(),
        }
    };
}

impl<'a> MediaInfoCacheOfFile<'a> {
    pub(crate) fn caching_path_configs(&mut self, cfg: &'a Config, src: &Path) -> PathConfigs<'a> {
        *self
            .path_configs
            .get_or_insert_with(|| PathConfigs::new(cfg, src))
    }

    pub(crate) fn caching_streams(&mut self, src: &Path) -> Result<&mut Vec<Stream>> {
        if let NotCached = &self.streams {
            return self.streams.set(MediaInfo::help_build_streams(src));
        }
        self.streams.try_mut()
    }

    pub(crate) fn caching_path_tail(&mut self, src: &Path, shortest_stem: &OsStr) -> &mut String {
        if !matches!(self.path_tail, Cached(_)) {
            self.path_tail = Cached(MediaInfo::help_build_path_tail(src, shortest_stem));
        }
        cached_or_unreachable!(&mut self.path_tail)
    }

    pub(crate) fn caching_relative_upmost(&mut self, cfg: &Config, src: &Path) -> &mut String {
        if !matches!(self.relative_upmost, Cached(_)) {
            self.relative_upmost = Cached(MediaInfo::help_build_relative_upmost(cfg, src));
        }
        cached_or_unreachable!(&mut self.relative_upmost)
    }

    pub(crate) fn caching_durations(&mut self, src: &Path) -> Result<&mut VideoAndAudioDurations> {
        if let NotCached = &self.video_and_audio_durations {
            let res = VideoAndAudioDurations::try_from_streams(src, self);
            return self.video_and_audio_durations.set(res);
        }
        self.video_and_audio_durations.try_mut()
    }
}

impl MediaInfo<'_> {
    pub(crate) fn build_shortest_file_stem(&self) -> Result<OsString> {
        let shortest = self
            .cache
            .of_files
            .iter()
            .filter_map(|(p, _)| p.file_stem())
            .min_by_key(|s| s.len())
            .map(|stem| stem.to_owned());

        Ok(shortest.unwrap_or_default())
    }

    pub(crate) fn build_streams_order(&mut self) -> Result<StreamsOrder> {
        StreamsOrder::new(self)
    }

    pub(crate) fn build_streams(&self, src: &Path) -> Result<Vec<Stream>> {
        MediaInfo::help_build_streams(src)
    }

    pub(crate) fn help_build_streams(src: &Path) -> Result<Vec<Stream>> {
        build_streams(src)
    }

    pub(crate) fn build_path_tail(&mut self, src: &Path) -> Result<String> {
        let shortest_stem = match self.get_cmn(MarkMediaInfoShortestFileStem) {
            Some(s) => s,
            None => return Ok(String::new()),
        };
        Ok(Self::help_build_path_tail(src, shortest_stem))
    }

    fn help_build_path_tail(src: &Path, shortest_stem: &OsStr) -> String {
        src.file_stem()
            .and_then(|stem| helpers::os_str_tail(shortest_stem, stem).ok())
            .map(|tail| tail.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub(crate) fn build_relative_upmost(&self, src: &Path) -> Result<String> {
        Ok(Self::help_build_relative_upmost(self.cfg, src))
    }

    fn help_build_relative_upmost(cfg: &Config, src: &Path) -> String {
        src.parent()
            .and_then(|parent| {
                helpers::os_str_tail(cfg.input.dir().as_os_str(), parent.as_os_str()).ok()
            })
            .map(|os| os.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub(crate) fn build_durations(&mut self, src: &Path) -> Result<VideoAndAudioDurations> {
        let cache = self.cache.caching_file_cache(src);
        VideoAndAudioDurations::try_from_streams(src, cache)
    }
}

fn build_streams(src: &Path) -> Result<Vec<Stream>> {
    use crate::ffmpeg::{ffi, format::input, util::media::Type};

    fn is_font_filename(opt_s: &Option<String>) -> bool {
        opt_s
            .as_ref()
            .is_some_and(|s| Extension::from_path(s).is_some_and(|ext| ext.is_font()))
    }

    let ictx = input(src)?;
    let mut idxs = StreamType::map::<usize>();

    let streams = ictx.streams().map(|stream| {
        let meta = stream.metadata();
        let lang = meta
            .get("language")
            .map(|v| Lang::new(v))
            .unwrap_or_default();
        let title = meta.get("title").map(|v| Value::Auto(v.to_owned()));
        let filename = meta.get("filename").map(|s| s.to_owned());

        let params = stream.parameters();
        let codec = CodecId(params.id());

        let is_attached_pic =
            || unsafe { ((*stream.as_ptr()).disposition & ffi::AV_DISPOSITION_ATTACHED_PIC) != 0 };
        let is_font = || codec.is_font() || is_font_filename(&filename);

        let ty = match params.medium() {
            Type::Audio => StreamType::Audio,
            Type::Subtitle => StreamType::Subtitles,
            Type::Video if is_attached_pic() => StreamType::AttachedPic,
            Type::Video => StreamType::Video,
            Type::Attachment if is_font() => StreamType::AttachedFont,
            Type::Attachment if is_attached_pic() => StreamType::AttachedPic,
            Type::Attachment => StreamType::AttachedOther,
            _ => StreamType::Other,
        };

        let i = stream.index();
        let i_ty = idxs[ty];
        idxs[ty] += 1;

        Stream {
            ty,
            i,
            i_ty,
            codec,
            lang: Value::Auto(lang),
            title,
            filename,
        }
    });

    Ok(streams.collect())
}
