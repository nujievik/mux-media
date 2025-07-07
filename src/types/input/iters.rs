use super::Input;
use crate::{EXTENSIONS, GlobSetPattern, MediaNumber, types::helpers::os_str_starts_with};
use log::{debug, trace, warn};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
};
use walkdir::{IntoIter, WalkDir};

macro_rules! iter_any_files_in_dir {
    ($id_fn:ident, $exts:ident) => {
        pub(super) fn $id_fn(&self, dir: &Path) -> impl Iterator<Item = PathBuf> {
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

    pub fn collect_fonts(&self) -> Vec<PathBuf> {
        self.dirs
            .par_iter()
            .flat_map(|dir| self.iter_fonts_in_dir(dir).collect::<Vec<_>>())
            .collect()
    }

    pub fn iter_media_grouped_by_stem<'a>(&'a self) -> impl Iterator<Item = MediaGroupedByStem> {
        let mut media_number = self.init_media_number();
        let mut repeats: HashSet<OsString> = HashSet::new();

        self.iter_media_in_dir(&self.upmost)
            .filter_map(move |path| {
                let up_stem = path.file_stem()?;

                if repeats.contains(up_stem) {
                    trace!(
                        "Found repeat stem '{}'. Skip this",
                        up_stem.to_string_lossy()
                    );
                    return None;
                }

                if let Some(ref mut num) = media_number {
                    num.upd(up_stem);
                    if let Some(range) = self.range {
                        if !range.contains(num.to_u64()) {
                            return None;
                        }
                    }
                }

                let matched: Vec<PathBuf> = self
                    .dirs
                    .par_iter()
                    .flat_map_iter(|dir| self.iter_media_in_dir(dir))
                    .filter(|p| {
                        p.file_stem()
                            .map(|file_stem| os_str_starts_with(&up_stem, file_stem))
                            .unwrap_or(false)
                    })
                    .collect();

                if matched.len() < 2 {
                    debug!(
                        "No external file found for stem '{}'. Skip this",
                        up_stem.to_string_lossy()
                    );
                    return None;
                }

                let mut cnt_upmost = 0;
                let mut cnt_dir = 0;
                let mut inserted_repeat = false;
                for path in &matched {
                    if let Some(parent) = path.parent() {
                        if parent == self.upmost {
                            cnt_upmost += 1;
                            if !inserted_repeat && cnt_upmost > 1 {
                                repeats.insert(up_stem.to_os_string());
                                inserted_repeat = true;
                            }
                        } else if self.is_upmost_higher && parent == self.dir {
                            cnt_dir += 1;
                        }
                    }
                }

                if self.is_upmost_higher && cnt_dir == 0 {
                    warn!(
                        "No track file found for stem '{}' in the input directory '{}'. Skip this",
                        up_stem.to_string_lossy(),
                        self.dir.display()
                    );
                    return None;
                }

                let out_name_middle = if self.out_need_num {
                    match &media_number {
                        Some(num) => OsString::from(num.as_str()),
                        None => {
                            trace!("Unexpected None file_number. Use default out_name_middle");
                            up_stem.to_os_string()
                        }
                    }
                } else {
                    up_stem.to_os_string()
                };

                Some(MediaGroupedByStem {
                    files: matched,
                    out_name_middle,
                    stem: up_stem.to_os_string(),
                })
            })
    }

    #[inline]
    fn init_media_number(&self) -> Option<MediaNumber> {
        if self.need_num || self.out_need_num {
            let path = self.iter_media_in_dir(&self.upmost).skip(1).next();
            if let Some(path) = path {
                if let Some(stem) = path.file_stem() {
                    return Some(MediaNumber::from(stem));
                }
            }
        };

        None
    }
}

pub struct MediaGroupedByStem {
    pub files: Vec<PathBuf>,
    pub out_name_middle: OsString,
    pub stem: OsString,
}

pub(super) struct DirIter<'a> {
    seen: HashSet<PathBuf>,
    walker: IntoIter,
    skip: Option<&'a GlobSetPattern>,
}

impl<'a> DirIter<'a> {
    pub fn new(root: impl Into<PathBuf>, down: u8, skip: Option<&'a GlobSetPattern>) -> Self {
        let walker = WalkDir::new(&root.into())
            .follow_links(true)
            .max_depth((down as usize) + 1)
            .into_iter();

        Self {
            seen: HashSet::new(),
            walker,
            skip,
        }
    }

    #[inline]
    fn should_skip(&self, path: &Path) -> bool {
        match self.skip {
            Some(pat) => pat.glob_set.is_match(path),
            None => false,
        }
    }
}

impl<'a> Iterator for DirIter<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry_result) = self.walker.next() {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();

                    if entry.file_type().is_dir() {
                        if self.should_skip(path) {
                            continue;
                        }

                        if let Ok(real_path) = std::fs::canonicalize(path) {
                            if self.seen.insert(real_path.clone()) {
                                return Some(real_path);
                            }
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        None
    }
}
