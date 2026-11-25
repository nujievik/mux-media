mod attached_font;
mod attached_pic;

use crate::ffmpeg::{
    self, Packet, Rational, Rescale,
    format::{self, context, stream},
};
use crate::{MILLISECOND_TIME_BASE, Msg, Result, helpers};
use log::warn;
use std::path::Path;

pub struct EncoderCopy {
    ist_time_base: Rational,
    ost_time_base: Rational,
    // in ist_time_base
    ts_offset: i64,
    ost_index: usize,
    // in ist_time_base
    max_dts: i64,
}

impl<'a> EncoderCopy {
    pub fn new(
        ist: &stream::Stream,
        octx: &'a mut context::Output,
    ) -> Result<(ffmpeg::StreamMut<'a>, EncoderCopy)> {
        let ost = helpers::add_copy_stream(ist, octx)?;
        let enc = Self {
            ist_time_base: MILLISECOND_TIME_BASE,
            ost_time_base: MILLISECOND_TIME_BASE,
            ts_offset: 0,
            ost_index: ost.index(),
            max_dts: i64::MIN,
        };

        Ok((ost, enc))
    }

    pub fn copy_attached_pic(
        ist: &stream::Stream,
        octx: &'a mut context::Output,
    ) -> Result<ffmpeg::StreamMut<'a>> {
        attached_pic::copy(ist, octx)
    }

    pub fn add_font_file(
        path: &Path,
        octx: &'a mut context::Output,
    ) -> Result<ffmpeg::StreamMut<'a>> {
        attached_font::add_font_file(path, octx)
    }
}

impl EncoderCopy {
    pub fn set_ist_time_base(&mut self, tb: Rational) {
        if self.ist_time_base != tb {
            self.ts_offset = self.ts_offset.rescale(self.ist_time_base, tb);
            self.ist_time_base = tb;
        }
    }

    pub fn set_ost_time_base(&mut self, tb: Rational) {
        self.ost_time_base = tb;
    }

    pub fn set_ts_offset<T: Into<i64>>(&mut self, offset: Option<T>) {
        if let Some(ts) = offset {
            self.ts_offset = ts.into().rescale(MILLISECOND_TIME_BASE, self.ist_time_base)
        }
    }

    pub fn process_packet(
        &mut self,
        octx: &mut context::Output,
        packet: &mut Packet,
    ) -> Result<()> {
        self.offset_packet_ts(packet);
        packet.rescale_ts(self.ist_time_base, self.ost_time_base);

        packet.set_position(-1);
        packet.set_stream(self.ost_index);
        packet.write_interleaved(octx)?;
        Ok(())
    }

    fn offset_packet_ts(&mut self, packet: &mut Packet) {
        let mut pts = packet.pts().map(|ts| ts + self.ts_offset);

        if let Some(mut dts) = packet.dts() {
            dts += self.ts_offset;

            if dts >= self.max_dts {
                self.max_dts = dts;
            } else {
                warn!(
                    "{}; {}: {}, {}: {}; {} {}. {}.",
                    Msg::NonMonotonicDts,
                    Msg::Previous,
                    self.max_dts,
                    Msg::Current,
                    dts,
                    Msg::ChangingTo,
                    self.max_dts,
                    Msg::ThisMayResultInIncorrectTimestampsInTheOutputFile
                );
                dts = self.max_dts;
            }

            if let Some(pts) = pts.as_mut().filter(|pts| **pts < dts) {
                warn!(
                    "{}; PTS: {}, DTS: {}; {} {}. {}.",
                    Msg::PtsLessThanDts,
                    pts,
                    dts,
                    Msg::ChangingTo,
                    dts,
                    Msg::ThisMayResultInIncorrectTimestampsInTheOutputFile
                );
                *pts = dts;
            }

            packet.set_dts(Some(dts));
        }

        packet.set_pts(pts);
    }
}
