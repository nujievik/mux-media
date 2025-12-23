mod group;
mod init_external_fonts;

use crate::{
    ArcPathBuf, CacheMIOfFile, Config, MediaInfo, Msg, MuxError, Result, i18n::logs,
    markers::MICmnStem, types::input::iters::MediaGroupedByStem,
};
use log::{error, info};
use rayon::prelude::*;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Mutex,
};

impl Config {
    /// Tries perform muxing, returning a count of successfully muxed media files.
    ///
    /// # Errors
    ///
    /// - **Only if** [`Config::exit_on_err`] is true.
    ///
    /// - Returns a muxing error if one occurs during processing.
    #[inline]
    pub fn mux(&self) -> Result<usize> {
        let fonts = init_external_fonts::init_external_fonts(self);
        let cnt = Mutex::new(0usize);
        let it = Mutex::new(self.input.iter_media_grouped_by_stem());

        (0..self.jobs).into_par_iter().try_for_each(|j| {
            let mut mi = MediaInfo::new(self, j);
            loop {
                let g = { it.lock().map_or(None, |mut it| it.next()) };
                match g {
                    Some(g) => mux_media_group(self, fonts.as_ref(), &cnt, &mut mi, g)?,
                    None => return Ok::<(), MuxError>(()),
                }
                mi.clear();
            }
        })?;

        return Ok(cnt.into_inner().unwrap_or(0));

        fn mux_media_group(
            cfg: &Config,
            fonts: Option<&(ArcPathBuf, CacheMIOfFile)>,
            cnt: &Mutex<usize>,
            mi: &mut MediaInfo<'_>,
            m: MediaGroupedByStem,
        ) -> Result<()> {
            let out = cfg.output.build_out(m.out_name_middle);
            info!("{} '{}'...", Msg::Muxing, out.display());

            match init_current_media(fonts, mi, m.stem, m.files, &out) {
                MuxCurrent::Ok(()) => (),
                MuxCurrent::Continue => return Ok(()),
                MuxCurrent::Err(e) => return Err(e),
            }

            match group::mux_group(mi, &out) {
                Ok(()) => {
                    info!("{} '{}'", Msg::SuccessMuxed, out.display());
                    if let Ok(mut cnt) = cnt.lock() {
                        *cnt += 1;
                    }
                }
                Err(e) if cfg.exit_on_err => return Err(e),
                Err(e) => error!("{}", e),
            };

            Ok(())
        }

        fn init_current_media(
            fonts: Option<&(ArcPathBuf, CacheMIOfFile)>,
            mi: &mut MediaInfo,
            stem: OsString,
            files: Vec<PathBuf>,
            out: &Path,
        ) -> MuxCurrent<()> {
            if out.exists() {
                logs::warn_file_is_already_exists(out);
                return MuxCurrent::Continue;
            }

            mi.set_cmn(MICmnStem, stem);

            if let Err(e) = mi
                .try_insert_many(files)
                .and_then(|_| mi.try_finalize_init_streams())
            {
                return Err(e).into();
            }

            if let Some((f, cache)) = fonts {
                mi.cache.of_files.insert(f.clone(), cache.clone());
            }

            if mi.cache.of_files.is_empty() {
                logs::warn_not_out_save_any(out);
                MuxCurrent::Continue
            } else {
                MuxCurrent::Ok(())
            }
        }
    }
}

/// A result of mux current files.
enum MuxCurrent<T> {
    Continue,
    Ok(T),
    Err(MuxError),
}

impl<T> From<Result<T>> for MuxCurrent<T> {
    fn from(res: Result<T>) -> MuxCurrent<T> {
        match res {
            Ok(val) => MuxCurrent::Ok(val),
            Err(e) => MuxCurrent::Err(e),
        }
    }
}
