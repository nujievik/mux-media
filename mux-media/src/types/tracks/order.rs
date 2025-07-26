use crate::{
    ArcPathBuf, LangCode, MediaInfo, MkvmergeArg, MuxError, TargetGroup, ToMkvmergeArgs, TrackID,
    TrackType,
    markers::{
        MCDefaultTFlags, MCEnabledTFlags, MCForcedTFlags, MCLocale, MISavedTracks, MITILang,
        MITargetGroup, MITargets,
    },
    mkvmerge_arg, to_mkvmerge_args, unmut,
};
use std::{cmp::Ordering, collections::HashMap, path::Path};

/// Sorts tracks of media files.
///
/// Stores:
/// - A vector of media file paths.
/// - A list of tuples: `(media_index, track_number, track_type)`,
///   where `media_index` refers to the index in the `media` vector.
///
/// # Sorting Priority
///
/// Tracks are sorted by the following rules (from highest to lowest priority):
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
///     # let args = [Path::new("-i"), &dir];
///     # let mux_config = MuxConfig::try_from_args(args).unwrap();
///     let video = ArcPathBuf::from(dir.join("3.mkv"));
///     let audio = ArcPathBuf::from(dir.join("1.ogg"));
///     let subs = ArcPathBuf::from(dir.join("2.srt"));
///     let mut mi = MediaInfo::from(&mux_config);
///     mi.try_insert(subs.clone()).unwrap();
///     mi.try_insert(audio.clone()).unwrap();
///     mi.try_insert(video.clone()).unwrap();
///     let order = TrackOrder::try_from(&mut mi).unwrap();
///     let (media, idxs) = (order.media, order.i_track_type);
///
///     assert_eq!(&video, &media[idxs[0].0]);
///     assert_eq!(&audio, &media[idxs[1].0]);
///     assert_eq!(&subs, &media[idxs[2].0]);
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
///     let order = TrackOrder::try_from(&mut mi).unwrap();
///     let (media, idxs) = (order.media, order.i_track_type);
///
///     assert_eq!(&second, &media[idxs[0].0]);
///     assert_eq!(&first, &media[idxs[1].0]);
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
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let (media, idxs) = (order.media, order.i_track_type);
///     #
///     assert_eq!(&second, &media[idxs[0].0]);
///     assert_eq!(&first, &media[idxs[1].0]);
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
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let (media, idxs) = (order.media, order.i_track_type);
///     #
///     assert_eq!(&second, &media[idxs[0].0]);
///     assert_eq!(&first, &media[idxs[1].0]);
///     ```
///
/// 5. `TargetGroup`:
///     - `Signs`
///     - Other group.
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
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let (media, idxs) = (order.media, order.i_track_type);
///     #
///     assert_eq!(&second, &media[idxs[0].0]);
///     assert_eq!(&first, &media[idxs[1].0]);
///     ```
///
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
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let (media, idxs) = (order.media, order.i_track_type);
///     #
///     assert_eq!(&second, &media[idxs[0].0]);
///     assert_eq!(&first, &media[idxs[1].0]);
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
///     # let mut mi = MediaInfo::from(&mux_config);
///     #
///     # mi.try_insert(first.clone()).unwrap();
///     # mi.try_insert(second.clone()).unwrap();
///     # let order = TrackOrder::try_from(&mut mi).unwrap();
///     # let (media, idxs) = (order.media, order.i_track_type);
///     #
///     assert_eq!(&first, &media[idxs[0].0]);
///     assert_eq!(&second, &media[idxs[1].0]);
///     ```
pub struct TrackOrder {
    pub media: Vec<ArcPathBuf>,
    pub i_track_type: Vec<(usize, u64, TrackType)>,

    // Prevents direct initialization outside the module.
    #[allow(dead_code)]
    field: (),
}

