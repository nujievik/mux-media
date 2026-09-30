#![feature(test)]
extern crate test;

use rayon::prelude::*;
use std::path::PathBuf;
use test::{Bencher, black_box};

fn file_iter() -> impl Iterator<Item = PathBuf> {
    const FILE_SET: [&str; 14] = [
        "audio_x1.mka",
        "audio_x8.mka",
        "font_attachs_x16.mks",
        "font_x8_other_x8.mks",
        "other_attachs_x16.mks",
        "other_x8_font_x8.mks",
        "sub_x1.mks",
        "sub_x1_font_x1.mks",
        "sub_x8.mks",
        "vid_1s_and_aud_1.013s.mkv",
        "vid_1s_and_srt_5s.mkv",
        "vid_x1_attach_x1.mkv",
        "video_x1.mkv",
        "video_x8.mkv",
    ];

    FILE_SET.iter().map(|f| {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("data");
        p.push(f);
        p
    })
}

#[bench]
fn open_matroska_files(b: &mut Bencher) {
    b.iter(|| {
        let xs: Vec<_> = file_iter().map(|f| matroska::open(f).unwrap()).collect();
        black_box(xs)
    })
}

#[bench]
fn par_open_matroska_files(b: &mut Bencher) {
    b.iter(|| {
        let xs: Vec<_> = file_iter()
            .par_bridge()
            .map(|f| matroska::open(f).unwrap())
            .collect();
        black_box(xs)
    })
}
