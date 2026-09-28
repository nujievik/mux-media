mod buf_packets;
mod current;
mod encoder;
mod external_fonts;
mod header;

use crate::config::MarkConfigChapters;
use crate::ffmpeg::{self, Rational, format};
use crate::media_info::{MarkMediaInfoStreamsOrder, MarkMediaInfoTargetPaths};
use crate::{
    Config, ConfigChaptersTimeRange, MediaInfo, Msg, MuxError, MuxLogger, Result, StreamsOrder,
    TryFinalizeInit, display,
};
use buf_packets::BufPackets;
use encoder::{Encode, Encoder};
use external_fonts::ExternalFonts;
use log::{LevelFilter, debug, info, warn};
use rayon::prelude::*;
use std::{
    fs,
    io::{self, Write},
    path::Path,
    sync::Mutex,
};

/// Tries run muxing, taking settings from the arguments that this program was started with
/// (normally passed via the command line).
///
/// # Errors
///
/// 1. Successful exit cases (e.g., `--help`, `--list-targets`, etc.)
///    returns an error with exit code `0`.
///
/// 2. CLI or JSON argument parsing failures
///    returns an error with exit code `2`.
///
/// 3. All other errors return exit code `1`.
///
///    - Critical errors return immediately.
///
///    - Errors while processing current files return an error if `--exit-on-err` is set;
///      otherwise, muxing continues with the next files.
pub fn run() -> Result<()> {
    fn init_cfg() -> Result<Config> {
        let mut cfg = Config::try_init()?;
        if let Err(e) = cfg.try_finalize_init() {
            cfg.output.remove_created_dirs();
            Err(e)
        } else {
            Ok(cfg)
        }
    }

    fn init_ffmpeg(cfg: &Config) -> Result<()> {
        if let Err(e) = ffmpeg::init() {
            cfg.output.remove_created_dirs();
            Err(e.into())
        } else {
            let lvl = match cfg.log_level.0 {
                LevelFilter::Trace => ffmpeg::log::Level::Trace,
                _ => ffmpeg::log::Level::Quiet,
            };
            ffmpeg::log::set_level(lvl);
            Ok(())
        }
    }

    let cfg = init_cfg()?;
    MuxLogger::init_with_filter(*cfg.log_level);
    init_ffmpeg(&cfg)?;

    let result = cfg.mux();
    cfg.output.remove_created_dirs();

    result.map(|cnt| match cnt {
        0 => warn!("{}", Msg::NotMuxedAny),
        _ => {
            info!("{} {} {}", Msg::SuccessfullyMuxed, cnt, Msg::Media);
            cfg.save_config_or_warn();
        }
    })
}

impl Config {
    /// Tries run muxing, returning a count of successfully muxed media files.
    ///
    /// # Errors
    ///
    /// - **Only if** [`Config::exit_on_err`] is true.
    ///
    /// - Returns an error if one occurs during processing.
    #[inline]
    pub fn mux(&self) -> Result<usize> {
        let fonts = ExternalFonts::get_new(self);
        let cnt = Mutex::new(0usize);
        let it = Mutex::new(self.input.iter_media_grouped_by_stem());

        (0..self.jobs).into_par_iter().try_for_each(|j| {
            let mut mi = MediaInfo::new(self, j);
            loop {
                let g = { it.lock().map_or(None, |mut it| it.next()) };
                match g {
                    Some(g) => current::mux_current_files(self, fonts.as_ref(), &cnt, &mut mi, g)?,
                    None => return Ok::<(), MuxError>(()),
                }
                mi.clear();
            }
        })?;

        if let Err(e) = remove_input_fonts(self, fonts) {
            warn!("{}: {}", Msg::FailOverwriteInputFiles, e);
        }

        Ok(cnt.into_inner().unwrap_or(0))
    }
}

impl MediaInfo<'_> {
    /// Tries muxing all files from [`MediaInfo::cache`] to `dest`.
    pub fn mux_files(&mut self, dest: &Path) -> Result<()> {
        let order = self.try_take_cmn(MarkMediaInfoStreamsOrder)?;

        // scope to drop berore overwrite
        {
            let mut octx = format::output(dest)?;
            let (mut icontexts, mut encoders, idx_map) =
                header::write_header(self, &order, &mut octx)?;

            let mut iters: Vec<_> = icontexts
                .iter_mut()
                .map(|ictx| Box::new(ictx.packets()))
                .collect();
            let mut buf_packets = BufPackets::new(&mut iters);

            let need_write_progress = self.cfg.log_level.is_need_info() && self.cfg.jobs <= 1;

            info!("{} '{}'...", Msg::MuxingTo, display(dest));

            if self.cfg.log_level.is_need_debug() {
                debug!(
                    "{}:\n{}",
                    Msg::MuxingFiles,
                    crate::display_file_list(order.iter_first_entries().map(|x| x.src()))
                );
            }

            // packets/msg frequency
            let mut progress_frequency = 50usize;
            let mut cnt = 0usize;
            let mut percentage = 0u64;
            let first_file_size = new_first_file_size(&order, need_write_progress);
            let mut writed = 0u64;

            loop {
                let (idx, (ist, mut packet)) = match buf_packets.take_minimal() {
                    Some(tuple) => tuple,
                    None => break,
                };
                buf_packets.fill_idx(idx);

                if need_write_progress && idx == 0 {
                    if cnt > progress_frequency {
                        let p = writed * 100 / first_file_size;
                        if p > percentage {
                            percentage = p;
                            print!("\r{:2}%", p);
                            let _ = io::stdout().flush();
                        } else {
                            progress_frequency = progress_frequency * 2;
                        }
                        cnt = 0;
                    }
                    writed += packet.size() as u64;
                    cnt += 1;
                }

                let enc = match idx_map[idx].get(ist.index()) {
                    Some(Some(i)) => &mut encoders[*i],
                    _ => continue,
                };
                enc.processing_packet(&mut octx, &mut packet)?;
            }

            for enc in &mut encoders {
                enc.finalize(&mut octx)?;
            }

            add_chapters(self, &order, &icontexts, &mut octx);

            octx.write_trailer()?;
        }

        info!("\r{} '{}'", Msg::SuccessfullyMuxed, display(dest));

        if let Err(e) = overwrite(self.cfg, dest, &order) {
            warn!("{}: {}", Msg::FailOverwriteInputFiles, e);
        }

        self.set_cmn(MarkMediaInfoStreamsOrder, order);
        Ok(())
    }
}

