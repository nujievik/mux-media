use super::*;
use crate::{Config, Extension, MediaInfo, Result, StreamType, display};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, ErrorKind, Write};
use subtitle_lines::ass::line::{AssLine, FontName};
use subtitle_lines::{AssLines, FromBytes, StreamingIterator};

impl StreamsOrder {
    pub fn add_external_and_ass_fonts(mut self, mi: &mut MediaInfo) -> Result<StreamsOrder> {
        if !mi.cfg.is_save_fonts() {
            return Ok(self);
        }

        let mut attach_filenames = self.attach_filenames;
        let mut fonts = Vec::with_capacity(mi.cfg.input.fonts.len());

        push_external_fonts(mi.cfg, &mut attach_filenames, &mut fonts);

        if *mi.cfg.auto_flags.extract_fonts {
            push_ass_fonts(mi, &mut self.items, &mut attach_filenames, &mut fonts)?;
        }

        recalculate_items(&mut self.items, fonts);

        self.attach_filenames = attach_filenames;
        Ok(self)
    }
}

fn push_external_fonts(
    cfg: &Config,
    attach_filenames: &mut FxHashSet<String>,
    fonts: &mut Vec<(String, PathBuf)>,
) {
    for f in cfg.input.iter_fonts() {
        let Some(fname) = f.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !attach_filenames.contains(fname) {
            attach_filenames.insert(fname.into());
            fonts.push((fname.to_lowercase(), f.into()))
        }
    }
}

fn push_ass_fonts(
    mi: &mut MediaInfo,
    items: &mut [StreamsOrderItem],
    attach_filenames: &mut FxHashSet<String>,
    fonts: &mut Vec<(String, PathBuf)>,
) -> Result<()> {
    let ass_iter = items.into_iter().filter(|x| x.is_first_entry).filter(|x| {
        Extension::from_path(x.src())
            .is_some_and(|ext| matches!(ext, Extension::Ass | Extension::Ssa))
    });

    for item in ass_iter {
        match ExtractAssFonts::new(mi, attach_filenames, fonts, item).and_then(|ext| ext.extract())
        {
            Ok(Some(path)) => item.src = Some(path),
            Ok(None) => continue,
            Err(e) if mi.cfg.exit_on_err => return Err(e),
            Err(e) => {
                log::warn!(
                    "fail extract fonts from '{}': {}. Copying subs 'as is'",
                    display(item.src()),
                    e
                );
            }
        }
    }

    Ok(())
}

fn recalculate_items(items: &mut Vec<StreamsOrderItem>, add_fonts: Vec<(String, PathBuf)>) {
    let mut not_attached_srcs_count = 0usize;
    let mut max_old_src_num = 0usize;
    let mut pos_first_attach = None;

    for (i, item) in items.iter().enumerate() {
        max_old_src_num = max_old_src_num.max(item.src_num);
        if pos_first_attach.is_none() {
            if item.ty.is_attached() {
                pos_first_attach = Some(i);
            } else if item.is_first_entry {
                not_attached_srcs_count += 1;
            }
        }
    }

    let mut fonts: Vec<StreamsOrderItem> = Vec::new();
    let mut tail: Vec<StreamsOrderItem> = Vec::new();

    if let Some(i) = pos_first_attach {
        for item in items.drain(i..) {
            match item.ty {
                StreamType::AttachedFont => fonts.push(item),
                _ => tail.push(item),
            }
        }
    }

    for (lowercased_filename, key) in add_fonts {
        fonts.push(StreamsOrderItem {
            ty: StreamType::AttachedFont,
            is_signs: false,
            key: key.into(),
            key_i_stream: 0,
            src: None,
            i_stream: 0,
            src_num: usize::MAX,
            is_first_entry: true,
            lowercased_filename: Some(lowercased_filename),
            is_font_file: true,
        });
    }

    fonts.sort_by(|a, b| a.lowercased_filename.cmp(&b.lowercased_filename));

    // map old_src_num -> new_src_num
    let mut old_to_new_src: Vec<Option<usize>> = vec![None; max_old_src_num + 1];
    for i in 0..not_attached_srcs_count {
        old_to_new_src[i] = Some(i);
    }

    let mut next_src_num = not_attached_srcs_count;

    for mut item in fonts.into_iter().chain(tail.into_iter()) {
        if item.src_num == usize::MAX {
            // new extracted font (always unique)
            item.src_num = next_src_num;
            next_src_num += 1;
        } else if let Some(mapped_num) = old_to_new_src[item.src_num] {
            item.src_num = mapped_num;
            item.is_first_entry = false;
        } else {
            old_to_new_src[item.src_num] = Some(next_src_num);
            item.src_num = next_src_num;
            item.is_first_entry = true;
            next_src_num += 1;
        }
        items.push(item);
    }
}

struct ExtractAssFonts<'a> {
    cfg: &'a Config,
    src_lines: AssLines<'static, BufReader<File>>,
    attach_filenames: &'a mut FxHashSet<String>,

    extracted_fonts: &'a mut Vec<(String, PathBuf)>,
    out_ass_path: PathBuf,
    out_ass: BufWriter<File>,

    is_blank_written: bool,
    is_fonts_section: bool,
}

