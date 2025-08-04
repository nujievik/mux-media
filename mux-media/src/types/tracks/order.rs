mod to_args;

use crate::{
    ArcPathBuf, LangCode, MediaInfo, MuxError, TrackType, immut,
    markers::{
        MCDefaultTFlags, MCEnabledTFlags, MCForcedTFlags, MCLocale, MISavedTracks, MITIItSigns,
        MITITrackIDs, MITargets,
    },
};
use std::cmp::Ordering;

/// Sorts media and tracks of media files.
///
/// # Stores
///
/// - A vector of media file paths.
///
/// - A vector of tuples: `(media_index, track_number, track_type)`,
///   where `media_index` refers to the index in the `media` vector.
///
/// # Sorting Priority
///
/// Sorts by the following rules (from highest to lowest priority):
///
/// 1. `TrackType`:
///     - `Video`
///     - `Audio`
///     - `Sub`
///     - `Button`
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("1");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     # let args = [Path::new("-i"), &dir];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     let video = ArcPathBuf::from(dir.join("3.mkv"));
///     let audio = ArcPathBuf::from(dir.join("1.ogg"));
///     let subs = ArcPathBuf::from(dir.join("2.srt"));
///
///     let mut mi = MediaInfo::from(&mux_config);
///     mi.try_insert(subs.clone()).unwrap();
///     mi.try_insert(audio.clone()).unwrap();
///     mi.try_insert(video.clone()).unwrap();
///
///     let order = TrackOrder::try_from(&mut mi).unwrap();
///     let media = order.media();
///
///     assert_eq!(&video, &media[0]);
///     assert_eq!(&audio, &media[1]);
///     assert_eq!(&subs, &media[2]);
///     ```
///
/// 2. `TFlagType::Default`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     let first = ArcPathBuf::from(dir.join("1.srt"));
///     let second = ArcPathBuf::from(dir.join("2.srt"));
///
///     let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--defaults"), p("true")];
///     let mux_config = MuxConfig::try_from_args(args).unwrap();
///
///     let mut mi = MediaInfo::from(&mux_config);
///     mi.try_insert(first.clone()).unwrap();
///     mi.try_insert(second.clone()).unwrap();
///
///     let order = TrackOrder::try_from(&mut mi).unwrap();
///     let media = order.media();
///
///     assert_eq!(&second, &media[0]);
///     assert_eq!(&first, &media[1]);
///     ```
///
/// 3. `TFlagType::Forced`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--forceds"), p("true")];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     # let mut mi = MediaInfo::from(&mux_config);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let media = order.media();
///     #
///     assert_eq!(&second, &media[0]);
///     assert_eq!(&first, &media[1]);
///     ```
///
/// 4. `TFlagType::Enabled`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir, p("--target"), &second, p("--enableds"), p("true")];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     # let mut mi = MediaInfo::from(&mux_config);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let media = order.media();
///     #
///     assert_eq!(&second, &media[0]);
///     assert_eq!(&first, &media[1]);
///     ```
///
/// 5.  It `Sub` track and signs:
///     - `true`
///     - `false`.
///
///     Its affected only `Sub` tracks if they has same 1-4.
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("3");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     let first = ArcPathBuf::from(dir.join("1.srt"));
///     let second = ArcPathBuf::from(dir.join("1_signs.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let args = [p("-i"), &dir];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     # let mut mi = MediaInfo::from(&mux_config);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let media = order.media();
///     #
///     assert_eq!(&second, &media[0]);
///     assert_eq!(&first, &media[1]);
///     ```
///
/// 6. Track language `LangCode`:
///     - `locale` language
///     - `Und` (undefined language)
///     - Other languages (excluding `Jpn`)
///     - `Jpn` (Japanese)
///
///     ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     # let p: fn(&str) -> &Path = Path::new;
///     let mut args = vec![p("-i"), &dir, p("--locale"), p("eng")];
///     args.extend([p("--target"), &second, p("--langs"), p("eng")]);
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     # let mut mi = MediaInfo::from(&mux_config);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let media = order.media();
///     #
///     assert_eq!(&second, &media[0]);
///     assert_eq!(&first, &media[1]);
///     ```
///
/// 7. Path name.
///
///    ```
///     # use mux_media::*;
///     # use std::path::Path;
///     #
///     # let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
///     #     .join("tests")
///     #     .join("test_data")
///     #     .join("order")
///     #     .join("2");
///     # let dir = ensure_long_path_prefix(dir);
///     #
///     // Hidden steps same as in 2.
///     #
///     # let first = ArcPathBuf::from(dir.join("1.srt"));
///     # let second = ArcPathBuf::from(dir.join("2.srt"));
///     #
///     let args = [Path::new("-i"), &dir];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     #
///     # let mut mi = MediaInfo::from(&mux_config);
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     #
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let media = order.media();
///     #
///     assert_eq!(&first, &media[0]);
///     assert_eq!(&second, &media[1]);
///     ```
#[derive(Clone)]
pub struct TrackOrder {
    media: Vec<ArcPathBuf>,
    i_track_type: Vec<(usize, u64, TrackType)>,
}

