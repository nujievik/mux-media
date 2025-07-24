use crate::{
    ArcPathBuf, LangCode, MediaInfo, MkvmergeArg, MuxError, TargetGroup, ToMkvmergeArgs, TrackID,
    TrackType,
    markers::{
        MCDefaultTFlags, MCEnabledTFlags, MCForcedTFlags, MCLocale, MISavedTracks, MITILang,
        MITargetGroup, MITargets,
    },
    mkvmerge_arg, to_mkvmerge_args, unmut_get,
};
use std::{cmp::Ordering, collections::HashMap, path::Path};

/// Sorts tracks of media files.
///
/// Stores:
/// - A vector of media file paths.
/// - A list of tuples: `(media_index, track_number, track_type)`,
///   where `media_index` refers to the index in the `paths` vector.
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
/// 2. `TFlagType::Default`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
/// 3. `TFlagType::Forced`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
/// 4. `TFlagType::Enabled`:
///     - User-defined `true`
///     - Auto
///     - User-defined `false`
/// 5. `TargetGroup`:
///     - `Signs`
///     - Other group.
///
///     Its affected only `Sub` tracks if they has same 1-4.
/// 6. Track language `LangCode`:
///     - `locale` language
///     - `Und` (undefined language)
///     - Other languages (excluding `Jpn`)
///     - `Jpn` (Japanese)
pub struct TrackOrder {
    pub paths: Vec<ArcPathBuf>,
    pub i_num_type: Vec<(usize, u64, TrackType)>,
}

impl TryFrom<&mut MediaInfo<'_>> for TrackOrder {
    type Error = MuxError;

    fn try_from(mi: &mut MediaInfo) -> Result<Self, Self::Error> {
        if mi.is_empty() {
            return Err("Not found any cached Media".into());
        }

        let mut paths: Vec<ArcPathBuf> = mi.get_cache().of_files.keys().cloned().collect();
        paths.sort(); // First sort by names

        let i_num_type = {
            let locale_lang = *mi.mc.get::<MCLocale>();
            let mut to_sort: Vec<(usize, u64, TrackType, OrderSortKey)> = Vec::new();

            for (i, path) in paths.iter().enumerate() {
                let num_types: Vec<(u64, TrackType)> = mi
                    .try_get::<MISavedTracks>(path)?
                    .iter()
                    .flat_map(|(tt, nums)| nums.iter().map(move |num| (*num, tt)))
                    .collect();

                let target_group = *mi.try_get::<MITargetGroup>(path)?;
                let targets = unmut_get!(@try, mi, MITargets, path)?;

                let defaults = mi.mc.get_trg::<MCDefaultTFlags>(targets);
                let forceds = mi.mc.get_trg::<MCForcedTFlags>(targets);
                let enableds = mi.mc.get_trg::<MCEnabledTFlags>(targets);

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
                .map(|(i, num, ttype, _)| (i, num, ttype))
                .collect()
        };

        Ok(Self { paths, i_num_type })
    }
}

mkvmerge_arg!(TrackOrder, "--track-order");

impl ToMkvmergeArgs for TrackOrder {
    fn to_mkvmerge_args(&self, _mi: &mut MediaInfo, _path: &Path) -> Vec<String> {
        let mut i_to_fid: HashMap<usize, usize> = HashMap::new();
        let mut max_fid: usize = 0;

        let order_arg: String = self
            .i_num_type
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
