use crate::{
    LangCode, MCDefaultTFlags, MCEnabledTFlags, MCForcedTFlags, MCLocale, MISavedTrackNums,
    MITILang, MITargetGroup, MITargets, MediaInfo, MuxError, TargetGroup, TrackID, TrackType,
};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct TrackOrder {
    pub fid_map: HashMap<u64, PathBuf>,
    pub fid_num_type: Vec<(u64, u64, TrackType)>,
}

impl TryFrom<&mut MediaInfo<'_>> for TrackOrder {
    type Error = MuxError;

    fn try_from(mi: &mut MediaInfo) -> Result<Self, Self::Error> {
        if mi.is_empty() {
            return Err("Not found any cached Media".into());
        }

        let locale_lang = *mi.mc.get::<MCLocale>();
        let paths: Vec<PathBuf> = mi.get_cache().keys().cloned().collect();

        let mut sorted: Vec<(usize, u64, TrackType, OrderSortKey)> = Vec::new();
        let mut i: usize = 0;

        for path in &paths {
            let num_types: Vec<(u64, TrackType)> = mi
                .try_get::<MISavedTrackNums>(path)?
                .iter()
                .flat_map(|(tt, nums)| nums.iter().map(move |num| (*num, tt)))
                .collect();

            let target_group = *mi.try_get::<MITargetGroup>(path)?;
            let targets = mi.try_get::<MITargets>(path)?.clone();

            let defaults = mi.mc.get_trg::<MCDefaultTFlags>(&targets);
            let forceds = mi.mc.get_trg::<MCForcedTFlags>(&targets);
            let enableds = mi.mc.get_trg::<MCEnabledTFlags>(&targets);

            for (num, ttype) in num_types {
                let tid = TrackID::Num(num);
                let lang = *mi.try_get_ti::<MITILang>(path, &tid)?;
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

                sorted.push((i, num, ttype, key));
            }

            i += 1;
        }

        sorted.sort_by(|a, b| a.3.cmp(&b.3));

        let mut fid_num_type: Vec<(u64, u64, TrackType)> = Vec::new();
        let mut i_to_fid: Vec<u64> = vec![0; paths.len()];
        let mut added: HashSet<usize> = HashSet::new();

        let mut fid: u64 = 0;

        for (i, num, ttype, _) in sorted {
            if !added.contains(&i) {
                added.insert(i);
                i_to_fid[i] = fid;
                fid += 1;
            }
            fid_num_type.push((fid, num, ttype));
        }

        i = paths.len();

        let fid_map: HashMap<u64, PathBuf> = paths
            .into_iter()
            .rev()
            .map(|pb| {
                i -= 1; // Len - 1
                (i_to_fid[i], pb)
            })
            .collect();

        Ok(Self {
            fid_map,
            fid_num_type,
        })
    }
}

#[derive(Clone)]
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
        };

        let flag_order = |flag: Option<bool>| match flag {
            Some(true) => 0,
            None => 1,
            Some(false) => 2,
        };

        let default = flag_order(default);
        let forced = flag_order(forced);
        let enabled = flag_order(enabled);

        let target_group = if target_group == TargetGroup::Signs {
            0
        } else {
            1
        };

        let lang = if lang == locale_lang {
            0
        } else if lang == LangCode::Jpn {
            3
        } else if lang == LangCode::Und {
            2
        } else {
            1
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderSortKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
