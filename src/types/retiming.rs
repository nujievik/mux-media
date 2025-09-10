mod audio;
mod new;
pub(crate) mod options;
mod split;
mod subs;

use crate::{ArcPathBuf, Duration, MediaInfo, Result, Tools, TrackType};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Retiming<'a, 'b> {
    pub temp_dir: &'a Path,
    // without json for parallel run.
    pub tools: Tools<'a>,
    pub media_info: &'b MediaInfo<'a>,
    pub thread: u8,
    pub base: ArcPathBuf,
    pub track: u64,
    pub chapters: Vec<RetimingChapter>,
    // retimed base video parts
    pub parts: Vec<RetimingPart>,
}

#[derive(Debug)]
pub struct RetimingChapter {
    pub start: Duration,
    pub end: Duration,
    pub uid: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct RetimingPart {
    pub i_start_chp: usize,
    pub i_end_chp: usize,
    pub external_src: Option<ArcPathBuf>,
    pub start: Duration,
    pub start_offset: f64,
    pub end: Duration,
    pub end_offset: f64,
}

impl Retiming<'_, '_> {
    pub(crate) fn try_any(
        &self,
        i: usize,
        src: &Path,
        track: u64,
        ty: TrackType,
    ) -> Result<Vec<PathBuf>> {
        match ty {
            TrackType::Video => {
                if track == self.track && src == self.base.as_path() {
                    self.try_base_video()
                } else {
                    Err("Unsupported retiming more than 1 video track at a time".into())
                }
            }
            TrackType::Audio => self.try_audio(i, src, track),
            TrackType::Sub => self.try_sub(i, src, track),
            _ => Err("Unsupported track".into()),
        }
    }

    fn try_base_video(&self) -> Result<Vec<PathBuf>> {
        let ty = TrackType::Video;
        self.parts
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let src = self.src(p);
                let dest = self
                    .temp_dir
                    .join(format!("{}-vid-base-{}.mkv", self.thread, i));
                self.try_split(src, self.track, ty, &dest, p.start, p.end)
                    .map(|_| dest)
            })
            .collect()
    }

    fn src<'a>(&'a self, p: &'a RetimingPart) -> &'a Path {
        p.external_src().unwrap_or(&self.base)
    }

    fn chapters_nonuid(&self, i_chp: usize) -> f64 {
        let uid = &self.chapters[i_chp].uid;
        self.chapters[..i_chp]
            .iter()
            .filter(|c| &c.uid != uid)
            .map(|c| c.end.as_secs_f64() - c.start.as_secs_f64())
            .sum()
    }

    fn parts_nonuid(&self, i_part: usize) -> f64 {
        let src = &self.parts[i_part].external_src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.external_src != src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }
}

impl RetimingPart {
    fn external_src(&self) -> Option<&Path> {
        self.external_src.as_ref().map(|p| p.as_path())
    }
}
