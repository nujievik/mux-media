use super::{StreamsOrder, StreamsOrderItem};
use crate::config::{MarkConfigDefaults, MarkConfigForceds, MarkConfigStreams};
use crate::media_info::*;
use crate::{ArcPathBuf, Lang, LangCode, MediaInfo, Result, Retiming, StreamType, display};
use log::warn;
use std::{cmp::Ordering, collections::HashSet};

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
            Err(err!("Not found any cached media file"))
        } else {
            let sources = sources(mi);
            let sorted_src_stream_ty = try_sorted_src_stream_ty(mi, &sources)?;
            let items = items(sources, sorted_src_stream_ty);
            try_order(mi, items)
        }
    }
}

fn sources(mi: &mut MediaInfo) -> Vec<ArcPathBuf> {
    let mut sources: Vec<ArcPathBuf> = mi.cache.of_files.keys().cloned().collect();
    sources.sort(); // First sort by names
    sources
}

fn try_sorted_src_stream_ty(
    mi: &mut MediaInfo,
    sources: &Vec<ArcPathBuf>,
) -> Result<Vec<(usize, usize, StreamType)>> {
    let cfg = mi.cfg;
    let locale = cfg.locale;

    let mut track_streams: Vec<(usize, usize, StreamType, OrderSortKey)> = Vec::new();
    let mut attach_streams: Vec<(usize, usize, StreamType, Option<String>)> = Vec::new();
    let mut attach_names: HashSet<String> = HashSet::new();

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

            if ty.is_an_attach() {
                if match &stream.filename {
                    Some(s) if attach_names.contains(s) => false,
                    Some(_) => true,
                    None => true,
                } {
                    let fname = stream.filename.as_ref().map(|s| s.to_lowercase());
                    attach_streams.push((i_src, stream.i, ty, fname));

                    if let Some(s) = &stream.filename {
                        let _ = attach_names.insert(s.clone());
                    }
                }
                return;
            }

            let lang = &stream.lang;
            let it_signs = mi.it_signs(src, stream);

            let (i, defaults) = cfg.stream_val(MarkConfigDefaults, &target_paths, stream);
            let default = defaults.get(&i, &lang);
            let (i, forceds) = cfg.stream_val(MarkConfigForceds, &target_paths, stream);
            let forced = forceds.get(&i, &lang);

            let key = OrderSortKey::new(ty, default, forced, it_signs, lang, locale);
            track_streams.push((i_src, stream.i, ty, key));
        });

        mi.set(MarkMediaInfoStreams, src, streams);
        mi.set(MarkMediaInfoTargetPaths, src, target_paths);
    }

    track_streams.sort_by(|a, b| a.3.cmp(&b.3));
    attach_streams.sort_by(|a, b| a.2.cmp(&b.2).then(a.3.cmp(&b.3)));

    let mut streams: Vec<(usize, usize, StreamType)> =
        Vec::with_capacity(track_streams.len() + attach_streams.len());

    for (i_src, i_stream, ty, _) in track_streams {
        streams.push((i_src, i_stream, ty));
    }
    for (i_src, i_stream, ty, _) in attach_streams {
        streams.push((i_src, i_stream, ty));
    }

    Ok(streams)
}

fn items(
    sources: Vec<ArcPathBuf>,
    sorted_src_stream_ty: Vec<(usize, usize, StreamType)>,
) -> Vec<StreamsOrderItem> {
    let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(sorted_src_stream_ty.len());
    let mut src_numbers = vec![Option::<usize>::None; sources.len()];
    let mut src_num = 0usize;

    for (i_src, i_stream, ty) in sorted_src_stream_ty {
        let (num, is_first) = num_is_first(i_src, &mut src_numbers, &mut src_num);

        items.push(StreamsOrderItem {
            ty,
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
    ty: StreamType,
    default: u8,
    forced: u8,
    it_signs: u8,
    lang: u8,
}

impl OrderSortKey {
    fn new(
        ty: StreamType,
        default: Option<bool>,
        forced: Option<bool>,
        it_signs: bool,
        lang: &Lang,
        locale: LangCode,
    ) -> Self {
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
            ty,
            default,
            forced,
            it_signs,
            lang,
        }
    }
}

impl PartialEq for OrderSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
            && self.default == other.default
            && self.forced == other.forced
            && self.it_signs == other.it_signs
            && self.lang == other.lang
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
        (self.ty, self.default, self.forced, self.it_signs, self.lang).cmp(&(
            other.ty,
            other.default,
            other.forced,
            other.it_signs,
            other.lang,
        ))
    }
}
