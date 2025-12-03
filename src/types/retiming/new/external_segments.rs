use super::CacheMatroska;
use crate::{ArcPathBuf, MediaInfo, MuxError, Result};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

static EXTERNAL_SEGMENTS: LazyLock<RwLock<ExternalSegments>> =
    LazyLock::new(|| RwLock::new(ExternalSegments::default()));

#[derive(Clone, Debug, Default)]
struct ExternalSegments {
    pub map: HashMap<Box<[u8]>, ArcPathBuf>,
    pub dir_set: HashSet<PathBuf>,
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
            Some(Err(error(dir, uid)))
        } else {
            None
        }
    }

    fn insert_all_in_dir(mi: &MediaInfo, cache: &CacheMatroska, dir: &Path) {
        let xs: Vec<(Box<[u8]>, ArcPathBuf)> = mi
            .cfg
            .input
            .iter_matroska_in_dir(dir)
            .par_bridge()
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

        let mut es = EXTERNAL_SEGMENTS.write().unwrap();

        for (k, v) in xs {
            es.map.insert(k, v);
        }
        es.dir_set.insert(dir.to_owned());
    }

    fn error(dir: &Path, uid: &[u8]) -> MuxError {
        err!(
            "Not found external matroska segment '{:?}' in the directory '{}'",
            uid,
            dir.display()
        )
    }
}
