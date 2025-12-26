use crate::ffmpeg::{self, Packet, Rational, Rescale};

pub struct IstPacket<'a>(pub ffmpeg::Stream<'a>, pub Packet);

impl<'a> IstPacket<'a> {
    pub fn into_inner(self) -> (ffmpeg::Stream<'a>, Packet) {
        (self.0, self.1)
    }
}

pub fn take_min_packet<'a>(
    packets: &mut Vec<Option<IstPacket<'a>>>,
) -> Option<(usize, IstPacket<'a>)> {
    let mut i_min = None::<usize>;
    let mut dts_min = None::<i64>;
    let mut time_base = None::<Rational>;

    for (i, ipkt) in packets.iter().enumerate() {
        let ipkt = some_or!(ipkt, continue);
        let dts = match ipkt.1.dts().or(ipkt.1.pts()) {
            Some(ts) => ts,
            None => return Some((i, packets[i].take().unwrap())),
        };

        let tb = *time_base.get_or_insert(ipkt.0.time_base());
        let dts = dts.rescale(ipkt.0.time_base(), tb);

        if dts_min.map_or(true, |min| dts < min) {
            i_min = Some(i);
            dts_min = Some(dts);
        }
    }

    i_min.map(|i| (i, packets[i].take().unwrap()))
}
