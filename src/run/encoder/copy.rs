use super::{Encode, Encoder};
use crate::ffmpeg::{
    self, Packet, Rational,
    format::{self, context},
};
use crate::{Result, add_copy_stream};

pub struct EncoderCopy {
    ist_time_base: Rational,
    ost_time_base: Rational,
    ost_index: usize,
}

impl Encode for EncoderCopy {
    fn set_ist_time_base(&mut self, tb: Rational) {
        self.ist_time_base = tb;
    }

    fn set_ost_time_base(&mut self, tb: Rational) {
        self.ost_time_base = tb;
    }

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()> {
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
        };

        Ok((ost, Encoder::Copy(enc)))
    }
}
