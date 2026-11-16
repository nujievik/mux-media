use super::{ConfigInput, InputMedia, InputMediaDir, Value};
use crate::{Extension, Msg, Result, TryFinalizeInit};
use log::{info, warn};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

macro_rules! filter_and_insert {
    ($self:ident, $media:expr, $skip:ident, $into_path_buf:expr) => {{
        let path_buf = $into_path_buf;

        let is_font = match Extension::from_path(&path_buf) {
            Some(ext) if ext.is_font() => true,
            Some(ext) if ext.is_media() => false,
            _ => continue,
        };

        if $skip.is_some_and(|gs| gs.is_match(&path_buf)) {
            continue;
        }

        if is_font {
            $self.fonts.push(path_buf);
        } else {
            $media.push(path_buf.into());
        }
    }};
}

impl TryFinalizeInit for ConfigInput {
    /// Collects subdirectories of input directory. Do nothing if input is files.
    ///
    /// # Errors
    ///
    /// Returns an error if not any media in the input directory.
    fn try_finalize_init(&mut self) -> Result<()> {
        let dir = match &mut *self.media {
            InputMedia::Dir(d) => d,
            InputMedia::Files(_) => return Ok(()),
        };
        let skip = self.skip.as_ref();

        info!("{}", Msg::CollectFiles);

        for res_entry in fs::read_dir(&dir.dir)? {
            let entry = match res_entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!(
                        "{} {}: {}",
                        Msg::CollectFilesError,
                        Msg::InInputDirectory,
                        err
                    );
                    continue;
                }
            };

            filter_and_insert!(self, dir.dir_media, skip, entry.path());
        }

        if dir.dir_media.is_empty() {
            return Err(err!(NoMediaFilesFoundInInputDirectory));
        }

        let mut walker = WalkDir::new(&dir.dir).follow_links(true).min_depth(2);
        if self.depth != u8::MAX {
            walker = walker.max_depth(self.depth as usize + 1); // +1 for files
        }

        for entry in walker.into_iter() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("{}: {}", Msg::CollectFilesError, err);
                    continue;
                }
            };

            filter_and_insert!(self, dir.other_media, skip, entry.into_path());
        }

        info!("{}\n", Msg::CollectedFilesSuccessfully);
        Ok(())
    }
}

impl ConfigInput {
    pub(crate) const DEPTH_DEFAULT: u8 = 16;

    pub(crate) fn empty() -> ConfigInput {
        ConfigInput {
            fonts: Vec::new(),
            media: Value::Auto(InputMedia::new_dir(PathBuf::new())),
            overwrite: false,
            range: None,
            skip: None,
            depth: Self::DEPTH_DEFAULT,
            orphans_too: false,
        }
    }
}

impl InputMedia {
    pub fn new_dir(dir: PathBuf) -> InputMedia {
        InputMedia::Dir(InputMediaDir {
            dir,
            dir_media: Vec::new(),
            other_media: Vec::new(),
        })
    }
}
