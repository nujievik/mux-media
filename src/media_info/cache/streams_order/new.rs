use super::{StreamsOrder, StreamsOrderItem, retiming::Retiming};
use crate::config::{
    MarkConfigDefaults, MarkConfigForceds, MarkConfigStreams, MarkConfigStreamsOrder,
};
use crate::media_info::*;
use crate::{ArcPathBuf, FxHashSet, Lang, LangCode, MediaInfo, Result, StreamType, display};
use log::warn;
use std::cmp::Ordering;

impl StreamsOrder {
    /// Tries construct [`StreamsOrder`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Not cached any media file in the [`MediaInfo`].
    /// ```
    /// use clap::Parser;
    /// use mux_media::{*, media_info::StreamsOrder};
    ///
    /// let cfg = Config::parse_from::<_, &str>([]);
    /// let mut mi = MediaInfo::new(&cfg, 0);
    /// StreamsOrder::new(&mut mi).unwrap_err();
    /// ```
    ///
    /// - Fails extract an media info.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn) and `exit_on_err` is `false`.
    ///
    /// - Warning: fails retiming any media.
    pub fn new(mi: &mut MediaInfo) -> Result<StreamsOrder> {
        if mi.cache.of_files.is_empty() {
            return Err(err!("not found any cached media file"));
        }

        let mut sources = sources(mi);
        let (attach_filenames, sorted_src_stream_key) =
            try_sorted_src_stream_key(mi, &mut sources)?;

        Ok(StreamsOrder {
            items: sorted_into_items(sources, sorted_src_stream_key),
            attach_filenames,
        })
    }

    /// Tries retime order streams (on matroska linked or manual --parts).
    ///
    /// # Errors
    /// - Fails retiming any **only if** `exit_on_err` is `true`.
    /// - Fails retiming all files.
    pub fn retiming(self, mi: &mut MediaInfo) -> Result<StreamsOrder> {
        retiming(self, mi)
    }
}

fn sources(mi: &mut MediaInfo) -> Vec<ArcPathBuf> {
    let mut sources: Vec<ArcPathBuf> = mi.cache.of_files.keys().cloned().collect();
    sources.sort(); // First sort by names
    sources
}

fn try_sorted_src_stream_key(
    mi: &mut MediaInfo,
    sources: &mut Vec<ArcPathBuf>,
) -> Result<(FxHashSet<String>, Vec<(usize, usize, OrderSortKey)>)> {
    let cfg = mi.cfg;
    let locale = cfg.locales.locale_streams();

    let mut streams_to_sort: Vec<(usize, usize, OrderSortKey)> = Vec::new();
    let mut attach_filenames: FxHashSet<String> = FxHashSet::default();

    for (i_src, src) in sources.iter().enumerate() {
        let streams = mi.try_take(MarkMediaInfoStreams, src)?;
        let path_configs = mi.path_configs(src);

        streams.iter().for_each(|stream| {
            let ty = stream.ty;

            let (i, cfg_streams) = cfg.stream_value(MarkConfigStreams, &path_configs, stream);
            if !cfg_streams.is_save(&i, &stream.lang) {
                return;
            }

            let lowercased_filename = if ty.is_attached() {
                match &stream.filename {
                    Some(fname) if !attach_filenames.contains(fname) => {
                        let _ = attach_filenames.insert(fname.clone());
                        Some(fname.to_lowercase())
                    }
                    Some(_) => return, // skip fname duplicates
                    None => None,
                }
            } else {
                None
            };

            let lang = &stream.lang;
            let is_signs = mi.is_signs(src, stream);

            let (i, defaults) = cfg.stream_value(MarkConfigDefaults, &path_configs, stream);
            let default = defaults.get(&i, &lang);
            let (i, forceds) = cfg.stream_value(MarkConfigForceds, &path_configs, stream);
            let forced = forceds.get(&i, &lang);

            let (i, cfg_order) = cfg.stream_value(MarkConfigStreamsOrder, &path_configs, stream);
            let cfg_value = cfg_order.get(&i, &lang);

            let key = OrderSortKey::new(
                cfg_value,
                ty,
                default,
                forced,
                is_signs,
                lang,
                locale,
                lowercased_filename,
            );

            streams_to_sort.push((i_src, stream.i, key))
        });

        mi.set(MarkMediaInfoStreams, src, streams);
    }

    streams_to_sort.sort_by(|a, b| a.2.cmp(&b.2));

    Ok((attach_filenames, streams_to_sort))
}

