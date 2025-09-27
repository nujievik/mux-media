mod audio;
mod new;
pub(crate) mod options;
mod split;
mod subs;
mod video;

use crate::{ArcPathBuf, Duration, MediaInfo, Result, Tools, TrackOrderItemRetimed, TrackType};
use std::path::Path;

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
    pub display: Vec<String>,
}

#[derive(Debug)]
pub struct RetimingPart {
    pub i_start_chp: usize,
    pub i_end_chp: usize,
    pub src: ArcPathBuf,
    pub no_retiming: bool,
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
    ) -> Result<TrackOrderItemRetimed> {
        match ty {
            TrackType::Video => self.try_video(src, track),
            TrackType::Audio => self.try_audio(i, src, track),
            TrackType::Sub => self.try_sub(i, src, track),
            _ => Err("Unsupported track".into()),
        }
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
        let src = &self.parts[i_part].src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.src != src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }
}
