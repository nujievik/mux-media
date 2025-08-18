use super::Input;
#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, EXTENSIONS, FileType, MediaNumber, i18n::logs, types::helpers};
use globset::GlobSet;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
};
use walkdir::{IntoIter, WalkDir};

macro_rules! iter_any_files_in_dir {
    ($fn:ident, $exts:ident, $doc:ident) => {
        #[doc = concat!("Returns an iterator over `", stringify!($doc), "` files in a directory.")]
        pub fn $fn(&self, dir: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
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
    iter_any_files_in_dir!(iter_media_in_dir, media, media);
    iter_any_files_in_dir!(iter_fonts_in_dir, fonts, font);

    /// Returns an iterator over given type files in a directory.
    pub fn iter_files_in_dir<'a>(
        &'a self,
        ft: FileType,
        dir: impl AsRef<Path> + 'a,
    ) -> Box<dyn Iterator<Item = PathBuf> + 'a> {
        match ft {
            FileType::Font => Box::new(self.iter_fonts_in_dir(dir)),
            FileType::Media => Box::new(self.iter_media_in_dir(dir)),
        }
    }

    /// Collects all font files from the discovered directories.
    ///
    /// # Warning
    ///
    /// This method assumes [`Self::try_finalize_init`] was called beforehand.
    /// If it wasn’t this will simply return an empty vector.
    ///
    /// ```
    /// # use clap::FromArgMatches;
    /// # use mux_media::{MuxConfig, markers::MCInput};
    /// # use std::path::PathBuf;
    /// #
    /// let cfg = MuxConfig::try_from_args::<&[&str], _>(&[]).unwrap();
    /// let input = cfg.field::<MCInput>();
    /// assert_eq!(Vec::<PathBuf>::new(), input.collect_fonts());
    /// ```
    pub fn collect_fonts(&self) -> Vec<PathBuf> {
        self.dirs[FileType::Font]
            .par_iter()
            .flat_map(|dir| self.iter_fonts_in_dir(dir).collect::<Vec<_>>())
            .collect()
    }

    /// Collects all font files from the discovered directories,
    /// sorts its by stem and filters stem duplicates.
    ///
    /// # Warning
    ///
    /// This method assumes [`Self::try_finalize_init`] was called beforehand.
    /// If it wasn’t this will simply return an empty vector.
    ///
    /// ```
    /// # use clap::FromArgMatches;
    /// # use mux_media::{MuxConfig, markers::MCInput};
    /// # use std::path::PathBuf;
    /// #
    /// let cfg = MuxConfig::try_from_args::<&[&str], _>(&[]).unwrap();
    /// let input = cfg.field::<MCInput>();
    /// let expected = Vec::<PathBuf>::new();
    /// assert_eq!(expected, input.collect_fonts_with_filter_and_sort());
    /// ```
    pub fn collect_fonts_with_filter_and_sort(&self) -> Vec<PathBuf> {
        let mut seen = HashSet::<OsString>::new();

        let mut fonts: Vec<PathBuf> = self
            .collect_fonts()
            .into_iter()
            .filter(|font| match font.file_stem() {
                Some(stem) if !seen.contains(stem) => {
                    let _ = seen.insert(stem.to_owned());
                    true
                }
                _ => false,
            })
            .collect();

        fonts.sort_by(|a, b| a.file_stem().cmp(&b.file_stem()));
        fonts
    }

    /// Returns an iterator over grouped media files by stem from discovered directories.
    ///
    /// # Warning
    ///
    /// This method assumes [`Self::try_finalize_init`] was called beforehand.
    /// If it wasn’t, the iterator will yield no items.
    ///
    /// ```
    /// # use clap::FromArgMatches;
    /// # use mux_media::{MuxConfig, markers::MCInput};
    /// # use std::path::PathBuf;
    /// #
    /// let cfg = MuxConfig::try_from_args::<&[&str], _>(&[]).unwrap();
    /// let input = cfg.field::<MCInput>();
    /// assert_eq!(None, input.iter_media_grouped_by_stem().next());
    /// ```
    pub fn iter_media_grouped_by_stem(&self) -> impl Iterator<Item = MediaGroupedByStem> {
        let mut media_number = self.init_media_number();
        let mut processed = HashSet::<OsString>::new();

        self.iter_media_in_dir(&self.dir).filter_map(move |path| {
            let up_stem = path.file_stem()?;

            if processed.contains(up_stem) {
                logs::trace_found_repeat(up_stem);
                return None;
            }

            if let Some(num) = media_number.as_mut() {
                num.upd(up_stem);

                if self
                    .range
                    .as_ref()
                    .map_or(false, |range| !range.contains(num.to_u64()))
                {
                    return None;
                }
            }

            let matched: Vec<PathBuf> = self.dirs[FileType::Media]
                .par_iter()
                .flat_map_iter(|dir| self.iter_media_in_dir(dir))
                .filter(|p| {
                    p.file_stem()
                        .map_or(false, |stem| helpers::os_str_starts_with(up_stem, stem))
                })
                .collect();

            if !self.solo && matched.len() < 2 {
                logs::warn_no_ext_media(up_stem);
                return None;
            }

            processed.insert(up_stem.to_owned());

            let out_name_middle = match &media_number {
                Some(num) if self.out_need_num => OsString::from(num.as_str()),
                _ => up_stem.to_owned(),
            };

            Some(MediaGroupedByStem {
                files: matched,
                out_name_middle,
                stem: up_stem.to_owned(),
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

#[derive(Debug, PartialEq)]
pub struct MediaGroupedByStem {
    pub files: Vec<PathBuf>,
    pub out_name_middle: OsString,
    pub stem: OsString,
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
