use super::{Encode, Encoder};
use crate::ffmpeg::{
    self, Packet, Rational,
    format::{self, context},
};
use crate::{Msg, Result, add_copy_stream};
use log::warn;

pub struct EncoderCopy {
    ist_time_base: Rational,
    ost_time_base: Rational,
    ost_index: usize,
    // in ist_time_base
    max_dts: i64,
}

impl Encode for EncoderCopy {
    fn set_ist_time_base(&mut self, tb: Rational) {
        self.ist_time_base = tb;
    }

    fn set_ost_time_base(&mut self, tb: Rational) {
        self.ost_time_base = tb;
    }

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()> {
        if let Some(dts) = packet.dts() {
            if dts >= self.max_dts {
                self.max_dts = dts;
                if let Some(pts) = packet.pts() {
                    if pts < dts {
                        warn!(
                            "{}; PTS: {}, DTS: {}; {} {}. {}.",
                            Msg::PtsLessThanDts,
                            pts,
                            dts,
                            Msg::ChangingTo,
                            dts,
                            Msg::ThisMayResultInIncorrectTimestampsInTheOutputFile
                        );
                        packet.set_pts(Some(dts));
                    }
                }
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
                packet.set_dts(Some(self.max_dts));
                packet.set_pts(packet.pts().map(|pts| pts.max(self.max_dts)));
            }
        }
        packet.rescale_ts(self.ist_time_base, self.ost_time_base);
        packet.set_position(-1);
        packet.set_stream(self.ost_index);
        packet.write_interleaved(octx)?;
        Ok(())
    }

    fn finalize(&mut self, _: &mut context::Output) -> Result<()> {
        Ok(())
    }
}

impl EncoderCopy {
    pub fn new_encoder<'a>(
        ist: &format::stream::Stream,
        octx: &'a mut context::Output,
    ) -> Result<(ffmpeg::StreamMut<'a>, Encoder)> {
        let ost = add_copy_stream(ist, octx)?;
        let enc = Self {
            ist_time_base: Rational(1, 1),
            ost_time_base: Rational(1, 1),
            ost_index: ost.index(),
            max_dts: i64::MIN,
        };

        Ok((ost, Encoder::Copy(enc)))
    }
}
