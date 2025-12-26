use crate::{
    ArcPathBuf, CacheMIOfFile, Config, MediaInfo, Msg, MuxError, Result, i18n::logs,
    markers::MICmnStem, types::input::iters::MediaGroupedByStem,
};
use log::{error, info};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub fn mux_current_files(
    cfg: &Config,
    fonts: Option<&(ArcPathBuf, CacheMIOfFile)>,
    cnt: &Mutex<usize>,
    mi: &mut MediaInfo<'_>,
    m: MediaGroupedByStem,
) -> Result<()> {
    let out = cfg.output.build_out(m.out_name_middle);
    info!("{} '{}'...", Msg::Muxing, out.display());

    match init_current_files(fonts, mi, m.stem, m.files, &out) {
        MuxCurrent::Ok(()) => (),
        MuxCurrent::Continue => return Ok(()),
        MuxCurrent::Err(e) => return Err(e),
    }

    match mi.mux_files(&out) {
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

fn init_current_files(
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