fn add_chapters(
    mi: &mut MediaInfo,
    order: &StreamsOrder,
    icontexts: &Vec<format::context::Input>,
    octx: &mut format::context::Output,
) {
    let cfg = mi.cfg;

    let cfg_chapters_it = order.iter_first_entries().filter_map(|ord| {
        let target_paths = mi.get(MarkMediaInfoTargetPaths, &ord.key)?;
        let cfg_chapters = cfg
            .get_targets(MarkConfigChapters, target_paths)
            .unwrap_or(&mi.cfg.chapters);
        Some((ord, cfg_chapters))
    });

    let mut i = 0i64;
    for (ord, cfg_chapters) in cfg_chapters_it {
        if cfg_chapters.no_flag {
            continue;
        }

        if let Some(ranges) = cfg_chapters.ranges.as_ref() {
            for r in ranges {
                add_cfg_chapter(octx, i, r);
                i += 1;
            }
            break;
        }

        let ictx = &icontexts[ord.src_num];

        for chp in ictx.chapters() {
            let meta = chp.metadata();
            let title = meta.get("title").map_or("", |v| v);

            if let Err(e) = octx.add_chapter(i, chp.time_base(), chp.start(), chp.end(), title) {
                warn!("fail copy chapter '{}': {}", title, e)
            }
            i += 1;
        }
    }
}

fn add_cfg_chapter(
    octx: &mut format::context::Output,
    i: i64,
    cfg_chapter: &ConfigChaptersTimeRange,
) {
    const TIME_BASE: Rational = Rational(1, 1000);

    let start = cfg_chapter.start.as_millis() as i64;
    let end = cfg_chapter.end.as_millis() as i64;
    let title = cfg_chapter.title.as_ref().map_or("", |v| v);

    if let Err(e) = octx.add_chapter(i, TIME_BASE, start, end, title) {
        warn!("fail add chapter '{}': {}", title, e);
    }
}

fn new_first_file_size(order: &StreamsOrder, need_write_progress: bool) -> u64 {
    let size = match order.get(0) {
        Some(ord) if need_write_progress => fs::metadata(ord.src()).map_or(1, |meta| meta.len()),
        _ => 1,
    };
    if size > 0 { size } else { 1 }
}

fn overwrite(cfg: &Config, temp_muxed_file: &Path, order: &StreamsOrder) -> Result<()> {
    if !cfg.overwrite {
        return Ok(());
    }

    let dest_file_name = temp_muxed_file
        .file_name()
        .ok_or_else(|| err!("fail get file name"))?;
    let dest = cfg.input.dir().join(dest_file_name);

    for x in order.iter_first_entries() {
        let path = x.src();

        // its share temp fonts - do not delete
        if path.parent().map_or(false, |p| p == cfg.output.temp_dir()) {
            continue;
        }

        debug!("{} '{}'...", Msg::RemovingInputFile, display(path));
        fs::remove_file(path)?;
        debug!("{} '{}'", Msg::InputFileSuccessfullyRemoved, display(path));
    }

    fs::rename(temp_muxed_file, &dest)?;
    info!(
        "{} '{}' {} '{}'",
        Msg::MuxedFile,
        display(temp_muxed_file),
        Msg::SuccessfullyMovedTo,
        display(&dest),
    );

    Ok(())
}

fn remove_input_fonts(cfg: &Config, fonts: Option<ExternalFonts>) -> Result<()> {
    if !cfg.overwrite {
        return Ok(());
    }

    let fonts = match fonts {
        Some(xs) => xs,
        None => return Ok(()),
    };

    for f in &fonts.files {
        debug!("{} '{}'...", Msg::RemovingInputFile, display(f));
        fs::remove_file(f)?;
        debug!("{} '{}'", Msg::InputFileSuccessfullyRemoved, display(f));
    }
    Ok(())
}
