mod base;
mod destination;
mod external;
mod ty;

use super::{RetimedStream, Retiming};
use crate::{Duration, Result, display};
use destination::Destination;
use log::warn;
use std::{
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use subtitle_lines::{
    AssLines, FromBytes, SrtLines, StreamingIterator, VttLines, ass::line::AssLine,
    srt::line::SrtLine, vtt::line::VttLine,
};
use ty::SubType;

impl Retiming<'_, '_> {
    pub(crate) fn try_sub(&self, i: usize, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        let is_base = src == **self.base;

        let fall = |dest: &mut Destination, err| -> Result<()> {
            if matches!(dest.ty, SubType::Srt) {
                return Err(err);
            }
            warn!(
                "Fail retiming '{}' stream {} as .{}: {}. Try retime as .srt",
                display(src),
                i_stream,
                dest.ty.as_ext(),
                err
            );
            dest.ty = SubType::Srt;
            dest.path.set_extension("srt");
            Ok(())
        };

        let mut dest = self.new_destination(i, src, i_stream, is_base);

        if is_base {
            if let Err(err) = self.try_base_sub(i_stream, &dest) {
                fall(&mut dest, err)?;
                self.try_base_sub(i_stream, &dest)?;
            }
        } else {
            if let Err(err) = self.try_external_sub(src, i_stream, &dest) {
                fall(&mut dest, err)?;
                self.try_external_sub(src, i_stream, &dest)?;
            }
        }

        Ok(RetimedStream {
            src: Some(dest.path),
            i_stream: 0,
        })
    }
}

impl Retiming<'_, '_> {
    fn len_prev_uid_parts(&self, i_part: usize) -> f64 {
        let src = &self.parts[i_part].src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.src == src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }

    fn len_prev_parts(&self, i_part: usize) -> f64 {
        self.parts[..i_part]
            .iter()
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }

    fn len_prev_nonuid_parts(&self, i_part: usize) -> f64 {
        let src = &self.parts[i_part].src;
        self.parts[..i_part]
            .iter()
            .filter(|p| &p.src != src)
            .map(|p| p.end.as_secs_f64() - p.start.as_secs_f64())
            .sum()
    }
}

fn try_extract(src: &Path, i_stream: usize, dest_ty: SubType, dest_path: &Path) -> Result<()> {
    use crate::ffmpeg::{Rational, format};

    let mut ictx = format::input(&src)?;
    let istream = ictx
        .stream(i_stream)
        .ok_or_else(|| err!("invalid stream index"))?;

    let out_time_base = match dest_ty {
        SubType::Ssa => Rational::new(1, 100),
        _ => Rational::new(1, 1000),
    };
    let codec_id = istream.parameters().id();

    let mut octx = format::output(dest_path)?;

    let ostream_index = {
        let mut ostream = octx.add_stream(codec_id)?;
        ostream.set_parameters(istream.parameters());
        ostream.set_time_base(out_time_base);
        ostream.index()
    };

    octx.write_header()?;

    for (stream, mut packet) in ictx.packets() {
        if stream.index() != i_stream {
            continue;
        }

        packet.set_stream(ostream_index);
        packet.rescale_ts(stream.time_base(), out_time_base);
        packet.write(&mut octx)?;
    }

    octx.write_trailer()?;
    Ok(())
}

fn merge(dest: &Destination, splits: &[PathBuf]) -> Result<()> {
    let f = fs::File::create(&dest.path)?;
    let mut writer = BufWriter::new(f);

    match dest.ty {
        SubType::Ssa => {
            let mut first_split_lines = AssLines::open_file(&splits[0])?;
            let mut is_written_events_mark = false;
            let mut buf_section_mark: Option<Vec<u8>> = None;

            while let Some(l) = first_split_lines.next() {
                match &l {
                    AssLine::SectionMark(mark) => {
                        let bytes = mark.as_bytes();

                        if is_written_events_mark {
                            buf_section_mark = Some(bytes.into());
                            break;
                        }

                        if bytes == b"[Events]" {
                            is_written_events_mark = true;
                        }
                    }
                    _ => (),
                };
                writer.write(l.as_bytes())?;
                writer.write(b"\n")?;
            }

            for split in splits.iter().skip(1) {
                let mut lines = AssLines::open_file(split)?;
                let mut is_written_event = false;

                while let Some(line) = lines.next() {
                    match &line {
                        AssLine::Event(_) => {
                            writer.write(line.as_bytes())?;
                            writer.write(b"\n")?;
                            is_written_event = true;
                        }
                        AssLine::SectionMark(_) if is_written_event => break,
                        _ => (),
                    }
                }
            }

            writer.write(b"\n")?;

            if let Some(bytes) = buf_section_mark {
                writer.write(&bytes)?;
                writer.write(b"\n")?;
            }

            while let Some(l) = first_split_lines.next() {
                writer.write(l.as_bytes())?;
                writer.write(b"\n")?;
            }
        }
        SubType::Srt => {
            let mut is_written_blank = true;
            for split in splits {
                if !is_written_blank {
                    writer.write(b"\n")?;
                }
                let mut lines = SrtLines::open_file(split)?;
                while let Some(l) = lines.next() {
                    writer.write(l.as_bytes())?;
                    writer.write(b"\n")?;
                    is_written_blank = matches!(l, SrtLine::Blank);
                }
            }
        }
        SubType::Vtt => {
            let mut is_written_blank = true;
            let mut is_first = true;

            for split in splits {
                if !is_written_blank {
                    writer.write(b"\n")?;
                }
                let mut lines = VttLines::open_file(split)?;
                while let Some(l) = lines.next() {
                    match &l {
                        _ if is_first => (),
                        VttLine::Blank
                        | VttLine::CueId(_)
                        | VttLine::TimeRangeAndStyle(_)
                        | VttLine::Text(_) => (),
                        _ => continue,
                    }
                    writer.write(l.as_bytes())?;
                    writer.write(b"\n")?;
                    is_written_blank = matches!(l, VttLine::Blank);
                }
                is_first = false;
            }
        }
    };
    Ok(())
}