impl ExtractAssFonts<'_> {
    fn new<'a>(
        mi: &MediaInfo<'a>,
        attach_filenames: &'a mut FxHashSet<String>,
        extracted_fonts: &'a mut Vec<(String, PathBuf)>,
        item: &StreamsOrderItem,
    ) -> Result<ExtractAssFonts<'a>> {
        let src_lines = AssLines::open_file(item.src())?;
        let cfg = mi.cfg;

        let out_ass_path = cfg.output.temp_dir().join(format!(
            "{}-subs-without-fonts-{}.ass",
            mi.job, item.src_num
        ));
        let out_ass = BufWriter::new(File::create(&out_ass_path)?);

        Ok(ExtractAssFonts {
            cfg,
            src_lines,
            attach_filenames,
            extracted_fonts,
            out_ass_path,
            out_ass,
            is_blank_written: false,
            is_fonts_section: false,
        })
    }

    fn extract(mut self) -> Result<Option<PathBuf>> {
        self.find_fonts_section()?;

        if !self.is_fonts_section {
            return Ok(None);
        }

        let mut name = None;
        while let Some(fontname) = self.find_fontname(&mut name)? {
            let (lowercased_filename, path) = if self.attach_filenames.contains(&fontname) {
                continue;
            } else {
                let lowercased_filename = fontname.to_lowercase();
                let path = self.cfg.output.temp_dir().join(&fontname);

                self.attach_filenames.insert(fontname);
                (lowercased_filename, path)
            };

            let file = match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(f) => f,
                Err(err) if matches!(err.kind(), ErrorKind::AlreadyExists) => {
                    self.extracted_fonts.push((lowercased_filename, path));
                    continue;
                }
                Err(err) => return Err(err.into()),
            };
            let mut writer = BufWriter::new(file);

            name = self.decode_font(&mut writer)?;
            writer.flush()?;
            self.extracted_fonts.push((lowercased_filename, path));
        }

        self.write_remainder()?;
        Ok(Some(self.out_ass_path))
    }

    fn write_remainder(&mut self) -> Result<()> {
        while let Some(line) = self.src_lines.next() {
            match line {
                AssLine::Blank if self.is_blank_written => continue,
                AssLine::Blank => self.is_blank_written = true,
                _ => self.is_blank_written = false,
            }
            self.out_ass.write_all(line.as_bytes())?;
            self.out_ass.write_all(b"\n")?;
        }
        self.out_ass.flush()?;
        Ok(())
    }

    fn find_fonts_section(&mut self) -> Result<()> {
        while let Some(line) = self.src_lines.next() {
            match line {
                AssLine::Blank if self.is_blank_written => continue,
                AssLine::Blank => self.is_blank_written = true,
                AssLine::SectionMark(mark) if mark.as_bytes() == b"[Fonts]" => {
                    self.is_fonts_section = true;
                    break;
                }
                _ => self.is_blank_written = false,
            }
            self.out_ass.write_all(line.as_bytes())?;
            self.out_ass.write_all(b"\n")?;
        }
        Ok(())
    }

    fn find_fontname(&mut self, current: &mut Option<String>) -> Result<Option<String>> {
        if let Some(current_fontname) = current.take() {
            return Ok(Some(current_fontname));
        }

        while let Some(line) = self.src_lines.next() {
            match line {
                AssLine::SectionMark(_) => {
                    self.is_blank_written = false;
                    self.is_fonts_section = false;
                    self.out_ass.write_all(line.as_bytes())?;
                    self.out_ass.write_all(b"\n")?;
                    return Ok(None);
                }
                AssLine::FontName(n) => return try_some_fontname(n),
                AssLine::Unrecognized(l) => {
                    if let Some(fontname) = l.strip_prefix(b"fontname: ") {
                        let fontname = str::from_utf8(fontname)?;
                        return Ok(Some(fontname.into()));
                    }
                }
                _ => continue,
            }
        }

        Ok(None)
    }

    fn decode_font(&mut self, writer: &mut BufWriter<File>) -> Result<Option<String>> {
        while let Some(line) = self.src_lines.next() {
            match line {
                AssLine::Blank => return Ok(None),
                AssLine::SectionMark(mark) => {
                    if !self.is_blank_written {
                        self.out_ass.write_all(b"\n")?;
                    }
                    self.out_ass.write_all(mark.as_bytes())?;
                    self.out_ass.write_all(b"\n")?;

                    self.is_fonts_section = false;
                    self.is_blank_written = false;
                    return Ok(None);
                }
                AssLine::FontName(n) => return try_some_fontname(n),
                AssLine::Font(l) => decode_font_line(writer, l.as_bytes())?,

                _ => return Err(err!("unexpected line type in [Fonts] section")),
            }
        }

        Ok(None)
    }
}

fn try_some_fontname(font_name: FontName) -> Result<Option<String>> {
    let fontname = str::from_utf8(font_name.name())?;
    Ok(Some(fontname.into()))
}

fn decode_font_line(writer: &mut BufWriter<File>, line_bytes: &[u8]) -> Result<()> {
    let mut iter = line_bytes.into_iter();

    loop {
        let mut chunk = [0u8; 4];
        let mut count = 0;

        for item in iter.by_ref().take(4) {
            if !(33..=96).contains(item) {
                return Err(err!("invalid font symbol"));
            }
            chunk[count] = item - 33;
            count += 1;
        }

        if count == 0 {
            return Ok(());
        }

        if count >= 2 {
            let b0 = (chunk[0] << 2) | (chunk[1] >> 4);
            writer.write_all(&[b0])?;
        } else {
            return Err(err!("invalid font block length"));
        }

        if count >= 3 {
            let b1 = (chunk[1] << 4) | (chunk[2] >> 2);
            writer.write_all(&[b1])?;
        }

        if count == 4 {
            let b2 = (chunk[2] << 6) | chunk[3];
            writer.write_all(&[b2])?;
        }
    }
}
