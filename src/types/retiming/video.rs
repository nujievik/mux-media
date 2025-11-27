use super::{RetimedStream, Retiming};
use crate::{Duration, Result, Tool, ToolOutput, Tools};
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

        let raw_splits: Vec<(usize, f64, f64, PathBuf)> = self
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
                let end = Duration::from_secs_f64(p.end.as_secs_f64() + SHIFT_END);

                try_split(&self.tools, &p.src, self.i_base_stream, &split, start, end)
                    .map(|(start, end)| (i, start, end, split))
            })
            .collect::<Result<_>>()?;

        let base_splits: Vec<_> = raw_splits
            .into_iter()
            .map(|(i, start, end, split)| {
                let p = &mut self.parts[i];
                p.start_offset += start - p.start.as_secs_f64();
                p.end_offset += end - p.end.as_secs_f64();

                p.start = Duration::from_secs_f64(start);
                p.end = Duration::from_secs_f64(end);
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
        ) -> Result<(f64, f64)> {
            let trg_start = trg_start.to_string();
            let trg_end = trg_end.to_string();
            let map = format!("0:{}", i_stream);

            let args = [
                p!("-y"),
                p!("-debug_ts"),
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

            return try_start_end(&out);

            fn try_start_end(out: &ToolOutput) -> Result<(f64, f64)> {
                use lazy_regex::{Lazy, Regex, regex};
                static OFFSET: &Lazy<Regex> = regex!(r" off_time:([-+]?[0-9]*\.?[0-9]+)");
                static PTS_DURATION_PAIR: &Lazy<Regex> = regex!(
                    r" pts_time:([-+]?[0-9]*\.?[0-9]+).*?duration_time:([-+]?[0-9]*\.?[0-9]+)"
                );

                let offset = OFFSET
                    .captures(&out.stderr)
                    .and_then(|caps| caps.get(1))
                    .and_then(|m| m.as_str().parse::<f64>().ok())
                    .ok_or_else(|| err!("Fail get frame offset"))?;

                let mut start: Option<f64> = None;
                let mut end: Option<(f64, f64)> = None;

                for caps in PTS_DURATION_PAIR.captures_iter(&out.stderr) {
                    let v = caps[1].parse::<f64>()?;
                    match start {
                        None => start = Some(v),
                        Some(prev) if prev > v => start = Some(v),
                        _ => (),
                    };
                    match end {
                        None => end = Some((v, caps[2].parse::<f64>()?)),
                        Some(prev) if prev.0 < v => end = Some((v, caps[2].parse::<f64>()?)),
                        _ => (),
                    }
                }

                if start.is_none() || end.is_none() {
                    return Err(err!("Fail get split start or end"));
                }

                let start = start.unwrap() - offset;
                let end = end.map(|(s, dur)| s + dur).unwrap() - offset;

                Ok((start, end))
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
