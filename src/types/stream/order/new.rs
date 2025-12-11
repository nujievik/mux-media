use super::{StreamsOrder, StreamsOrderItem};
use crate::{
    ArcPathBuf, LangCode, MediaInfo, Muxer, Result, RetimedStream, Retiming, StreamType,
    StreamsSupported, i18n::logs, markers::*,
};
use log::warn;
use rayon::prelude::*;
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
        return if mi.cache.of_files.is_empty() {
            Err(err!("Not found any cached media file"))
        } else {
            let sources = sources(mi);
            let sorted_src_stream_ty = try_sorted_src_stream_ty(mi, &sources)?;
            let items = items(mi.cfg.muxer, sources, sorted_src_stream_ty);
            try_order(mi, items)
        };

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
                let streams = mi.try_take(MIStreams, src)?;
                let target_paths = mi.try_take(MITargetPaths, src)?;

                streams.iter().for_each(|stream| {
                    let ty = stream.ty;
                    // skip temp dummy sub
                    if ty.is_sub()
                        && stream.i == 0
                        && src.parent().map_or(false, |p| p == cfg.output.temp_dir)
                    {
                        return;
                    }

                    let (i, cfg_streams) = cfg.stream_val(CfgStreams, &target_paths, stream);
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

                    let lang = *stream.lang;
                    let it_signs = mi.it_signs(src, stream);

                    let (i, defaults) = cfg.stream_val(CfgDefaults, &target_paths, stream);
                    let default = defaults.get(&i, &lang);
                    let (i, forceds) = cfg.stream_val(CfgForceds, &target_paths, stream);
                    let forced = forceds.get(&i, &lang);

                    let key = OrderSortKey::new(ty, default, forced, it_signs, lang, locale);
                    track_streams.push((i_src, stream.i, ty, key));
                });

                mi.set(MIStreams, src, streams);
                mi.set(MITargetPaths, src, target_paths);
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
            muxer: Muxer,
            sources: Vec<ArcPathBuf>,
            sorted_src_stream_ty: Vec<(usize, usize, StreamType)>,
        ) -> Vec<StreamsOrderItem> {
            let len = sorted_src_stream_ty.len();
            let mut items: Vec<StreamsOrderItem> = Vec::with_capacity(len);
            let mut src_numbers = vec![Option::<usize>::None; len];
            let mut src_num = 0usize;
            let mut sup = StreamsSupported::new(muxer);

            for (i_src, i_stream, ty) in sorted_src_stream_ty {
                if !sup.is_supported(ty) {
                    logs::warn_container_does_not_support(muxer, &sources[i_src], i_stream);
                    continue;
                }
                let (num, is_first) = num_is_first(i_src, &mut src_numbers, &mut src_num);

                items.push(StreamsOrderItem {
                    ty,
                    key: sources[i_src].clone(),
                    key_i_stream: i_stream,
                    src: None,
                    i_stream,
                    src_time: None,
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
                Err(e) if e.code == 0 => return Ok(order),
                Err(e) => return Err(e),
            };

            let mut i_retimed = order
                .0
                .par_iter()
                .enumerate()
                .filter_map(|(i, m)| {
                    if m.ty.is_track() {
                        match rtm.try_any(i, m) {
                            Ok(retimed) => Some(Ok((i, retimed))),
                            Err(e) if exit_on_err => Some(Err(e)),
                            Err(e) => {
                                warn!(
                                    "Fail retime '{}' stream {}: {}. Skipping",
                                    m.key.display(),
                                    m.i_stream,
                                    e
                                );
                                None
                            }
                        }
                    } else {
                        let retimed = RetimedStream {
                            i_stream: m.i_stream,
                            ..Default::default()
                        };
                        Some(Ok((i, retimed)))
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            if i_retimed.is_empty() {
                return Err(err!("Not save any track"));
            }

            i_retimed.sort_by(|a, b| a.0.cmp(&b.0));
            let mut src_numbers = vec![Option::<usize>::None; order.len()];
            let mut src_num = 0usize;

            let items: Vec<_> = i_retimed
                .into_iter()
                .map(|(i, rtm)| {
                    let item = &order.0[i];
                    let (num, is_first) = num_is_first(i, &mut src_numbers, &mut src_num);

                    StreamsOrderItem {
                        ty: item.ty,
                        key: item.key.clone(),
                        key_i_stream: item.key_i_stream,
                        src: rtm.src,
                        i_stream: rtm.i_stream,
                        src_time: rtm.src_time,
                        src_num: num,
                        is_first_entry: is_first,
                    }
                })
                .collect();

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
        lang: LangCode,
        locale_lang: LangCode,
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
            _ if lang == locale_lang => 0,
            LangCode::Und => 1,
            LangCode::Jpn => 3,
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
