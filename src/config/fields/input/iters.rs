use super::{ConfigInput, InputMedia, InputMediaDir};
#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, FileNumber, MediaInfo, RangeUsizeInclusive, i18n::logs};
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

    let media = || dir.dir_media.iter().chain(dir.other_media.iter());

    let mut stems: Vec<_> = media().map(|p| p.file_stem()).collect();
    let mut capacity = 0;

    iter::from_fn(move || {
        for pos in 0..dir.dir_media.len() {
            let Some(prefix) = stems[pos].take() else {
                continue;
            };
            let prefix_bytes = prefix.as_encoded_bytes();

            if let Some((range, num)) = range_and_file_number.as_mut() {
                num.update(prefix_bytes);
                match num.get_usize() {
                    Some(n) if range.contains(&n) => (),
                    _ => {
                        logs::debug_media_out_of_range(prefix);
                        continue;
                    }
                }
            }

            let mut files: Vec<&'a ArcPathBuf> = Vec::with_capacity(capacity);

            for (file, opt_stem) in media().zip(stems.iter_mut()) {
                let Some(stem) = opt_stem else {
                    continue;
                };
                if stem.as_encoded_bytes().starts_with(prefix_bytes) {
                    files.push(file);
                    *opt_stem = None;
                }
            }

            if files.is_empty() && !input.orphans_too {
                logs::warn_no_ext_media(prefix);
                stems[pos] = Some(prefix);
                continue;
            }

            files.push(&dir.dir_media[pos]);
            capacity = capacity.max(files.len());

            return Some(NewMediaGroupedByStem {
                files,
                stem: prefix,
            });
        }
        None
    })
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
