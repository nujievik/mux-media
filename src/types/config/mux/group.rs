mod header;

use crate::{
    MediaInfo, Result,
    ffmpeg::{self, Packet, Rational, Rescale, format},
    markers::*,
};
use std::path::Path;

pub(super) fn mux_group(mi: &mut MediaInfo, dest: &Path) -> Result<()> {
    let order = mi.try_take_cmn(MICmnStreamsOrder)?;
    let mut octx = format::output(dest)?;
    let (mut icontexts, idx_map) = header::write_header(mi, &order, &mut octx)?;

    let mut iters: Vec<_> = icontexts
        .iter_mut()
        .map(|ictx| Box::new(ictx.packets()))
        .collect();

    let len = iters.len();
    let mut buf_packets = Vec::with_capacity(len);
    for _ in 0..len {
        buf_packets.push(None);
    }

    loop {
        buf_packets
            .iter_mut()
            .enumerate()
            .filter(|(_, pkt)| pkt.is_none())
            .for_each(|(i, pkt)| {
                *pkt = match iters[i].next() {
                    Some((ist, packet)) => Some(IstPacket(ist, packet)),
                    None => None,
                }
            });

        let (idx, (ist, mut packet)) = match take_min_packet(&mut buf_packets) {
            Some((i, ipkt)) => (i, ipkt.into_inner()),
            None => break,
        };

        let ost_index = match idx_map[idx][ist.index()] {
            Some(i) => i,
            None => continue,
        };
        let ost = octx.stream(ost_index).unwrap();

        packet.rescale_ts(ist.time_base(), ost.time_base());
        packet.set_position(-1);
        packet.set_stream(ost_index);
        packet.write_interleaved(&mut octx)?;
    }

    mi.set_cmn(MICmnStreamsOrder, order);

    octx.write_trailer()?;
    Ok(())
}

struct IstPacket<'a>(ffmpeg::Stream<'a>, Packet);

impl<'a> IstPacket<'a> {
    fn into_inner(self) -> (ffmpeg::Stream<'a>, Packet) {
        (self.0, self.1)
    }
}

fn take_min_packet<'a>(packets: &mut Vec<Option<IstPacket<'a>>>) -> Option<(usize, IstPacket<'a>)> {
    let mut i_min = None::<usize>;
    let mut dts_min = None::<i64>;
    let mut time_base = None::<Rational>;

    for (i, ipkt) in packets.iter().enumerate() {
        let ipkt = some_or!(ipkt, continue);
        let dts = some_or!(ipkt.1.dts().or(ipkt.1.pts()), continue);

        let tb = *time_base.get_or_insert(ipkt.0.time_base());
        let dts = dts.rescale(ipkt.0.time_base(), tb);

        if dts_min.is_none() || dts < dts_min.unwrap() {
            i_min = Some(i);
            dts_min = Some(dts);
        }
    }

    i_min.map(|i| (i, packets[i].take().unwrap()))
}
