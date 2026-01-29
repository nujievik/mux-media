use crate::ffmpeg::{self, Packet, Rational, Rescale, format::context::input::PacketIter};

pub struct BufPackets<'a>(Vec<BufPkt<'a>>);

struct BufPkt<'a> {
    buf: Option<(ffmpeg::Stream<'a>, Packet)>,
    iter: &'a mut Box<PacketIter<'a>>,
}

impl<'a> BufPkt<'a> {
    fn new(iter: &'a mut Box<PacketIter<'a>>) -> Self {
        Self {
            buf: iter.next(),
            iter,
        }
    }
}

impl<'a> BufPackets<'a> {
    pub fn new(packets: &'a mut Vec<Box<PacketIter<'a>>>) -> Self {
        Self(packets.iter_mut().map(|iter| BufPkt::new(iter)).collect())
    }

    pub fn fill_idx(&mut self, i: usize) {
        let pkt = &mut self.0[i];
        pkt.buf = pkt.iter.next();
    }

    pub fn take_minimal(&mut self) -> Option<(usize, (ffmpeg::Stream<'a>, Packet))> {
        let mut i_min = None::<usize>;
        let mut dts_min = None::<i64>;
        let mut time_base = None::<Rational>;

        for (i, buf_pkt) in self.0.iter().enumerate() {
            let (ist, pkt) = some_or!(buf_pkt.buf.as_ref(), continue);
            let dts = match pkt.dts().or(pkt.pts()) {
                Some(ts) => ts,
                None => return Some((i, self.0[i].buf.take().unwrap())),
            };

            let tb = *time_base.get_or_insert(ist.time_base());
            let dts = dts.rescale(ist.time_base(), tb);

            if dts_min.map_or(true, |min| dts < min) {
                i_min = Some(i);
                dts_min = Some(dts);
            }
        }

        i_min.map(|i| (i, self.0[i].buf.take().unwrap()))
    }
}
