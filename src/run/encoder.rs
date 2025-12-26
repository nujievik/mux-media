mod copy;

pub use copy::EncoderCopy;

use crate::Result;
use crate::ffmpeg::{
    self, Packet, Rational,
    format::{self, context},
};

#[non_exhaustive]
pub enum Encoder {
    Copy(EncoderCopy),
}

pub trait Encode {
    fn set_ist_time_base(&mut self, tb: Rational);

    fn set_ost_time_base(&mut self, tb: Rational);

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()>;

    fn finalize(&mut self, octx: &mut context::Output) -> Result<()>;
}

impl Encoder {
    pub fn new<'a>(
        ist: &format::stream::Stream,
        octx: &'a mut format::context::Output,
    ) -> Result<(ffmpeg::StreamMut<'a>, Self)> {
        EncoderCopy::new_encoder(ist, octx)
    }
}

impl Encode for Encoder {
    fn set_ist_time_base(&mut self, tb: Rational) {
        match self {
            Self::Copy(enc) => enc.set_ist_time_base(tb),
        }
    }

    fn set_ost_time_base(&mut self, tb: Rational) {
        match self {
            Self::Copy(enc) => enc.set_ost_time_base(tb),
        }
    }

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()> {
        match self {
            Self::Copy(enc) => enc.processing_packet(octx, packet),
        }
    }

    fn finalize(&mut self, octx: &mut context::Output) -> Result<()> {
        match self {
            Self::Copy(enc) => enc.finalize(octx),
        }
    }
}
