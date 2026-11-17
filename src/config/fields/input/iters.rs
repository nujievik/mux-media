#[cfg(test)]
mod file_number_tests;

use super::{ConfigInput, InputMedia, InputMediaDir};
#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, MediaInfo, RangeUsizeInclusive, i18n::logs};
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
    let media = || dir.dir_media.iter().chain(dir.other_media.iter());

    let mut stems: Vec<_> = media().map(|p| p.file_stem()).collect();
    let mut range_and_file_number =
        init_range_and_file_number(input, &stems[..dir.dir_media.len()]);
    let mut capacity = 0;

    iter::from_fn(move || {
        for pos in 0..dir.dir_media.len() {
            let Some(prefix) = stems[pos].take() else {
                continue;
            };
            let prefix_bytes = prefix.as_encoded_bytes();

            if let Some((range, num)) = range_and_file_number.as_mut() {
                num.update(prefix);
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
    stems: &[Option<&'a OsStr>],
) -> Option<(&'a RangeUsizeInclusive, FileNumber<'a>)> {
    let range = input.range.as_ref()?;
    let stem = match stems.into_iter().skip(1).find_map(|&x| x) {
        Some(stem) => stem,
        None => OsStr::new(""),
    };
    Some((range, FileNumber::new(stem)))
}

/// Extracts an ascii-digits number from file names.
struct FileNumber<'a> {
    file_stem_bytes: &'a [u8],
    stack_ranges: [(u8, u8); 128],

    other_ranges: Vec<(usize, usize)>,
    first_other_ranges_idx: Option<usize>,

    number_count: usize,
    number_idx: usize,
    number: Option<usize>,
}

impl<'a> FileNumber<'a> {
    fn new(file_stem: &'a OsStr) -> Self {
        let file_stem_bytes = file_stem.as_encoded_bytes();
        let mut stack_ranges = [(0u8, 0u8); 128];
        let mut other_ranges = Vec::new();
        let mut first_other_ranges_idx = None;
        let mut number_count = 0;

        let mut push_range = |start, end| {
            if end <= u8::MAX as usize && number_count < stack_ranges.len() {
                stack_ranges[number_count] = (start as u8, end as u8);
            } else {
                other_ranges.push((start, end));
                first_other_ranges_idx.get_or_insert(number_count);
            }

            number_count += 1;
        };

        let mut start = None;

        for (i, byte) in file_stem_bytes.into_iter().enumerate() {
            match (byte.is_ascii_digit(), start) {
                (true, None) => start = Some(i),
                (false, Some(st)) => {
                    push_range(st, i);
                    start = None;
                }
                _ => continue,
            }
        }

        if let Some(start) = start {
            push_range(start, file_stem_bytes.len());
        }

        let mut new = FileNumber {
            file_stem_bytes,
            stack_ranges,
            other_ranges,
            first_other_ranges_idx,
            number_count,
            number_idx: 0,
            number: None,
        };

        // SAFETY: only ascii digit bytes
        let s = unsafe { str::from_utf8_unchecked(new.as_bytes_idx(0)) };
        new.number = s.parse::<usize>().ok();

        new
    }

    fn update(&mut self, stem: &'a OsStr) {
        let new_bytes = stem.as_encoded_bytes();

        for i in iter::once(self.number_idx)
            .chain(0..self.number_idx)
            .chain(self.number_idx + 1..self.number_count)
        {
            let (start, end) = self.digit_start_end_idx(i);

            if end > new_bytes.len() {
                break;
            }

            const MAX_USIZE_LEN: usize = (usize::MAX.ilog10() + 1) as usize;
            if end - start > MAX_USIZE_LEN {
                continue;
            }

            let maybe_new_number = &new_bytes[start..end];

            if maybe_new_number != &self.file_stem_bytes[start..end] {
                let Ok(s) = str::from_utf8(maybe_new_number) else {
                    break;
                };
                let Ok(number) = s.parse::<usize>() else {
                    break;
                };

                self.number_idx = i;
                self.number = Some(number);
                return;
            }
        }

        *self = FileNumber::new(stem)
    }
}

impl FileNumber<'_> {
    /// Gets `usize` number if parsable.
    fn get_usize(&self) -> Option<usize> {
        self.number
    }

    fn as_bytes_idx(&self, idx: usize) -> &[u8] {
        let (start, end) = self.digit_start_end_idx(idx);
        &self.file_stem_bytes[start..end]
    }

    fn digit_start_end_idx(&self, idx: usize) -> (usize, usize) {
        match self.first_other_ranges_idx {
            Some(i) if idx >= i => self.other_ranges[idx - i],
            _ => {
                let (start, end) = self.stack_ranges[idx];
                (start as usize, end as usize)
            }
        }
    }
}
