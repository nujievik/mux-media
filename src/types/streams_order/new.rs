use super::{StreamsOrder, StreamsOrderItem};
use crate::config::{
    MarkConfigDefaults, MarkConfigForceds, MarkConfigStreams, MarkConfigStreamsOrder,
};
use crate::media_info::*;
use crate::{
    ArcPathBuf, FxHashSet, Lang, LangCode, MediaInfo, Result, Retiming, StreamType, display,
};
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
    /// use mux_media::*;
    ///
    /// let cfg = Config::parse_from::<_, &str>([]);
    /// let mut mi = MediaInfo::new(&cfg, 0);
    /// StreamsOrder::new(&mut mi).unwrap_err();
    /// ```
    ///
    /// - Fails extract an media info.
    ///
    /// - Fails retiming any **only if** `exit_on_err` is `true`.
    ///
    /// - Fais retiming all files.
    ///
    /// # Logging
    ///
    /// - **Only if** [`log`] is initialized with at least [`LevelFilter::Warn`](
    ///   log::LevelFilter::Warn) and `exit_on_err` is `false`.
    ///
    /// - Warning: fails retiming any media.
    pub fn new(mi: &mut MediaInfo) -> Result<StreamsOrder> {
        if mi.cache.of_files.is_empty() {
            Err(err!("not found any cached media file"))
        } else {
            let sources = sources(mi);
            let sorted_src_stream_key = try_sorted_src_stream_key(mi, &sources)?;
            let items = items(sources, sorted_src_stream_key);
            try_order(mi, items)
        }
    }
}

fn sources(mi: &mut MediaInfo) -> Vec<ArcPathBuf> {
    let mut sources: Vec<ArcPathBuf> = mi.cache.of_files.keys().cloned().collect();
    sources.sort(); // First sort by names
    sources
}

fn try_sorted_src_stream_key(
    mi: &mut MediaInfo,
    sources: &Vec<ArcPathBuf>,
) -> Result<Vec<(usize, usize, OrderSortKey)>> {
    let cfg = mi.cfg;
    let locale = cfg.locale;

    let mut streams_to_sort: Vec<(usize, usize, OrderSortKey)> = Vec::new();
    let mut attach_filenames: FxHashSet<String> = FxHashSet::default();

    for (i_src, src) in sources.iter().enumerate() {
        let streams = mi.try_take(MarkMediaInfoStreams, src)?;
        let target_paths = mi.try_take(MarkMediaInfoTargetPaths, src)?;

        streams.iter().for_each(|stream| {
            let ty = stream.ty;
            // skip temp dummy sub
            if ty.is_sub()
                && stream.i == 0
                && src.parent().map_or(false, |p| p == cfg.output.temp_dir())
            {
                return;
            }

            let (i, cfg_streams) = cfg.stream_val(MarkConfigStreams, &target_paths, stream);
            if !cfg_streams.is_save(&i, &stream.lang) {
                return;
            }

            let lang = &stream.lang;
            let it_signs = mi.it_signs(src, stream);

            let (i, defaults) = cfg.stream_val(MarkConfigDefaults, &target_paths, stream);
            let default = defaults.get(&i, &lang);
            let (i, forceds) = cfg.stream_val(MarkConfigForceds, &target_paths, stream);
            let forced = forceds.get(&i, &lang);

            let (i, cfg_order) = cfg.stream_val(MarkConfigStreamsOrder, &target_paths, stream);
            let cfg_value = cfg_order.get(&i, &lang);

            let fname = if ty.is_an_attach() {
                match &stream.filename {
                    Some(fname) if !attach_filenames.contains(fname) => {
                        let _ = attach_filenames.insert(fname.clone());
                        Some(fname.to_lowercase())
                    }
                    Some(_) | None => None,
                }
            } else {
                None
            };

            let key = OrderSortKey::new(
                cfg_value, ty, default, forced, it_signs, lang, locale, fname,
            );

            streams_to_sort.push((i_src, stream.i, key))
        });

        mi.set(MarkMediaInfoStreams, src, streams);
        mi.set(MarkMediaInfoTargetPaths, src, target_paths);
    }

    streams_to_sort.sort_by(|a, b| a.2.cmp(&b.2));
    Ok(streams_to_sort)
}

fn items(
    sources: Vec<ArcPathBuf>,
    sorted_src_stream_key: Vec<(usize, usize, OrderSortKey)>,
) -> Vec<StreamsOrderItem> {
    let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(sorted_src_stream_key.len());
    let mut src_numbers = vec![Option::<usize>::None; sources.len()];
    let mut src_num = 0usize;

    for (i_src, i_stream, sort_key) in sorted_src_stream_key {
        let (num, is_first) = num_is_first(i_src, &mut src_numbers, &mut src_num);

        items.push(StreamsOrderItem {
            ty: sort_key.ty,
            key: sources[i_src].clone(),
            key_i_stream: i_stream,
            src: None,
            i_stream,
            src_num: num,
            is_first_entry: is_first,
        })
    }

    items
}

fn try_order(mi: &mut MediaInfo, items: Vec<StreamsOrderItem>) -> Result<StreamsOrder> {
    let exit_on_err = mi.cfg.exit_on_err;
    let order = StreamsOrder(items);

    let rtm = match Retiming::try_new(mi, &order) {
        Ok(rtm) => rtm,
        Err(e) if e.code() == 0 => return Ok(order),
        Err(e) => return Err(e),
    };

    let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(order.0.len());

    let mut src_numbers = vec![Option::<usize>::None; order.len()];
    let mut src_num = 0usize;

    for (i, mut item) in order.0.into_iter().enumerate() {
        if item.ty.is_track() {
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

    Ok(StreamsOrder(items))
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

struct OrderSortKey {
    cfg_value: u8,
    ty: StreamType,
    default: u8,
    forced: u8,
    it_signs: u8,
    lang: u8,
    fname: Option<String>,
}

impl OrderSortKey {
    fn new(
        cfg_value: Option<u8>,
        ty: StreamType,
        default: Option<bool>,
        forced: Option<bool>,
        it_signs: bool,
        lang: &Lang,
        locale: LangCode,
        fname: Option<String>,
    ) -> Self {
        let cfg_value = cfg_value.unwrap_or(255);

        let flag_order = |flag: Option<bool>| match flag {
            Some(true) => 0,
            None => 1,
            Some(false) => 2,
        };

        let default = flag_order(default);
        let forced = flag_order(forced);

        let it_signs = if it_signs { 0 } else { 1 };

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
            it_signs,
            lang,
            fname,
        }
    }
}

impl PartialEq for OrderSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cfg_value == other.cfg_value
            && self.ty == other.ty
            && self.default == other.default
            && self.forced == other.forced
            && self.it_signs == other.it_signs
            && self.lang == other.lang
            && &self.fname == &other.fname
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
            self.it_signs,
            self.lang,
            &self.fname,
        )
            .cmp(&(
                other.cfg_value,
                other.ty,
                other.default,
                other.forced,
                other.it_signs,
                other.lang,
                &other.fname,
            ))
    }
}
