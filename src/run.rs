mod current;
mod encoder;
mod header;
mod init_external_fonts;
mod packet;

use crate::{
    Config, MediaInfo, Msg, MuxError, MuxLogger, Result, StreamsOrder, TryFinalizeInit,
    ffmpeg::{self, format},
    markers::*,
};
use encoder::{Encode, Encoder};
use log::{error, info, warn};
use packet::IstPacket;
use rayon::prelude::*;
use std::{path::Path, sync::Mutex};

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
        let order = self.try_take_cmn(MICmnStreamsOrder)?;
        let mut octx = format::output(dest)?;
        let (mut icontexts, mut encoders, idx_map) = header::write_header(self, &order, &mut octx)?;

        let mut iters: Vec<_> = icontexts
            .iter_mut()
            .map(|ictx| Box::new(ictx.packets()))
            .collect();

        let len = iters.len();
        let mut buf_packets = Vec::with_capacity(len);
        for _ in 0..len {
            buf_packets.push(None);
        }

        loop {
            buf_packets
                .iter_mut()
                .enumerate()
                .filter(|(_, pkt)| pkt.is_none())
                .for_each(|(i, pkt)| {
                    *pkt = match iters[i].next() {
                        Some((ist, packet)) => Some(IstPacket(ist, packet)),
                        None => None,
                    }
                });

            let (idx, (ist, mut packet)) = match packet::take_min_packet(&mut buf_packets) {
                Some((i, ipkt)) => (i, ipkt.into_inner()),
                None => break,
            };

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