fn sorted_into_items(
    sources: Vec<ArcPathBuf>,
    sorted_src_stream_key: Vec<(usize, usize, OrderSortKey)>,
) -> Vec<StreamsOrderItem> {
    let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(sorted_src_stream_key.len());
    let mut src_numbers = vec![Option::<usize>::None; sources.len()];
    let mut src_num = 0usize;

    for (i_src, i_stream, sort_key) in sorted_src_stream_key {
        let (num, is_first_entry) = num_is_first(i_src, &mut src_numbers, &mut src_num);

        items.push(StreamsOrderItem {
            ty: sort_key.ty,
            is_signs: sort_key.is_signs,
            key: sources[i_src].clone(),
            key_i_stream: i_stream,
            src: None,
            i_stream,
            src_num: num,
            is_first_entry,
            lowercased_filename: sort_key.lowercased_filename,
            is_font_file: false,
        })
    }

    items
}

fn retiming(mut order: StreamsOrder, mi: &mut MediaInfo) -> Result<StreamsOrder> {
    let exit_on_err = mi.cfg.exit_on_err;

    let rtm = match Retiming::try_new(mi, &order) {
        Ok(rtm) => rtm,
        Err(e) if e.code() == 0 => return Ok(order),
        Err(e) => return Err(e),
    };

    let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(order.len());

    let mut src_numbers = vec![Option::<usize>::None; order.len()];
    let mut src_num = 0usize;

    for (i, mut item) in order.items.into_iter().enumerate() {
        if matches!(
            item.ty,
            StreamType::Video | StreamType::Audio | StreamType::Subtitles
        ) {
            match rtm.try_any(i, &item) {
                Ok(retimed) => {
                    item.src = Some(retimed.src);
                    item.i_stream = retimed.i_stream;
                }
                Err(e) if exit_on_err => return Err(e),
                Err(e) => {
                    warn!(
                        "fail retime '{}' stream {}: {}. Skipping",
                        display(&item.key),
                        item.i_stream,
                        e
                    );
                    continue;
                }
            }
        }

        let (num, is_first) = num_is_first(i, &mut src_numbers, &mut src_num);
        item.src_num = num;
        item.is_first_entry = is_first;

        items.push(item)
    }

    order.items = items;
    mi.chapters = Some(rtm.into_chapters_time_ranges());

    Ok(order)
}

fn num_is_first(
    i_src: usize,
    src_numbers: &mut Vec<Option<usize>>,
    src_num: &mut usize,
) -> (usize, bool) {
    match src_numbers[i_src] {
        Some(n) => (n, false),
        None => {
            let n = *src_num;
            src_numbers[i_src] = Some(n);
            *src_num += 1;
            (n, true)
        }
    }
}

pub(crate) struct OrderSortKey {
    cfg_value: u8,
    ty: StreamType,
    default: u8,
    forced: u8,
    is_signs: bool,
    lang: u8,
    lowercased_filename: Option<String>,
}

impl OrderSortKey {
    pub(crate) fn new(
        cfg_value: Option<u8>,
        ty: StreamType,
        default: Option<bool>,
        forced: Option<bool>,
        is_signs: bool,
        lang: &Lang,
        locale: LangCode,
        lowercased_filename: Option<String>,
    ) -> Self {
        let cfg_value = cfg_value.unwrap_or(255);

        let flag_order = |flag: Option<bool>| match flag {
            Some(true) => 0,
            None => 1,
            Some(false) => 2,
        };

        let default = flag_order(default);
        let forced = flag_order(forced);

        let lang = match lang {
            Lang::Code(c) if c == &locale => 0,
            Lang::Code(LangCode::Und) => 1,
            Lang::Code(LangCode::Jpn) => 3,
            _ => 2,
        };

        Self {
            cfg_value,
            ty,
            default,
            forced,
            is_signs,
            lang,
            lowercased_filename,
        }
    }
}

impl PartialEq for OrderSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cfg_value == other.cfg_value
            && self.ty == other.ty
            && self.default == other.default
            && self.forced == other.forced
            && self.is_signs == other.is_signs
            && self.lang == other.lang
            && &self.lowercased_filename == &other.lowercased_filename
    }
}
impl Eq for OrderSortKey {}

impl PartialOrd for OrderSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.cfg_value,
            self.ty,
            self.default,
            self.forced,
            other.is_signs,
            self.lang,
            &self.lowercased_filename,
        )
            .cmp(&(
                other.cfg_value,
                other.ty,
                other.default,
                other.forced,
                self.is_signs,
                other.lang,
                &other.lowercased_filename,
            ))
    }
}
