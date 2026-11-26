use super::init_ffmpeg;
use crate::media_info::{MarkMediaInfoShortestFileStem, MarkMediaInfoStreams, StreamsOrder};
use crate::{ArcPathBuf, Config, MediaInfo, Result, TryFinalizeInit, Value, display};
use log::error;

pub(crate) fn print_info(mut cfg: Config) -> Result<()> {
    cfg.input.try_finalize_init()?;
    init_ffmpeg(&cfg)?;

    cfg.auto_flags.extract_fonts = Value::User(false);
    cfg.exit_on_err = false;
    let mut mi = MediaInfo::new(&cfg, 0);

    let Some(media) = cfg.input.new_iter_media_grouped_by_stem().next() else {
        error!("no mediafiles found");
        print_external_fonts(&cfg);
        return Ok(());
    };
    mi.set_cmn(MarkMediaInfoShortestFileStem, media.stem.into());

    let _ = mi.try_insert_many(&media.files);
    let _ = mi.try_finalize_init_streams();

    print_already_cached_streams(&mi, &media.files);
    print_external_fonts(&cfg);

    // unwrap safe: add_external_and_ass_fonts never returns err if extract_fonts false
    match StreamsOrder::new(&mut mi).map(|o| o.add_external_and_ass_fonts(&mut mi).unwrap()) {
        Ok(o) => print_streams_order(&o),
        Err(e) => error!("{e}"),
    }

    Ok(())
}

fn print_already_cached_streams(mi: &MediaInfo<'_>, media: &[&ArcPathBuf]) {
    println!("Unsorted media files:");
    let mut is_first = true;

    for src in media {
        let streams = some_or!(continue; mi.immut(MarkMediaInfoStreams, src));

        if is_first {
            is_first = false;
        } else {
            print!("\n");
        };
        println!("{}", display(src));

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
    if cfg.input.fonts.is_empty() {
        return;
    }

    println!("\n\nUnsorted font files:");
    for font in &cfg.input.fonts {
        println!("{}", display(font));
    }
}
