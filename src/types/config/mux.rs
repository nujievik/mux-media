use crate::{
    ArcPathBuf, CacheMIOfFile, CacheState, Config, EXTENSIONS, MediaInfo, Msg, MuxCurrent,
    MuxError, Muxer, Result, Tool, Tools, i18n::logs, markers::MICmnStem,
    types::input::iters::MediaGroupedByStem,
};
use log::{error, info, trace};
use rayon::prelude::*;
use std::{
    ffi::OsString,
    fs,
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
    //
    // Panics on a [`Mutex::lock`] error.
    // This indicates an internal logic error that must be fixed.
    #[inline]
    pub fn mux(&self) -> Result<usize> {
        let fonts = init_external_fonts(self);
        let cnt = Mutex::new(0usize);
        let it = Mutex::new(self.input.iter_media_grouped_by_stem());

        (0..self.jobs).into_par_iter().try_for_each(|t| {
            let mut mi = MediaInfo::new(self, t);
            loop {
                let g = { it.lock().unwrap().next() };
                match g {
                    Some(g) => mux_media_group(self, fonts.as_ref(), &cnt, &mut mi, g)?,
                    None => return Ok::<(), MuxError>(()),
                }
                mi.clear();
            }
        })?;

        return Ok(cnt.into_inner().unwrap_or(0));

        fn init_external_fonts(cfg: &Config) -> Option<(ArcPathBuf, CacheMIOfFile)> {
            if !matches!(cfg.muxer, Muxer::Matroska) {
                return None;
            }

            let fonts = cfg.input.collect_fonts();
            if fonts.is_empty() {
                return None;
            }

            let fall = |e| {
                log::warn!("Fail write external fonts: {}. Skipping", e);
                None
            };

            let out = cfg.output.temp_dir.join("external-fonts.mkv");
            if let Err(e) = write_temp_fonts(cfg, fonts, &out) {
                return fall(e);
            }

            let streams = match MediaInfo::help_build_streams(&out) {
                Ok(xs) => xs,
                Err(e) => return fall(e),
            };
            let cache = CacheMIOfFile {
                streams: CacheState::Cached(streams),
                ..Default::default()
            };

            Some((out.into(), cache))
        }

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

            match cfg.muxer.mux_current(mi, &out) {
                MuxCurrent::Ok(tool_out) => {
                    trace!("{}", tool_out);
                    info!("{} '{}'", Msg::SuccessMuxed, out.display());
                    if let Ok(mut cnt) = cnt.lock() {
                        *cnt += 1;
                    }
                }
                MuxCurrent::Continue => (),
                MuxCurrent::Err(e) if cfg.exit_on_err => return Err(e),
                MuxCurrent::Err(e) => error!("{}", e),
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

fn write_temp_fonts(cfg: &Config, fonts: Vec<PathBuf>, out: &Path) -> Result<()> {
    let srt = cfg.output.temp_dir.join("dummy-subs.srt");

    {
        let text = "1\n00:00:00,000 --> 00:00:01,000\n.\n";
        fs::write(&srt, text)?;
    }

    let mut i = 0usize;
    let metas: Vec<String> = fonts
        .iter()
        .map(|_| {
            i += 1;
            format!("-metadata:s:{}", i)
        })
        .collect();

    let mut args = Vec::with_capacity(4 + fonts.len() * 4);
    args.push(p!("-y"));
    args.push(p!("-i"));
    args.push(&srt);
    fonts.iter().enumerate().for_each(|(i, f)| {
        args.push(p!("-attach"));
        args.push(f);
        args.push(p!(&metas[i]));
        args.push(p!(guess_mime(f)));
    });
    args.push(out);

    let tools = Tools::from(cfg);
    let _ = tools.run(Tool::Ffmpeg, &args)?;
    return Ok(());

    fn guess_mime(f: &Path) -> &'static str {
        if f.extension()
            .map_or(false, |ext| EXTENSIONS.otf.contains(ext.as_encoded_bytes()))
        {
            "mimetype=application/vnd.ms-opentype"
        } else {
            "mimetype=application/x-truetype-font"
        }
    }
}
