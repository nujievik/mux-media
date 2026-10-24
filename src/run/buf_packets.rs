use crate::ffmpeg::{
    self, Packet, Rational, Rescale, format::context::input::PacketIter, util::media::Type,
};

const NANOSECOND_TIME_BASE: Rational = Rational(1, 1_000_000_000);

pub struct BufPackets<'a>(Vec<BufPkt<'a>>);

struct BufPkt<'a> {
    buf: Option<(ffmpeg::Stream<'a>, Packet)>,
    iter: &'a mut Box<PacketIter<'a>>,
    // in ist_time_base
    offset_to_base_pts: Option<i64>,
}

impl<'a> BufPkt<'a> {
    fn new(iter: &'a mut Box<PacketIter<'a>>) -> Self {
        Self {
            buf: iter.next(),
            iter,
            offset_to_base_pts: None,
        }
    }
}

impl<'a> BufPackets<'a> {
    pub fn new(auto_ts_offsets: bool, packets: &'a mut Vec<Box<PacketIter<'a>>>) -> Self {
        let mut bufs: Vec<_> = packets.iter_mut().map(|iter| BufPkt::new(iter)).collect();

        let base_pts = if auto_ts_offsets {
            bufs[0]
                .buf
                .as_ref()
                .filter(|(ist, _)| matches!(ist.parameters().medium(), Type::Video | Type::Audio))
                .and_then(|(ist, pkt)| {
                    let pts = pkt.pts()?;
                    Some(pts.rescale(ist.time_base(), NANOSECOND_TIME_BASE))
                })
        } else {
            None
        };

        if let Some(base_pts) = base_pts {
            for buf_pkt in bufs.iter_mut().skip(1) {
                let (ist, pkt) = some_or!(continue; buf_pkt.buf.as_ref());

                if !matches!(ist.parameters().medium(), Type::Audio) {
                    continue;
                }

                let pts = some_or!(continue; pkt.pts());
                let base_pts = base_pts.rescale(NANOSECOND_TIME_BASE, ist.time_base());
                let offset = pts - base_pts;

                if offset != 0 {
                    buf_pkt.offset_to_base_pts = Some(offset);
                }
            }
        }

        Self(bufs)
    }

    pub fn fill_idx(&mut self, i: usize) {
        let pkt = &mut self.0[i];
        pkt.buf = pkt.iter.next();
    }

    pub fn take_minimal(&mut self) -> Option<(usize, (ffmpeg::Stream<'a>, Packet))> {
        let mut i_min = None;
        let mut dts_min = i64::MAX;

        for (i, buf_pkt) in self.0.iter().enumerate() {
            let (ist, pkt) = some_or!(continue; buf_pkt.buf.as_ref());

            let dts = match (pkt.dts(), buf_pkt.offset_to_base_pts) {
                (Some(dts), Some(offset)) => dts - offset,
                (Some(dts), None) => dts,
                (None, _) => {
                    i_min = Some(i);
                    break;
                }
            };

            let dts = dts.rescale(ist.time_base(), NANOSECOND_TIME_BASE);
            if dts < dts_min {
                i_min = Some(i);
                dts_min = dts;
            }
        }

        let i_min = i_min?;
        let buf_pkt = &mut self.0[i_min];
        let (ist, mut pkt) = buf_pkt.buf.take().unwrap();

        if let Some(offset) = buf_pkt.offset_to_base_pts {
            pkt.set_dts(pkt.dts().map(|ts| ts - offset));
            pkt.set_pts(pkt.pts().map(|ts| ts - offset));
        }

        Some((i_min, (ist, pkt)))
    }
}
