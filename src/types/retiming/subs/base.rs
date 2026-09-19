use super::*;
use std::collections::HashMap;
use subtitle_lines::{SubtitleLines, WriteOptions};

impl Retiming<'_, '_> {
    pub(super) fn try_base_sub(&self, i_stream: usize, dest: &Destination) -> Result<()> {
        let mut opts = WriteOptions::new();
        let mut sources: HashMap<&Path, PathBuf> = HashMap::with_capacity(self.parts.len());
        let mut splits: Vec<PathBuf> = Vec::with_capacity(self.parts.len());

        for (i_part, p) in self.parts.iter().enumerate() {
            let src_path = p.src.as_path();

            if !sources.contains_key(src_path) {
                let path = self.temp_dir.join(format!(
                    "{}-sub-base-{}-src-{}.{}",
                    self.job,
                    i_stream,
                    i_part,
                    dest.ty.as_ext()
                ));
                try_extract(src_path, i_stream, dest.ty, &path)?;
                sources.insert(src_path, path);
            }

            let extracted_stream = sources.get(src_path).unwrap();

            opts.start = Some(p.start);
            opts.end = Some(p.end);

            let offset = SignedTime::new(
                true,
                self.len_prev_nonuid_parts(i_part) + self.len_prev_uid_parts(i_part),
            ) - p.start;

            if offset.is_positive() {
                opts.add_time = Some(offset.as_unsigned_time());
                opts.sub_time = None;
            } else {
                opts.sub_time = Some(offset.as_unsigned_time());
                opts.add_time = None;
            }

            let lines = SubtitleLines::open_file(extracted_stream)?;
            let split = dest.destination_split(i_part);

            lines.write_with(&split, &opts)?;
            splits.push(split);
        }

        merge(dest, &splits)
    }
}
