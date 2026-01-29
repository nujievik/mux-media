mod buf_packets;
mod current;
mod encoder;
mod header;
mod init_external_fonts;

use crate::{
    Config, MediaInfo, Msg, MuxError, MuxLogger, Result, StreamsOrder, TryFinalizeInit,
    ffmpeg::{self, format},
    markers::*,
};
use buf_packets::BufPackets;
use encoder::{Encode, Encoder};
use log::{LevelFilter, error, info, warn};
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
            ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
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
            info!("{} {} {}", Msg::SuccessMuxed, cnt, Msg::LMedia);
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
        let fonts = init_external_fonts::init_external_fonts(self);
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

        Ok(cnt.into_inner().unwrap_or(0))
    }
}

impl MediaInfo<'_> {
    /// Tries muxing all files from [`MediaInfo::cache`] to `dest`.
    pub fn mux_files(&mut self, dest: &Path) -> Result<()> {
        // packets/msg frequency
        const PROGRESS_FREQUENCY: usize = 1000;

        let order = self.try_take_cmn(MICmnStreamsOrder)?;
        let mut octx = format::output(dest)?;
        let (mut icontexts, mut encoders, idx_map) = header::write_header(self, &order, &mut octx)?;

        let mut iters: Vec<_> = icontexts
            .iter_mut()
            .map(|ictx| Box::new(ictx.packets()))
            .collect();
        let mut buf_packets = BufPackets::new(&mut iters);

        let need_write_progress = match log::max_level() {
            LevelFilter::Error => false,
            _ => self.cfg.jobs <= 1,
        };
        info!("{} '{}...", Msg::Muxing, dest.display());

        let mut cnt = 0usize;
        let first_file_size = new_first_file_size(&order, need_write_progress);
        let mut writed = 0u64;

        loop {
            let (idx, (ist, mut packet)) = match buf_packets.take_minimal() {
                Some(tuple) => tuple,
                None => break,
            };
            buf_packets.fill_idx(idx);

            if need_write_progress && idx == 0 {
                if cnt > PROGRESS_FREQUENCY {
                    print!("\r{:2}%", writed * 100 / first_file_size);
                    let _ = io::stdout().flush();
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

        copy_chapters(self, &order, &icontexts, &mut octx);
        self.set_cmn(MICmnStreamsOrder, order);

        octx.write_trailer()?;
        info!("\r{} '{}'", Msg::SuccessMuxed, dest.display());
        Ok(())
    }
}

fn copy_chapters(
    mi: &mut MediaInfo,
    order: &StreamsOrder,
    icontexts: &Vec<format::context::Input>,
    octx: &mut format::context::Output,
) {
    let cfg = mi.cfg;
    let it = order.iter_first_entries().filter_map(|ord| {
        let target_paths = mi.get(MITargetPaths, &ord.key)?;
        let chapters = cfg
            .get_targets(CfgChapters, target_paths)
            .unwrap_or(&mi.cfg.chapters);

        if chapters.no_flag {
            None
        } else {
            Some(&icontexts[ord.src_num])
        }
    });

    for (i, chp) in it.flat_map(|ictx| ictx.chapters().enumerate()) {
        let title = match chp.metadata().get("title") {
            Some(title) => String::from(title),
            None => i.to_string(),
        };

        if let Err(e) = octx.add_chapter(chp.id(), chp.time_base(), chp.start(), chp.end(), &title)
        {
            error!("Fail copy chapter '{}': {}", title, e)
        }
    }
}

fn new_first_file_size(order: &StreamsOrder, need_write_progress: bool) -> u64 {
    let size = match order.get(0) {
        Some(ord) if need_write_progress => fs::metadata(ord.src()).map_or(1, |meta| meta.len()),
        _ => 1,
    };
    if size > 0 { size } else { 1 }
}
