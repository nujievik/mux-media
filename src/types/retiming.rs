mod audio;
mod new;
pub(crate) mod options;
mod subs;
mod video;

use crate::ffmpeg::{
    Rational, Rescale,
    format::{self, context},
};
use crate::{
    ArcPathBuf, Duration, MediaInfo, Result, StreamType, StreamsOrderItem, add_copy_stream,
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

// Returns (input stream time base, output stream time base, output stream index).
fn write_stream_copy_header(
    ictx: &context::Input,
    ist_index: usize,
    octx: &mut context::Output,
) -> Result<(Rational, Rational, usize)> {
    let ist = ictx
        .stream(ist_index)
        .ok_or_else(|| err!("Not found stream"))?;
    let ost = add_copy_stream(&ist, octx)?;
    let ost_index = ost.index();
    octx.write_header()?;
    let ost_time_base = octx.stream(ost_index).unwrap().time_base();

    Ok((ist.time_base(), ost_time_base, ost_index))
}

fn try_concat(src: &Path, splits: &Vec<PathBuf>, dest: &Path) -> Result<()> {
    let mut icontexts = Vec::with_capacity(splits.len());
    for p in splits {
        icontexts.push(format::input(p)?);
    }
    let mut octx = format::output(dest)?;

    let (_, ost_time_base, ost_index) = write_stream_copy_header(&icontexts[0], 0, &mut octx)?;
    let mut pts_offset: i64 = 0;
    let mut dts_offset: i64 = 0;
    let mut was_error = false;

    for ictx in icontexts.iter_mut() {
        let ist = ictx.streams().next().unwrap();
        let ist_index = ist.index();
        let ist_time_base = ist.time_base();
        let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);

        let mut last_dts = 0;
        let mut max_pts = 0;

        for (ist, mut packet) in ictx.packets() {
            if ist_index != ist.index() {
                continue;
            }

            if let Some(dts) = packet.dts() {
                let dts = rescale(dts) + dts_offset;
                packet.set_dts(Some(dts));
                last_dts = dts;
            }

            if let Some(pts) = packet.pts() {
                let pts = rescale(pts) + pts_offset;
                let pts = pts.max(last_dts);
                packet.set_pts(Some(pts));
                max_pts = max_pts.max(pts);
            }

            packet.set_stream(ost_index);
            if packet.write_interleaved(&mut octx).is_err() && !was_error {
                log::error!(
                    "Fail concat retimed parts of '{}'. Output file may be corrupted\nTry --no-linked to fix",
                    src.display()
                );
                was_error = true;
            }
        }

        dts_offset = last_dts;
        pts_offset = max_pts.max(last_dts);
    }

    octx.write_trailer()?;
    Ok(())
}
