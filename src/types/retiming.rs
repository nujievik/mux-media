mod audio;
mod concat;
mod new;
pub(crate) mod options;
mod subs;
mod video;

use crate::{
    ArcPathBuf, Duration, MediaInfo, Result, StreamType, StreamsOrderItem,
    ffmpeg::{Rational, format::context},
};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Retiming<'a, 'b> {
    pub temp_dir: &'a Path,
    pub media_info: &'b MediaInfo<'a>,
    pub job: u8,
    pub base: ArcPathBuf,
    pub i_base_stream: usize,
    pub chapters: Vec<RetimingChapter>,
    // retimed base video parts
    pub parts: Vec<RetimingPart>,
    pub base_splits: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct RetimingChapter {
    pub start: Duration,
    pub end: Duration,
    pub uid: Option<Vec<u8>>,
    pub title: Option<String>,
}

#[derive(Debug)]
pub struct RetimingPart {
    pub i_start_chp: usize,
    pub i_end_chp: usize,
    pub src: ArcPathBuf,
    pub start: Duration,
    pub start_offset: f64,
    pub end: Duration,
    pub end_offset: f64,
}

#[derive(Debug, Default)]
pub struct RetimedStream {
    pub src: Option<PathBuf>,
    pub i_stream: usize,
    pub src_time: Option<(Duration, Duration)>,
}

impl Retiming<'_, '_> {
    pub(crate) fn try_any(&self, i: usize, item: &StreamsOrderItem) -> Result<RetimedStream> {
        let src = &item.key;
        let i_stream = item.i_stream;
        let ty = item.ty;

        match ty {
            StreamType::Video => self.try_video(src, i_stream),
            StreamType::Audio => self.try_audio(i, src, i_stream),
            StreamType::Sub => self.try_sub(i, src, i_stream),
            _ => Err(err!("Unsupported stream {} {:?}", i_stream, ty)),
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

    fn is_save_single_part(&self) -> bool {
        self.parts.len() == 1 && self.parts[0].start.is_zero()
    }

    fn single_part_base_retimed_stream(&self, src: &Path, i_stream: usize) -> RetimedStream {
        let p = &self.parts[0];
        let src = if p.src.as_path() != src {
            Some(PathBuf::from(&p.src))
        } else {
            None
        };
        let src_time = Some((p.start, p.end));
        RetimedStream {
            src,
            i_stream,
            src_time,
        }
    }
}

// Writes a split header, returning
// (input stream time base, output stream time base, output stream index).
fn write_split_header(
    ictx: &context::Input,
    ist_index: usize,
    octx: &mut context::Output,
) -> Result<(Rational, Rational, usize)> {
    let (ist_time_base, ost_index) = ictx
        .streams()
        .find_map(|ist| {
            if ist_index != ist.index() {
                return None;
            }
            let mut ost = octx.add_stream(ist.parameters().id()).ok()?;
            ost.set_parameters(ist.parameters());
            Some((ist.time_base(), ost.index()))
        })
        .ok_or_else(|| err!("Not found stream"))?;

    octx.write_header()?;

    let ost_time_base = octx
        .stream(ost_index)
        .map(|ost| ost.time_base())
        .ok_or_else(|| err!("Not found stream"))?;

    Ok((ist_time_base, ost_time_base, ost_index))
}
