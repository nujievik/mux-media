mod audio;
mod new;
mod signed_time;
mod subs;
mod video;

use crate::ffmpeg::{
    Rational, Rescale,
    format::{self, context},
};
use crate::{
    ArcPathBuf, ConfigChaptersTimeRange, MILLISECOND_TIME_BASE, MediaInfo, Result, StreamType,
    StreamsOrderItem, Time, add_copy_stream, display,
};
use signed_time::SignedTime;
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
    pub start: Time,
    pub end: Time,
    pub uid: Option<Vec<u8>>,
    pub title: Option<String>,
}

#[derive(Debug)]
pub struct RetimingPart {
    pub i_start_chp: usize,
    pub i_end_chp: usize,
    pub src: ArcPathBuf,
    pub start: Time,
    pub start_offset: SignedTime,
    pub end: Time,
    pub end_offset: SignedTime,
}

#[derive(Debug, Default)]
pub struct RetimedStream {
    pub src: PathBuf,
    pub i_stream: usize,
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
            _ => Err(err!("unsupported stream")),
        }
    }

    pub(crate) fn into_chapters_time_ranges(mut self) -> Vec<ConfigChaptersTimeRange> {
        let mut chapters = Vec::with_capacity(self.chapters.len());
        let mut len = Time::ZERO;

        for p in &self.parts {
            for i_chp in p.i_start_chp..=p.i_end_chp {
                let chp = &self.chapters[i_chp];

                if &self.chapters[p.i_start_chp].uid != &chp.uid {
                    continue;
                }

                let start = len;
                let mut end = len + chp.end - chp.start;
                if i_chp == p.i_end_chp {
                    end = (p.end_offset + end).as_unsigned_time();
                }

                let chapter = ConfigChaptersTimeRange {
                    title: self.chapters[i_chp].title.take(),
                    start,
                    end,
                };
                chapters.push(chapter);
                len += end - start;
            }
        }
        chapters
    }

    fn chapters_nonuid(&self, i_chp: usize) -> Time {
        let uid = &self.chapters[i_chp].uid;
        self.chapters[..i_chp]
            .iter()
            .filter(|c| &c.uid != uid)
            .map(|c| c.end - c.start)
            .sum()
    }
}

fn time_to_ts(time: Time, dest_time_base: Rational) -> i64 {
    (time.as_millis() as i64).rescale(MILLISECOND_TIME_BASE, dest_time_base)
}

fn ts_to_time(ts: i64, ts_time_base: Rational) -> Time {
    let millis = ts.rescale(ts_time_base, MILLISECOND_TIME_BASE);
    Time::from_millis(millis as u64)
}

// Returns (input stream time base, output stream time base, output stream index).
fn write_stream_copy_header(
    ictx: &context::Input,
    ist_index: usize,
    octx: &mut context::Output,
) -> Result<(Rational, Rational, usize)> {
    let ist = ictx
        .stream(ist_index)
        .ok_or_else(|| err!("not found stream"))?;
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

    let (ist_time_base, ost_time_base, ost_index) =
        write_stream_copy_header(&icontexts[0], 0, &mut octx)?;
    let mut ts_offset: i64 = 0;

    for (_, mut packet) in icontexts[0].packets() {
        packet.set_stream(ost_index);
        packet.rescale_ts(ist_time_base, ost_time_base);
        if let Some(dts) = packet.dts() {
            ts_offset = dts;
        }
        packet.write(&mut octx)?;
    }

    let mut last_dts = ts_offset;
    let mut was_error = false;

    for ictx in icontexts.iter_mut().skip(1) {
        let ist = ictx.streams().next().unwrap();
        let ist_time_base = ist.time_base();
        let rescale = |ts: i64| ts.rescale(ist_time_base, ost_time_base);
        let mut is_start_none_dts = true;

        for (_, mut packet) in ictx.packets() {
            packet.set_stream(ost_index);
            packet.set_duration(rescale(packet.duration()));

            match packet.dts() {
                Some(dts) => {
                    is_start_none_dts = false;
                    last_dts = rescale(dts) + ts_offset;
                    packet.set_dts(Some(last_dts));
                }
                None if is_start_none_dts => packet.set_dts(Some(last_dts)),
                None => (),
            }

            if let Some(pts) = packet.pts() {
                packet.set_pts(Some(rescale(pts) + ts_offset));
            }

            if packet.write(&mut octx).is_err() && !was_error {
                log::warn!(
                    "Fail concat retimed parts of '{}'. Output file may be corrupted
Try --no-linked or --parts [!]n[,m] to fix",
                    display(src)
                );
                was_error = true;
            }
        }

        ts_offset = last_dts;
    }

    octx.write_trailer()?;
    Ok(())
}