impl TryFrom<&mut MediaInfo<'_>> for TrackOrder {
    type Error = MuxError;

    fn try_from(mi: &mut MediaInfo) -> Result<Self, Self::Error> {
        if mi.is_empty() {
            return Err("Not found any cached Media".into());
        }

        let mut media: Vec<ArcPathBuf> = mi.cache().of_files.keys().cloned().collect();
        media.sort(); // First sort by names

        let i_track_type = {
            let locale_lang = *mi.mux_config.field::<MCLocale>();
            let mut to_sort: Vec<(usize, u64, TrackType, OrderSortKey)> = Vec::new();

            for (i, path) in media.iter().enumerate() {
                let num_types: Vec<(u64, TrackType)> = mi
                    .try_get::<MISavedTracks>(path)?
                    .iter()
                    .flat_map(|(tt, nums)| nums.iter().map(move |num| (*num, tt)))
                    .collect();

                let target_group = *mi.try_get::<MITargetGroup>(path)?;
                let targets = unmut!(@try, mi, MITargets, path)?;

                let defaults = mi.mux_config.trg_field::<MCDefaultTFlags>(targets);
                let forceds = mi.mux_config.trg_field::<MCForcedTFlags>(targets);
                let enableds = mi.mux_config.trg_field::<MCEnabledTFlags>(targets);

                for (num, ttype) in num_types {
                    let tid = TrackID::Num(num);
                    let lang = *mi.try_get_ti::<MITILang>(path, num)?;
                    let lang_tid = TrackID::Lang(lang);

                    let default = defaults.get(&tid).or_else(|| defaults.get(&lang_tid));
                    let forced = forceds.get(&tid).or_else(|| forceds.get(&lang_tid));
                    let enabled = enableds.get(&tid).or_else(|| enableds.get(&lang_tid));

                    let key = OrderSortKey::new(
                        ttype,
                        default,
                        forced,
                        enabled,
                        target_group,
                        lang,
                        locale_lang,
                    );

                    to_sort.push((i, num, ttype, key));
                }
            }

            to_sort.sort_by(|a, b| a.3.cmp(&b.3));

            to_sort
                .into_iter()
                .map(|(i, track, ttype, _)| (i, track, ttype))
                .collect()
        };

        Ok(Self {
            media,
            i_track_type,
            field: (),
        })
    }
}

mkvmerge_arg!(TrackOrder, "--track-order");

impl ToMkvmergeArgs for TrackOrder {
    fn to_mkvmerge_args(&self, _mi: &mut MediaInfo, _path: &Path) -> Vec<String> {
        let mut i_to_fid: HashMap<usize, usize> = HashMap::new();
        let mut max_fid: usize = 0;

        let order_arg: String = self
            .i_track_type
            .iter()
            .map(|(i, num, _)| {
                let fid = match i_to_fid.get(i) {
                    Some(fid) => *fid,
                    None => {
                        let fid = max_fid;
                        i_to_fid.insert(*i, fid);
                        max_fid += 1;
                        fid
                    }
                };
                format!("{}:{}", fid, num)
            })
            .collect::<Vec<_>>()
            .join(",");

        vec![Self::MKVMERGE_ARG.into(), order_arg]
    }

    to_mkvmerge_args!(@fn_os);
}

struct OrderSortKey {
    track_type: u8,
    default: u8,
    forced: u8,
    enabled: u8,
    target_group: u8,
    lang: u8,
}

impl OrderSortKey {
    fn new(
        track_type: TrackType,
        default: Option<bool>,
        forced: Option<bool>,
        enabled: Option<bool>,
        target_group: TargetGroup,
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

        let target_group = match target_group {
            TargetGroup::Signs => 0,
            _ => 1,
        };

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
            target_group,
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
            && self.target_group == other.target_group
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
            self.target_group,
            self.lang,
        )
            .cmp(&(
                other.track_type,
                other.default,
                other.forced,
                other.enabled,
                other.target_group,
                other.lang,
            ))
    }
}
