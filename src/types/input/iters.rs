use super::Input;
use crate::{ArcPathBuf, EXTENSIONS, MediaNumber, i18n::logs, types::helpers};
use globset::GlobSet;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use walkdir::{IntoIter, WalkDir};

macro_rules! iter_any_files_in_dir {
    ($fn:ident, $exts:ident) => {
        pub(super) fn $fn(&self, dir: &Path) -> impl Iterator<Item = PathBuf> {
            std::fs::read_dir(dir)
                .ok()
                .into_iter()
                .flat_map(|rd| rd.filter_map(Result::ok))
                .map(|e| e.path())
                .filter(|path| {
                    if path.is_dir() {
                        return false;
                    }

                    let ext = match path.extension() {
                        Some(e) => e,
                        None => return false,
                    };

                    if !EXTENSIONS.$exts.contains(ext.as_encoded_bytes()) {
                        return false;
                    }

                    if let Some(pat) = &self.skip {
                        if pat.glob_set.is_match(path) {
                            return false;
                        }
                    }

                    true
                })
        }
    };
}

impl Input {
    iter_any_files_in_dir!(iter_media_in_dir, media);
    iter_any_files_in_dir!(iter_fonts_in_dir, fonts);

    /// Collects all font files from the discovered directories.
    ///
    /// # Note
    ///
    /// This method assumes try_finalize_init` was called beforehand.
    /// If it wasn’t, the `dirs` field will be empty and this will simply return an empty vector.
    pub fn collect_fonts(&self) -> Vec<PathBuf> {
        self.dirs
            .par_iter()
            .flat_map(|dir| self.iter_fonts_in_dir(dir).collect::<Vec<_>>())
            .collect()
    }

    /// Returns an iterator over grouped media files by stem from discovered directories.
    ///
    /// # Note
    ///
    /// This method assumes `try_finalize_init` was called beforehand.
    /// If it wasn’t, the iterator will yield no items.
    pub fn iter_media_grouped_by_stem<'a>(&'a self) -> impl Iterator<Item = MediaGroupedByStem> {
        let mut media_number = self.init_media_number();
        let mut processed = HashSet::<Arc<OsString>>::new();

        self.iter_media_in_dir(&self.dir)
            .filter_map(move |path| {
                let up_stem = path.file_stem()?;
                let up_stem = Arc::new(up_stem.to_os_string());

                if processed.contains(&up_stem) {
                    logs::trace_found_repeat(&up_stem);
                    return None;
                }

                if let Some(num) = media_number.as_mut() {
                    num.upd(&up_stem);

                    if self
                        .range
                        .as_ref()
                        .map_or(false, |range| !range.contains(num.to_u64()))
                    {
                        return None;
                    }
                }

                let matched: Vec<PathBuf> = self
                    .dirs
                    .par_iter()
                    .flat_map_iter(|dir| self.iter_media_in_dir(dir))
                    .filter(|p| {
                        p.file_stem()
                            .map_or(false, |fs| helpers::os_str_starts_with(&up_stem, fs))
                    })
                    .collect();

                if matched.len() < 2 {
                    logs::debug_no_ext_media(&up_stem);
                    return None;
                }

                processed.insert(up_stem.clone());

                let out_name_middle: Arc<OsString> = match &media_number {
                    Some(num) if self.out_need_num => OsString::from(num.as_str()).into(),
                    None if self.out_need_num => {
                        log::trace!("Unexpected None file_number. Use default out_name_middle");
                        up_stem.clone()
                    }
                    _ => up_stem.clone(),
                };

                Some(MediaGroupedByStem {
                    files: matched,
                    out_name_middle,
                    stem: up_stem,
                })
            })
    }

    #[inline(always)]
    fn init_media_number(&self) -> Option<MediaNumber> {
        (self.need_num || self.out_need_num)
            .then(|| self.iter_media_in_dir(&self.dir).skip(1).next())
            .flatten()
            .and_then(|path| path.file_stem().map(MediaNumber::from))
    }
}

pub struct MediaGroupedByStem {
    pub files: Vec<PathBuf>,
    pub out_name_middle: Arc<OsString>,
    pub stem: Arc<OsString>,
}

pub(super) struct DirIter<'a> {
    seen: HashSet<ArcPathBuf>,
    walker: IntoIter,
    skip: Option<&'a GlobSet>,
}

impl<'a> DirIter<'a> {
    pub fn new(root: &Path, depth: usize, skip: Option<&'a GlobSet>) -> Self {
        let walker = WalkDir::new(root)
            .follow_links(true)
            .max_depth(depth)
            .into_iter();

        Self {
            seen: HashSet::new(),
            walker,
            skip,
        }
    }

    #[inline(always)]
    fn should_skip(&self, path: &Path) -> bool {
        match self.skip {
            Some(gs) => gs.is_match(path),
            None => false,
        }
    }
}

impl<'a> Iterator for DirIter<'a> {
    type Item = ArcPathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.walker.next() {
            let entry = match entry {
                Ok(entry) if entry.file_type().is_dir() => entry,
                _ => continue,
            };

            let path = entry.path();

            if self.should_skip(path) {
                continue;
            }

            let path = match std::fs::canonicalize(path) {
                Ok(path) if !self.seen.contains(path.as_path()) => path,
                _ => continue,
            };

            let path = ArcPathBuf::from(path);
            self.seen.insert(path.clone());

            return Some(path);
        }
        None
    }
}
