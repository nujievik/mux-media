use super::init_ffmpeg;
use crate::media_info::{MarkMediaInfoShortestFileStem, MarkMediaInfoStreams, StreamsOrder};
use crate::{Config, MediaInfo, MuxLogger, Result, TryFinalizeInit, display};
use log::error;
use std::path::PathBuf;

pub(crate) fn print_info(mut cfg: Config) -> Result<()> {
    MuxLogger::init_with_filter(*cfg.log_level);
    cfg.input.try_finalize_init()?;
    init_ffmpeg(&cfg)?;
    let mut mi = MediaInfo::new(&cfg, 0);

    let media = match cfg.input.iter_media_grouped_by_stem().next() {
        Some(m) => m,
        None => {
            error!("no mediafiles found");
            print_external_fonts(&cfg);
            return Ok(());
        }
    };
    mi.set_cmn(MarkMediaInfoShortestFileStem, media.stem);

    for f in &media.files {
        if let Err(e) = mi.try_insert(f) {
            error!("{e}");
        }
    }
    if let Err(e) = mi.try_finalize_init_streams() {
        error!("{e}");
    }

    print_already_cached_streams(&mi, &media.files);

    match StreamsOrder::new(&mut mi) {
        Ok(o) => print_streams_order(&o),
        Err(e) => error!("{e}"),
    }

    print_external_fonts(&cfg);
    Ok(())
}

fn print_already_cached_streams(mi: &MediaInfo<'_>, media: &[PathBuf]) {
    println!("Unsorted mediafiles:");

    for src in media {
        let streams = some_or!(continue; mi.immut(MarkMediaInfoStreams, src));
        println!("mediafile '{}':", display(src));

        for st in streams {
            println!("  Stream {}: {:?} {}:", st.i, st.ty, st.i_ty);
            println!("    language: {}", st.lang.as_str());
            if let Some(title) = st.title.as_ref() {
                println!("    title: {}", title);
            }
            if let Some(fname) = st.filename.as_ref() {
                println!("    filename: {}", fname);
            }
        }
    }
}

fn print_streams_order(order: &StreamsOrder) {
    let width = order.len().to_string().len();
    println!("\n\nSorted streams order:");

    for (i, ord) in (0..).zip(order.iter()) {
        println!(
            "{:>width$}. {:?} (from '{}' stream {})",
            i,
            ord.ty,
            display(&ord.key),
            ord.key_i_stream
        );
    }
}

fn print_external_fonts(cfg: &Config) {
    let fonts = cfg.input.collect_fonts();
    if fonts.is_empty() {
        return;
    }

    println!("\n\nAdditional external fonts:");
    for font in &fonts {
        println!("{}", display(font));
    }
}
