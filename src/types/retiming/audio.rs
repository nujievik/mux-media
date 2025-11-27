use super::{RetimedStream, Retiming};
use crate::{Duration, Result, Tool, Tools};
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(crate) fn try_audio(&self, i: usize, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if self.is_save_single_part() && src == **self.base {
            return Ok(self.single_part_base_retimed_stream(src, i_stream));
        }

        let splits = if src == **self.base {
            self.try_base_audio(i_stream)
        } else {
            self.try_external_audio(i, src, i_stream)
        }?;

        let txt = self
            .temp_dir
            .join(format!("{}-aud-splits-{}.txt", self.thread, i));

        let dest = self.temp_dir.join(format!("{}-aud-{}.mka", self.thread, i));
        self.try_concat(&splits, &txt, &dest)?;

        Ok(RetimedStream {
            src: Some(dest),
            i_stream: 0,
            src_time: None,
        })
    }

    fn try_base_audio(&self, i_stream: usize) -> Result<Vec<PathBuf>> {
        self.parts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-base-{}-{}.mka", self.thread, i_stream, i));

                try_split(&self.tools, &p.src, i_stream, &dest, p.start, p.end)?;
                Ok(dest)
            })
            .collect()
    }

    fn try_external_audio(&self, i: usize, src: &Path, i_stream: usize) -> Result<Vec<PathBuf>> {
        let mut segments: Vec<PathBuf> = Vec::with_capacity(self.chapters.len());

        for p in self.parts.iter() {
            let uid = &self.chapters[p.i_start_chp].uid;
            for i_chp in p.i_start_chp..=p.i_end_chp {
                let chp = &self.chapters[i_chp];
                if uid != &chp.uid {
                    continue;
                }
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-{}-{}.mka", self.thread, i, i_chp));

                let chp_nonuid = self.chapters_nonuid(i_chp);

                let start = chp.start.as_secs_f64() + p.start_offset + chp_nonuid;
                let end_offset = if i_chp == p.i_end_chp {
                    p.end_offset
                } else {
                    p.start_offset
                };
                let end = chp.end.as_secs_f64() + end_offset + chp_nonuid;

                let trg_start = Duration::from_secs_f64(start);
                let trg_end = Duration::from_secs_f64(end);
                try_split(&self.tools, src, i_stream, &dest, trg_start, trg_end)?;

                segments.push(dest);
            }
        }

        Ok(segments)
    }
}

fn try_split(
    tools: &Tools,
    src: &Path,
    i_stream: usize,
    dest: &Path,
    trg_start: Duration,
    trg_end: Duration,
) -> Result<()> {
    let trg_start = trg_start.to_string();
    let trg_end = trg_end.to_string();
    let map = format!("0:{}", i_stream);

    let args = [
        p!("-y"),
        p!("-ss"),
        p!(&trg_start),
        p!("-to"),
        p!(&trg_end),
        p!("-i"),
        src,
        p!("-map"),
        p!(&map),
        dest,
    ];
    let _ = tools.run(Tool::Ffmpeg, &args)?;
    Ok(())
}
