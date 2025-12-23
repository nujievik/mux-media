use crate::{
    DispositionType, MediaInfo, Result, Stream, StreamType, StreamsOrder, StreamsOrderItem,
    VERSION,
    ffmpeg::{
        self, Dictionary,
        format::{self, context},
    },
    markers::*,
};
use enum_map::EnumMap;
use std::path::Path;

pub(super) fn write_header(
    mi: &mut MediaInfo,
    order: &StreamsOrder,
    octx: &mut context::Output,
) -> Result<(Vec<context::Input>, Vec<Vec<Option<usize>>>)> {
    let len = order.iter_first_entries().count();
    let mut icontexts = Vec::with_capacity(len);
    let mut idx_map: Vec<Vec<Option<usize>>> = vec![vec![]; len];

    let auto = mi.cfg.auto_flags.map_dispositions();
    let mut counts: EnumMap<StreamType, EnumMap<DispositionType, usize>> = EnumMap::default();

    for ord in &order.0 {
        let ist = try_input_stream(mi, &mut icontexts, ord)?;
        let stream = mi
            .immut(MIStreams, &ord.key)
            .map(|xs| &xs[ord.key_i_stream]);

        let mut ost = octx.add_stream(ffmpeg::codec::Id::None)?;
        ost.set_parameters(ist.parameters());
        unsafe {
            (*ost.parameters().as_mut_ptr()).codec_tag = 0;
        }
        ost.set_time_base(ist.time_base());
        ost.set_metadata(new_stream_metadata(stream, &ist));
        set_ost_dispositions(mi, &auto, &mut counts, ord, stream, &mut ost);

        push_idx(&mut idx_map[ord.src_num], ist.index(), ost.index());
    }

    let mut meta = octx.metadata().to_owned();
    meta.set("application", VERSION);
    octx.set_metadata(meta);

    octx.write_header()?;

    Ok((icontexts, idx_map))
}

fn try_input_stream<'a>(
    mi: &mut MediaInfo,
    icontexts: &'a mut Vec<context::Input>,
    ord: &StreamsOrderItem,
) -> Result<ffmpeg::Stream<'a>> {
    fn get_sub_charenc<'a>(mi: &'a mut MediaInfo, src: &Path) -> Option<&'a str> {
        if *mi.cfg.auto_flags.encs {
            mi.get(MISubCharEncoding, src)
                .and_then(|enc| enc.get_ffmpeg_sub_charenc())
        } else {
            None
        }
    }

    if icontexts.get(ord.src_num).is_none() {
        let src = ord.src();
        let ictx = if let Some(s) = get_sub_charenc(mi, src) {
            let mut opts = Dictionary::new();
            opts.set("sub_charenc", s);
            format::input_with_dictionary(src, opts)
        } else {
            format::input(src)
        }?;
        icontexts.push(ictx);
    }

    let ictx = &icontexts[ord.src_num];
    ictx.stream(ord.i_stream)
        .ok_or_else(|| err!("Not found stream"))
}

fn new_stream_metadata<'a>(stream: Option<&Stream>, ist: &'a ffmpeg::Stream<'a>) -> Dictionary<'a> {
    let mut meta = ist.metadata().to_owned();

    if let Some(st) = stream {
        meta.set("language", st.lang.as_str());

        if let Some(s) = st.name.as_ref() {
            meta.set("title", &*s);
        }
        if let Some(s) = st.filename.as_ref() {
            meta.set("filename", s);
        }
    }

    meta
}

fn set_ost_dispositions(
    mi: &MediaInfo,
    auto: &EnumMap<DispositionType, bool>,
    counts: &mut EnumMap<StreamType, EnumMap<DispositionType, usize>>,
    ord: &StreamsOrderItem,
    stream: Option<&Stream>,
    ost: &mut ffmpeg::StreamMut,
) {
    let stream = some_or!(stream, return);
    let target_paths = some_or!(mi.immut(MITargetPaths, &ord.key), return);

    let st = unsafe { &mut *ost.as_mut_ptr() };

    for ty in DispositionType::iter() {
        let (i_key, values) = mi.cfg.stream_val_dispositions(ty, target_paths, stream);

        let v = values.get(&i_key, &stream.lang).unwrap_or_else(|| {
            if auto[ty] {
                let cnt = counts[stream.ty][ty];
                cnt < values.max(ty)
            } else {
                false
            }
        });

        if v {
            counts[stream.ty][ty] += 1;
            st.disposition |= ty.bits();
        }
    }
}

fn push_idx(map: &mut Vec<Option<usize>>, ist_index: usize, ost_index: usize) {
    if map.get(ist_index).is_none() {
        for _ in map.len()..=ist_index {
            map.push(None);
        }
    }
    map[ist_index] = Some(ost_index);
}
