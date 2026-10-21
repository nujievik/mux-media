mod input_stream;

use super::{Encode, Encoder};
use crate::config::MarkConfigTsOffsets;
use crate::ffmpeg::{self, Dictionary, format::context};
use crate::media_info::{MarkMediaInfoStreams, MarkMediaInfoTargetPaths};
use crate::{
    DispositionType, Lang, MediaInfo, Result, Stream, StreamType, StreamsOrder, StreamsOrderItem,
    Target, VERSION, immut,
};
use enum_map::EnumMap;

pub(super) fn write_header(
    mi: &mut MediaInfo,
    order: &StreamsOrder,
    octx: &mut context::Output,
) -> Result<(Vec<context::Input>, Vec<Encoder>, Vec<Vec<Option<usize>>>)> {
    let len = order.iter_first_entries().count();
    let mut icontexts = Vec::with_capacity(len);
    let mut idx_map: Vec<Vec<Option<usize>>> = vec![vec![]; len];
    let mut encoders: Vec<Encoder> = Vec::with_capacity(order.len());

    let auto = mi.cfg.auto_flags.map_dispositions();
    let mut counts: EnumMap<StreamType, EnumMap<DispositionType, usize>> = EnumMap::default();

    // (is_locale, value)
    let mut lang_default_audio: Option<(bool, Lang)> = None;

    for ord in &order.0 {
        let ist = input_stream::new(mi, &mut icontexts, ord)?;
        let (mut ost, mut enc) = Encoder::new(&ist, octx)?;

        if let Some(xs) = immut!(mi, MarkMediaInfoStreams, &ord.key) {
            let stream = &xs[ord.key_i_stream];
            ost.set_metadata(new_ost_metadata(stream, &ist));

            if let Some(target_paths) = mi.immut(MarkMediaInfoTargetPaths, &ord.key) {
                set_ost_dispositions(
                    mi,
                    target_paths,
                    &auto,
                    &mut counts,
                    &mut lang_default_audio,
                    ord,
                    stream,
                    &mut ost,
                );

                let (i, offsets) = mi.cfg.stream_val(MarkConfigTsOffsets, target_paths, stream);
                if let Some(offset) = offsets.get(&i, &stream.lang) {
                    enc.set_ts_offset(Some(offset as i64));
                }
            }
        }

        push_idx(&mut idx_map[ord.src_num], ord.i_stream, ost.index());
        encoders.push(enc);
    }

    let mut meta = Dictionary::new();
    meta.set("encoding_tool", VERSION);
    octx.set_metadata(meta);

    octx.write_header()?;

    for (i, ord) in order.0.iter().enumerate() {
        let ost_index = some_or!(continue; idx_map[ord.src_num][ord.i_stream]);
        let ist_tb = icontexts[ord.src_num]
            .stream(ord.i_stream)
            .unwrap()
            .time_base();
        let ost_tb = octx.stream(ost_index).unwrap().time_base();

        let enc = &mut encoders[i];
        enc.set_ist_time_base(ist_tb);
        enc.set_ost_time_base(ost_tb);
    }

    Ok((icontexts, encoders, idx_map))
}

fn new_ost_metadata<'a>(stream: &Stream, ist: &'a ffmpeg::Stream<'a>) -> Dictionary<'a> {
    let mut meta = ist.metadata().to_owned();
    meta.set("language", stream.lang.as_str());
    if let Some(s) = stream.title.as_ref() {
        meta.set("title", &*s);
    }
    if let Some(s) = stream.filename.as_ref() {
        meta.set("filename", s);
    }
    meta
}

fn set_ost_dispositions(
    mi: &MediaInfo,
    target_paths: &Vec<Target>,
    auto: &EnumMap<DispositionType, bool>,
    counts: &mut EnumMap<StreamType, EnumMap<DispositionType, usize>>,
    lang_default_audio: &mut Option<(bool, Lang)>,
    ord: &StreamsOrderItem,
    stream: &Stream,
    ost: &mut ffmpeg::StreamMut,
) {
    let st = unsafe { &mut *ost.as_mut_ptr() };

    for ty in DispositionType::iter() {
        let (i_key, values) = mi.cfg.stream_val_dispositions(ty, target_paths, stream);

        let v = match values.get(&i_key, &stream.lang) {
            Some(v) => v,
            None if !auto[ty] => false,
            None if counts[stream.ty][ty] >= values.max_for_stream_type(ty, stream.ty) => false,

            None if !matches!(ty, DispositionType::Default) => true,

            None if stream.ty.is_sub() => {
                match lang_default_audio.as_ref() {
                    // set default signs only if signs lang == lang_default_audio
                    Some((_, lang)) if ord.is_signs => lang == &*stream.lang,

                    // set default subtitles only if audio language not locale
                    Some((is_locale, _)) if *is_locale => false,

                    // set default subtitles only if stream language != audio language
                    Some((_, lang)) => lang != &*stream.lang,

                    None => true,
                }
            }

            None => true,
        };

        if v {
            if matches!(ty, DispositionType::Default)
                && lang_default_audio.is_none()
                && stream.ty.is_audio()
            {
                let is_locale = &*stream.lang == &Lang::Code(mi.cfg.locales.locale_streams());
                *lang_default_audio = Some((is_locale, (*stream.lang).clone()));
            }

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
