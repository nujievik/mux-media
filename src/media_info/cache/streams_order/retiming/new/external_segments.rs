use super::CacheMatroska;
use crate::{ArcPathBuf, Extension, FxHashMap, FxHashSet, MediaInfo, Result, config::InputMedia};
use either::Either;
use rayon::prelude::*;
use rustc_hash::FxBuildHasher;
use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

static EXTERNAL_SEGMENTS: RwLock<ExternalSegments> = RwLock::new(ExternalSegments {
    map: FxHashMap::with_hasher(FxBuildHasher),
    dir_set: FxHashSet::with_hasher(FxBuildHasher),
});
struct ExternalSegments {
    map: FxHashMap<Box<[u8]>, ArcPathBuf>,
    dir_set: FxHashSet<PathBuf>,
}

pub(super) fn find_external_segment(
    mi: &MediaInfo,
    cache: &CacheMatroska,
    dir: &Path,
    uid: &[u8],
) -> Result<ArcPathBuf> {
    return if let Some(res) = get_cached(dir, uid) {
        res
    } else {
        insert_all_in_dir(mi, cache, dir);
        get_cached(dir, uid).unwrap()
    };

    fn get_cached(dir: &Path, uid: &[u8]) -> Option<Result<ArcPathBuf>> {
        let es = EXTERNAL_SEGMENTS.read().unwrap();

        if let Some(p) = es.map.get(uid) {
            Some(Ok(p.clone()))
        } else if es.dir_set.contains(dir) {
            Some(Err(err!(
                "not found external matroska segment in video directory"
            )))
        } else {
            None
        }
    }

    fn insert_all_in_dir(mi: &MediaInfo, cache: &CacheMatroska, dir: &Path) {
        let is_in_dir = |p: &&ArcPathBuf| p.parent() == Some(dir);

        let iter = match mi.cfg.input.media.deref() {
            InputMedia::Dir(d) if dir == d.dir => Either::Left(d.dir_media.iter()),
            InputMedia::Dir(d) => Either::Right(d.other_media.iter().filter(is_in_dir)),
            InputMedia::Files(xs) => Either::Right(xs.iter().filter(is_in_dir)),
        };

        let uid_path_pairs: Vec<(Box<[u8]>, ArcPathBuf)> = iter
            .par_bridge()
            .filter(|p| Extension::from_path(p).is_some_and(|ext| ext.is_matroska()))
            .filter_map(|m| {
                if let Some(u) = match cache.immut(&m) {
                    Some(mat) => mat.info.uid.clone(),
                    None => matroska::open(&m).ok().map_or(None, |mat| mat.info.uid),
                } {
                    Some((u.into(), m.into()))
                } else {
                    None
                }
            })
            .collect();

        let mut segments = EXTERNAL_SEGMENTS.write().unwrap();
        segments.map.extend(uid_path_pairs);
        segments.dir_set.insert(dir.to_owned());
    }
}