impl TrackOrder {
    /// Consumes [`Self`], returning its fields.
    #[inline(always)]
    pub fn into_media_and_i_track_type(self) -> (Vec<ArcPathBuf>, Vec<(usize, u64, TrackType)>) {
        (self.media, self.i_track_type)
    }

    /// Returns the sorted media.
    #[inline(always)]
    pub fn media(&self) -> &Vec<ArcPathBuf> {
        &self.media
    }

    /// Returns the sorted `(media_index, track_number, track_type)`.
    #[inline(always)]
    pub fn i_track_type(&self) -> &Vec<(usize, u64, TrackType)> {
        &self.i_track_type
    }
}

impl TryFrom<&mut MediaInfo<'_>> for TrackOrder {
    type Error = MuxError;

    fn try_from(mi: &mut MediaInfo) -> Result<Self, Self::Error> {
        if mi.is_empty() {
            return Err("Not found any cached Media".into());
        }

        let mut raw_media: Vec<ArcPathBuf> = mi.cache().of_files.keys().cloned().collect();
        raw_media.sort(); // First sort by names

        let raw_i_track_type: Vec<(usize, u64, TrackType)> = {
            let locale_lang = *mi.mux_config.field::<MCLocale>();
            let mut to_sort: Vec<(usize, u64, TrackType, OrderSortKey)> = Vec::new();

            for (i, path) in raw_media.iter().enumerate() {
                let tracks = mi.try_take::<MISavedTracks>(path)?;

                let targets = immut!(@try, mi, MITargets, path)?;

                let defaults = mi.mux_config.trg_field::<MCDefaultTFlags>(targets);
                let forceds = mi.mux_config.trg_field::<MCForcedTFlags>(targets);
                let enableds = mi.mux_config.trg_field::<MCEnabledTFlags>(targets);

                for (ty, num) in tracks
                    .iter()
                    .flat_map(|(ty, nums)| nums.iter().map(move |num| (ty, *num)))
                {
                    let it_signs = matches!(ty, TrackType::Sub)
                        && *mi.get_ti::<MITIItSigns>(path, num).unwrap_or(&false);
                    let tids = mi.try_get_ti::<MITITrackIDs>(path, num)?;
                    let lang = LangCode::from(&tids[1]);

                    let default = defaults.get(&tids[0]).or_else(|| defaults.get(&tids[1]));
                    let forced = forceds.get(&tids[0]).or_else(|| forceds.get(&tids[1]));
                    let enabled = enableds.get(&tids[0]).or_else(|| enableds.get(&tids[1]));

                    let key = OrderSortKey::new(
                        ty,
                        default,
                        forced,
                        enabled,
                        it_signs,
                        lang,
                        locale_lang,
                    );

                    to_sort.push((i, num, ty, key));
                }

                mi.set::<MISavedTracks>(path, tracks);
            }

            to_sort.sort_by(|a, b| a.3.cmp(&b.3));

            to_sort
                .into_iter()
                .map(|(i, track, ttype, _)| (i, track, ttype))
                .collect()
        };

        let mut media: Vec<ArcPathBuf> = Vec::with_capacity(raw_media.len());

        let mut i_track_type: Vec<(usize, u64, TrackType)> =
            Vec::with_capacity(raw_i_track_type.len());

        let mut old_i_to_new: Vec<Option<usize>> = vec![None; raw_media.len()];
        let mut new_i = 0;

        raw_i_track_type.into_iter().for_each(|(old_i, track, tt)| {
            let i = match old_i_to_new[old_i] {
                Some(i) => i,

                None => {
                    old_i_to_new[old_i] = Some(new_i);

                    media.push(raw_media[old_i].clone());

                    let i = new_i;
                    new_i += 1;

                    i
                }
            };

            i_track_type.push((i, track, tt));
        });

        Ok(Self {
            media,
            i_track_type,
        })
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
            TrackType::Button => 3,
            _ => 4,
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
