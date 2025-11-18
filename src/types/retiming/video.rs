use super::{RetimedStream, Retiming};
use crate::{Duration, Result, Tool, Tools};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(super) fn try_video(&self, src: &Path, i_stream: usize) -> Result<RetimedStream> {
        if i_stream != self.i_base_stream && src != self.base.as_path() {
            return Err(err!(
                "Unsupported retiming more than 1 video track at a time. Skipping {} stream {}",
                src.display(),
                i_stream
            ));
        }

        if self.parts.len() == 1 {
            Ok(self.single_part_base_retimed_stream(src, i_stream))
        } else {
            self.try_base_video()
        }
    }

    pub(super) fn init_base_splits(&mut self) -> Result<()> {
        const SHIFT_START: f64 = 0.3;
        const SHIFT_END: f64 = -0.1;

        let raw_splits: Vec<(usize, f64, PathBuf)> = self
            .parts
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let split = self
                    .temp_dir
                    .join(format!("{}-vid-base-{}.mkv", self.thread, i));

                let start = if p.start.0.is_zero() {
                    p.start
                } else {
                    Duration::from_secs_f64(p.start.as_secs_f64() + SHIFT_START)
                };
                let end = Duration::from_secs_f64(p.end.as_secs_f64() - SHIFT_END);

                try_split(&self.tools, &p.src, self.i_base_stream, &split, start, end)
                    .map(|dur| (i, dur, split))
            })
            .collect::<Result<_>>()?;

        let base_splits: Vec<_> = raw_splits
            .into_iter()
            .map(|(i, dur, split)| {
                let p = &mut self.parts[i];
                let end_f64 = p.end.as_secs_f64();
                let old_dur = end_f64 - p.start.as_secs_f64();
                let add_offset = dur - old_dur;

                p.end = Duration::from_secs_f64(end_f64 + add_offset);
                p.end_offset += add_offset;
                split
            })
            .collect();

        self.base_splits = base_splits;
        return Ok(());

        fn try_split(
            tools: &Tools,
            src: &Path,
            i_stream: usize,
            dest: &Path,
            trg_start: Duration,
            trg_end: Duration,
        ) -> Result<f64> {
            use lazy_regex::{Lazy, Regex, regex};
            static REGEX_END_DURATION: &Lazy<Regex> = regex!(r"end duration =\s+(\d+)");

            let trg_start = trg_start.to_string();
            let trg_end = trg_end.to_string();
            let map = format!("0:{}", i_stream);

            let args = [
                p!("-y"),
                p!("-loglevel"),
                p!("debug"),
                p!("-ss"),
                p!(&trg_start),
                p!("-to"),
                p!(&trg_end),
                p!("-i"),
                src,
                p!("-map"),
                p!(&map),
                p!("-c"),
                p!("copy"),
                dest,
            ];
            let out = tools.run(Tool::Ffmpeg, &args)?;

            match REGEX_END_DURATION
                .captures(&out.stderr)
                .and_then(|cap| cap.get(1).and_then(|s| s.as_str().parse::<u64>().ok()))
            {
                Some(ms) => Ok(ms as f64 / 1000.0),
                None => Err(err!("Fail get end split")),
            }
        }
    }

    fn try_base_video(&self) -> Result<RetimedStream> {
        let txt = self
            .temp_dir
            .join(format!("{}-vid-base-parts.txt", self.thread));

        let dest = self.temp_dir.join(format!("{}-vid-base.mkv", self.thread));
        self.try_concat(&self.base_splits, &txt, &dest)?;

        Ok(RetimedStream {
            src: Some(dest),
            i_stream: 0,
            src_time: None,
        })
    }
}
