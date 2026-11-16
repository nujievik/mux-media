use super::{ConfigInput, InputMedia, InputMediaDir};
#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, FileNumber, MediaInfo, RangeUsizeInclusive, helpers, i18n::logs};
use either::Either;
use std::{
    ffi::{OsStr, OsString},
    iter,
    path::{Path, PathBuf},
};

impl ConfigInput {
    /// Iters all found font files.
    ///
    /// # Warning
    ///
    /// This method assumes [`ConfigInput::try_finalize_init`] was called beforehand.
    /// If it wasn’t this will simply return an empty iterator.
    #[inline]
    pub fn iter_fonts(&self) -> impl Iterator<Item = &Path> {
        self.fonts.iter().map(|p| p.as_path())
    }

    pub fn new_iter_media_grouped_by_stem<'a>(
        &'a self,
    ) -> impl Iterator<Item = NewMediaGroupedByStem<'a>> {
        match self.media.deref() {
            InputMedia::Dir(dir) => Either::Left(iter_media_grouped_by_stem_dir(self, dir)),

            InputMedia::Files(files) => {
                let path_it = files.iter().map(|p| p.as_path());
                let stem = MediaInfo::help_build_shortest_file_stem(path_it);

                let group = NewMediaGroupedByStem {
                    files: files.iter().collect(),
                    stem,
                };
                Either::Right(iter::once(group))
            }
        }
    }

    /// Collects all font files from the discovered directories.
    ///
    /// # Warning
    ///
    /// This method assumes [`ConfigInput::try_finalize_init`] was called beforehand.
    /// If it wasn’t this will simply return an empty vector.
    ///
    /// ```
    /// use clap::Parser;
    /// use mux_media::Config;
    ///
    /// let i = Config::parse_from::<_, &str>([]).input;
    /// assert!(i.collect_fonts().is_empty());
    /// ```
    #[deprecated = "use iter_fonts instead"]
    pub fn collect_fonts(&self) -> Vec<PathBuf> {
        self.iter_fonts().map(|p| p.into()).collect()
    }

    /// Returns an iterator over grouped media files by stem from discovered directories.
    ///
    /// # Warning
    ///
    /// This method assumes [`ConfigInput::try_finalize_init`] was called beforehand.
    /// If it wasn’t, the iterator will yield no items.
    ///
    /// ```
    /// use clap::Parser;
    /// use mux_media::Config;
    ///
    /// let i = Config::parse_from::<_, &str>([]).input;
    /// assert_eq!(None, i.iter_media_grouped_by_stem().next());
    /// ```
    #[deprecated = "use new_iter_media_grouped_by_stem instead"]
    pub fn iter_media_grouped_by_stem(&self) -> impl Iterator<Item = MediaGroupedByStem> {
        self.new_iter_media_grouped_by_stem()
            .map(|g| MediaGroupedByStem {
                files: g.files.into_iter().map(|p| p.as_path().into()).collect(),
                stem: g.stem.into(),
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct NewMediaGroupedByStem<'a> {
    pub files: Vec<&'a ArcPathBuf>,
    pub stem: &'a OsStr,
}

/// A media files that share a common stem.
#[derive(Debug, PartialEq)]
pub struct MediaGroupedByStem {
    pub files: Vec<PathBuf>,
    pub stem: OsString,
}

fn iter_media_grouped_by_stem_dir<'a>(
    input: &'a ConfigInput,
    dir: &'a InputMediaDir,
) -> impl Iterator<Item = NewMediaGroupedByStem<'a>> {
    let mut range_and_file_number = init_range_and_file_number(input, dir);
    let mut pos = 0;
    let mut capacity = 0;

    let len = dir.dir_media.len();
    let mut is_taken = IsMediaTaken(vec![0u8; len.max(dir.other_media.len())]);

    iter::from_fn(move || {
        loop {
            while pos < len && is_taken.dir(pos) {
                pos += 1;
            }
            if pos == len {
                return None;
            }
            is_taken.set_dir_true(pos);

            let Some(current_stem) = dir.dir_media[pos].file_stem() else {
                continue;
            };

            if let Some((range, num)) = range_and_file_number.as_mut() {
                num.update(current_stem.as_encoded_bytes());
                match num.get_usize() {
                    Some(n) if range.contains(&n) => (),
                    _ => {
                        logs::debug_media_out_of_range(current_stem);
                        continue;
                    }
                }
            }

            let mut files: Vec<&'a ArcPathBuf> = Vec::with_capacity(capacity);

            for (i, f) in dir.dir_media.iter().enumerate() {
                if is_taken.dir(i) {
                    continue;
                }
                let Some(stem) = f.file_stem() else {
                    is_taken.set_dir_true(i);
                    continue;
                };
                if helpers::os_str_starts_with(current_stem, stem) {
                    files.push(f);
                    is_taken.set_dir_true(i);
                }
            }

            for (i, f) in dir.other_media.iter().enumerate() {
                if is_taken.other(i) {
                    continue;
                }
                let Some(stem) = f.file_stem() else {
                    is_taken.set_other_true(i);
                    continue;
                };
                if helpers::os_str_starts_with(current_stem, stem) {
                    is_taken.set_other_true(i);
                    files.push(f);
                }
            }

            if files.is_empty() && !input.orphans_too {
                logs::warn_no_ext_media(current_stem);
                is_taken.set_dir_false(pos);
                pos += 1;
                continue;
            }

            files.push(&dir.dir_media[pos]);
            capacity = capacity.max(files.len());

            return Some(NewMediaGroupedByStem {
                files,
                stem: current_stem,
            });
        }
    })
}

struct IsMediaTaken(Vec<u8>);

impl IsMediaTaken {
    const TAKEN_DIR: u8 = 1 << 0;
    const TAKEN_OTHER: u8 = 1 << 1;

    fn dir(&self, i: usize) -> bool {
        (self.0[i] & Self::TAKEN_DIR) != 0
    }

    fn other(&self, i: usize) -> bool {
        (self.0[i] & Self::TAKEN_OTHER) != 0
    }

    fn set_dir_true(&mut self, i: usize) {
        self.0[i] |= Self::TAKEN_DIR;
    }

    fn set_dir_false(&mut self, i: usize) {
        self.0[i] &= !Self::TAKEN_DIR;
    }

    fn set_other_true(&mut self, i: usize) {
        self.0[i] |= Self::TAKEN_OTHER;
    }
}

#[inline(always)]
fn init_range_and_file_number<'a>(
    input: &'a ConfigInput,
    dir: &'a InputMediaDir,
) -> Option<(&'a RangeUsizeInclusive, FileNumber)> {
    let range = input.range.as_ref()?;

    if let Some(second_file) = dir.dir_media.iter().skip(1).next() {
        if let Some(stem) = second_file.file_stem() {
            return Some((range, FileNumber::from(stem)));
        }
    }

    const EMPTY_SLICE: &[u8] = &[];
    Some((range, FileNumber::from(EMPTY_SLICE)))
}
