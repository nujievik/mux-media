use super::{TrackOrder, TrackOrderItem};
use crate::{
    ArcPathBuf, LangCode, MediaInfo, MuxError, Result, Retiming, TrackType, immut, IsDefault,
    markers::{
        MCDefaultTrackFlags, MCEnabledTrackFlags, MCForcedTrackFlags, MISavedTracks, MITIItSigns,
        MITITrackIDs, MITargets,
    },
};
use log::warn;
use rayon::prelude::*;
use std::cmp::Ordering;

impl TryFrom<&mut MediaInfo<'_>> for TrackOrder {
    type Error = MuxError;

    /// Tries construct [`TrackOrder`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Not cached any media file in the [`MediaInfo`].
    /// ```
    /// # use mux_media::*;
    /// # let cfg = MuxConfig::default();
    /// # let mut mi = MediaInfo::from(&cfg);
    /// assert!(mi.is_no_files());
    /// assert!(TrackOrder::try_from(&mut mi).is_err());
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
    fn try_from(mi: &mut MediaInfo) -> Result<TrackOrder> {
        if mi.is_no_files() {
            return Err("Not found any cached media file".into());
        }

        let raw_media = raw_media(mi);
        let raw_ittk = try_raw_i_track_type_key(mi, &raw_media)?;
        let raw_items = raw_items(raw_media, raw_ittk);

        return try_order(mi, raw_items);

        fn raw_media(mi: &mut MediaInfo) -> Vec<ArcPathBuf> {
            let mut raw_media: Vec<ArcPathBuf> = mi.cache.of_files.keys().cloned().collect();
            raw_media.sort(); // First sort by names
            raw_media
        }

        fn try_raw_i_track_type_key(
            mi: &mut MediaInfo,
            raw_media: &Vec<ArcPathBuf>,
        ) -> Result<Vec<(usize, u64, TrackType, OrderSortKey)>> {
            let locale = mi.cfg.locale;
            let mut ittk: Vec<(usize, u64, TrackType, OrderSortKey)> = Vec::new();

            for (i, m) in raw_media.iter().enumerate() {
                let tracks = mi.try_take::<MISavedTracks>(m)?;
                let targets = immut!(@try, mi, MITargets, m)?;

                let defaults = mi.cfg.target(MCDefaultTrackFlags, targets);
                let forceds = mi.cfg.target(MCForcedTrackFlags, targets);
                let enableds = mi.cfg.target(MCEnabledTrackFlags, targets);

                for (ty, num) in tracks
                    .iter()
                    .flat_map(|(ty, nums)| nums.iter().map(move |num| (ty, *num)))
                {
                    let it_signs = matches!(ty, TrackType::Sub)
                        && *mi.get_ti::<MITIItSigns>(m, num).unwrap_or(&false);
                    let tids = mi.try_get_ti::<MITITrackIDs>(m, num)?;
                    let lang = LangCode::from(&tids[1]);

                    let default = defaults.get(&tids[0]).or_else(|| defaults.get(&tids[1]));
                    let forced = forceds.get(&tids[0]).or_else(|| forceds.get(&tids[1]));
                    let enabled = enableds.get(&tids[0]).or_else(|| enableds.get(&tids[1]));

                    let key =
                        OrderSortKey::new(ty, default, forced, enabled, it_signs, lang, locale);

                    ittk.push((i, num, ty, key));
                }

                mi.set::<MISavedTracks>(m, tracks);
            }

            ittk.sort_by(|a, b| a.3.cmp(&b.3));
            Ok(ittk)
        }

        fn raw_items(
            raw_media: Vec<ArcPathBuf>,
            ittk: Vec<(usize, u64, TrackType, OrderSortKey)>,
        ) -> Vec<TrackOrderItem> {
            let mut items: Vec<TrackOrderItem> = Vec::with_capacity(ittk.len());
            let mut numbers = vec![Option::<u64>::None; raw_media.len()];
            let mut num = 0u64;

            for (i, track, ty, _) in ittk {
                let (is_first_entry, number) = is_first_num(i, &mut numbers, &mut num);
                items.push(TrackOrderItem {
                    media: raw_media[i].clone(),
                    number,
                    is_first_entry,
                    track,
                    ty,
                    retimed: None,
                })
            }

            return items;

            fn is_first_num(
                i: usize,
                numbers: &mut Vec<Option<u64>>,
                num: &mut u64,
            ) -> (bool, u64) {
                let first = numbers[i].is_none();
                let num = match numbers[i] {
                    Some(n) => n,
                    None => {
                        let n = *num;
                        numbers[i] = Some(n);
                        *num += 1;
                        n
                    }
                };
                (first, num)
            }
        }

        fn try_order(mi: &mut MediaInfo, raw_items: Vec<TrackOrderItem>) -> Result<TrackOrder> {
            let exit_on_err = mi.cfg.exit_on_err;
            let mut order = TrackOrder(raw_items);

            if !mi.cfg.muxer.is_default() {
                return Ok(order);
            }

            let rtm = match Retiming::try_new(mi, &order) {
                Ok(rtm) => rtm,
                Err(e) if e.code == 0 => return Ok(order),
                Err(e) => return Err(e),
            };

            let i_retimed = order
                .0
                .par_iter()
                .enumerate()
                .filter_map(|(i, m)| match rtm.try_any(i, &m.media, m.track, m.ty) {
                    Ok(vec) => Some(Ok((i, vec))),
                    Err(e) if exit_on_err => Some(Err(e)),
                    Err(e) => {
                        warn!(
                            "Fail retime '{}' track {}: {}. Skipping",
                            m.media.display(),
                            m.track,
                            e
                        );
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            if i_retimed.is_empty() {
                return Err("Not save any track".into());
            }

            if i_retimed.len() == order.len() {
                i_retimed.into_iter().for_each(|(i, rtm)| {
                    let item = &mut order.0[i];
                    item.is_first_entry = true;
                    item.retimed = Some(rtm);
                });
                return Ok(order);
            }

            let items = i_retimed
                .into_iter()
                .enumerate()
                .map(|(num, (i, rtm))| TrackOrderItem {
                    media: order[i].media.clone(),
                    number: num as u64,
                    is_first_entry: true,
                    track: order[i].track,
                    ty: order[i].ty,
                    retimed: Some(rtm),
                })
                .collect();

            Ok(TrackOrder(items))
        }
    }
}

struct OrderSortKey {
    track_type: u8,
    default: u8,
    forced: u8,
    enabled: u8,
    it_signs: u8,
    lang: u8,
}

impl OrderSortKey {
    fn new(
        track_type: TrackType,
        default: Option<bool>,
        forced: Option<bool>,
        enabled: Option<bool>,
        it_signs: bool,
        lang: LangCode,
        locale_lang: LangCode,
    ) -> Self {
        let track_type = match track_type {
            TrackType::Video => 0,
            TrackType::Audio => 1,
            TrackType::Sub => 2,
            _ => 3,
        };

        let flag_order = |flag: Option<bool>| match flag {
            Some(true) => 0,
            None => 1,
            Some(false) => 2,
        };

        let default = flag_order(default);
        let forced = flag_order(forced);
        let enabled = flag_order(enabled);

        let it_signs = if it_signs { 0 } else { 1 };

        let lang = match lang {
            _ if lang == locale_lang => 0,
            LangCode::Und => 1,
            LangCode::Jpn => 3,
            _ => 2,
        };

        Self {
            track_type,
            default,
            forced,
            enabled,
            it_signs,
            lang,
        }
    }
}

impl PartialEq for OrderSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.track_type == other.track_type
            && self.default == other.default
            && self.forced == other.forced
            && self.enabled == other.enabled
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
        (
            self.track_type,
            self.default,
            self.forced,
            self.enabled,
            self.it_signs,
            self.lang,
        )
            .cmp(&(
                other.track_type,
                other.default,
                other.forced,
                other.enabled,
                other.it_signs,
                other.lang,
            ))
    }
}
