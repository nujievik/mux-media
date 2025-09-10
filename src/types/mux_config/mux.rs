use crate::{
    CacheMICommon, CacheState, MediaInfo, Msg, MuxConfig, MuxCurrent, MuxError, Result, i18n::logs,
    markers::MICmnStem, types::input::iters::MediaGroupedByStem,
};
use log::{error, info, trace};
use rayon::prelude::*;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Mutex,
};

impl MuxConfig {
    /// Tries perform muxing, returning a count of successfully muxed media files.
    ///
    /// # Errors
    ///
    /// - **Only if** [`MuxConfig::exit_on_err`] is true.
    ///
    /// - Returns a muxing error if one occurs during processing.
    //
    // Panics on a [`Mutex::lock`] error.
    // This indicates an internal logic error that must be fixed.
    #[inline]
    pub fn mux(&self) -> Result<usize> {
        let cache = init_cache_common(self);
        let cnt = Mutex::new(0usize);
        let it = Mutex::new(self.input.iter_media_grouped_by_stem());

        (0..self.threads).into_par_iter().try_for_each(|t| {
            loop {
                let mut mi = init_media_info(self, &cache, t);

                match it.lock().map(|mut it| it.next()) {
                    Ok(Some(m)) => mux_media_group(self, &cnt, &mut mi, m)?,
                    Ok(None) => return Ok::<(), MuxError>(()),
                    Err(e) => panic!("{}", e),
                }
            }
        })?;

        return Ok(cnt.lock().map(|c| *c).unwrap_or(0));

        fn init_cache_common(cfg: &MuxConfig) -> CacheMICommon {
            let mi = MediaInfo::from(cfg);
            let external_fonts = CacheState::from_res(mi.build_external_fonts());

            CacheMICommon {
                external_segments: Default::default(),
                external_fonts,
            }
        }

        fn init_media_info<'a>(cfg: &'a MuxConfig, cache: &CacheMICommon, t: u8) -> MediaInfo<'a> {
            let mut mi = MediaInfo::from(cfg);
            mi.cache.common = cache.clone();
            mi.thread = t;
            let json = cfg.output.temp_dir.join(format!("{}.json", t));
            mi.tools.json = Some(json);
            mi
        }

        fn mux_media_group(
            cfg: &MuxConfig,
            cnt: &Mutex<usize>,
            mi: &mut MediaInfo<'_>,
            m: MediaGroupedByStem,
        ) -> Result<()> {
            let out = cfg.output.build_out(m.out_name_middle);
            info!("{} '{}'...", Msg::Muxing, out.display());

            match init_current_media(cfg.exit_on_err, mi, m.stem, m.files, &out) {
                MuxCurrent::Ok(()) => (),
                MuxCurrent::Continue => return Ok(()),
                MuxCurrent::Err(e) => return Err(e),
            }

            match cfg.muxer.mux_current(mi, &out) {
                MuxCurrent::Ok(tool_out) => {
                    trace!("{}", tool_out);
                    tool_out.log_warns();
                    info!("{} '{}'", Msg::SuccessMuxed, out.display());
                    if let Ok(mut cnt) = cnt.lock() {
                        *cnt += 1;
                    }
                }
                MuxCurrent::Continue => (),
                MuxCurrent::Err(e) if cfg.exit_on_err => return Err(e),
                MuxCurrent::Err(e) => error!("{}", e),
            };

            mi.clear_current();
            Ok(())
        }

        fn init_current_media(
            exit_on_err: bool,
            mi: &mut MediaInfo,
            stem: OsString,
            files: Vec<PathBuf>,
            out: &Path,
        ) -> MuxCurrent<()> {
            if out.exists() {
                logs::warn_file_is_already_exists(out);
                return MuxCurrent::Continue;
            }

            mi.set_cmn::<MICmnStem>(stem);

            if let Err(e) = mi.try_insert_many(files, exit_on_err) {
                return Err(e).into();
            }

            if mi.is_no_files() {
                logs::warn_not_out_save_any(out);
                return MuxCurrent::Continue;
            }

            MuxCurrent::Ok(())
        }
    }
}
