use super::{ConfigInput, InputMedia, InputMediaDir, Value};
use crate::{Extension, Result, TryFinalizeInit};
use std::path::PathBuf;
use walkdir::WalkDir;

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

        let mut walker = WalkDir::new(&dir.dir).follow_links(true);
        if self.depth != u8::MAX {
            walker = walker.max_depth(self.depth as usize + 1); // +1 for files
        }

        for entry in walker.into_iter() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    log::warn!("collect files error: {}", e);
                    continue;
                }
            };

            let path = entry.path();

            let to_insert = match Extension::from_path(path) {
                Some(ext) if ext.is_media() && entry.depth() < 2 => Some(true),
                Some(ext) if ext.is_media() => Some(false),
                Some(ext) if ext.is_font() => None,
                _ => continue,
            };

            if self.skip.as_ref().is_some_and(|gs| gs.is_match(path)) {
                continue;
            }

            let path = entry.into_path();

            match to_insert {
                Some(true) => dir.dir_media.push(path.into()),
                Some(false) => dir.other_media.push(path.into()),
                None => self.fonts.push(path),
            }
        }

        if dir.dir_media.is_empty() {
            Err(err!(NoMediaFilesFoundInInputDirectory))
        } else {
            Ok(())
        }
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
