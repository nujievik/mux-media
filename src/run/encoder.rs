mod copy;

pub use copy::EncoderCopy;

use crate::ffmpeg::{
    self, Packet, Rational,
    format::{self, context},
};
use crate::{Result, helpers};

#[non_exhaustive]
pub enum Encoder {
    AttachedPic,
    Copy(EncoderCopy),
}

pub trait Encode {
    fn set_ist_time_base(&mut self, tb: Rational);

    fn set_ost_time_base(&mut self, tb: Rational);

    fn set_ts_offset(&mut self, offset: Option<i64>);

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()>;

    fn finalize(&mut self, octx: &mut context::Output) -> Result<()>;
}

impl Encoder {
    pub fn new<'a>(
        ist: &format::stream::Stream,
        octx: &'a mut format::context::Output,
        is_attach: bool,
    ) -> Result<(ffmpeg::StreamMut<'a>, Self)> {
        if is_attach {
            match helpers::add_attached_pic(ist, octx) {
                Ok(ost) => Ok((ost, Encoder::AttachedPic)),
                Err(e) => {
                    log::error!("{}", e);
                    Err(err!("fail copy attachment stream.\nTry --no-attachs"))
                }
            }
        } else {
            EncoderCopy::new_encoder(ist, octx)
        }
    }
}

impl Encode for Encoder {
    fn set_ist_time_base(&mut self, tb: Rational) {
        match self {
            Self::AttachedPic => (),
            Self::Copy(enc) => enc.set_ist_time_base(tb),
        }
    }

    fn set_ost_time_base(&mut self, tb: Rational) {
        match self {
            Self::AttachedPic => (),
            Self::Copy(enc) => enc.set_ost_time_base(tb),
        }
    }

    fn set_ts_offset(&mut self, offset: Option<i64>) {
        match self {
            Self::AttachedPic => (),
            Self::Copy(enc) => enc.set_ts_offset(offset),
        }
    }

    fn processing_packet(&mut self, octx: &mut context::Output, packet: &mut Packet) -> Result<()> {
        match self {
            Self::AttachedPic => Ok(()),
            Self::Copy(enc) => enc.processing_packet(octx, packet),
        }
    }

    fn finalize(&mut self, octx: &mut context::Output) -> Result<()> {
        match self {
            Self::AttachedPic => Ok(()),
            Self::Copy(enc) => enc.finalize(octx),
        }
    }
}
