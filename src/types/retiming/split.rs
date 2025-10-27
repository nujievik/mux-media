use super::Retiming;
use crate::{Duration, IsDefault, MediaInfo, Result, Tool, ToolOutput, Tools, TrackType};
use lazy_regex::{Lazy, Regex, regex};
use std::path::Path;

static REGEX_TIME_SPLIT: &Lazy<Regex> =
    regex!(r"Timestamp used in split decision: (\d{2}:\d{2}:\d{2}\.\d{9})");

impl Retiming<'_, '_> {
    pub(super) fn try_split(
        &self,
        src: &Path,
        tid: u64,
        ty: TrackType,
        dest: &Path,
        trg_start: Duration,
        trg_end: Duration,
    ) -> Result<(Duration, f64, Duration, f64)> {
        let out = run(&self.tools, src, tid, ty, dest, trg_start, trg_end)?;
        let (start, end, was_end_of_src) = try_start_end(trg_start, out, dest)?;

        // Real playback <= track duration.
        let end_offset = if was_end_of_src && end <= trg_end {
            0f64
        } else {
            end.as_secs_f64() - trg_end.as_secs_f64()
        };
        let start_offset = start.as_secs_f64() - trg_start.as_secs_f64();

        return Ok((start, start_offset, end, end_offset));

        fn try_start_end(
            trg_start: Duration,
            out: ToolOutput,
            dest: &Path,
        ) -> Result<(Duration, Duration, bool)> {
            let mut it = REGEX_TIME_SPLIT
                .captures_iter(out.as_str_stdout())
                .map(|cap| {
                    cap[1]
                        .parse::<Duration>()
                        .or_else(|e| Err(err!("Unexpected fail parse timestamp: {}", e)))
                });

            let first = it.next();

            if let Some(end) = it.next() {
                // unwrap is safe: first Some if second Some.
                return Ok((first.unwrap()?, end?, false));
            }

            let try_play = || -> Result<_> {
                MediaInfo::help_build_matroska(dest)?
                    .info
                    .duration
                    .map(|d| Duration::from(d))
                    .ok_or_else(|| err!("Unexpected None duration"))
            };

            match first {
                Some(Err(e)) => Err(e),
                // When the trg_start is zero, it is not shifted; so take `f` as the end.
                Some(Ok(f)) if trg_start.is_default() => Ok((Duration::default(), f, false)),
                // otherwise, `f` is start time.
                Some(Ok(f)) => Ok((f, f + try_play()?, true)),
                None => Ok((Duration::default(), try_play()?, true)),
            }
        }

        fn run(
            tools: &Tools,
            src: &Path,
            tid: u64,
            ty: TrackType,
            dest: &Path,
            trg_start: Duration,
            trg_end: Duration,
        ) -> Result<ToolOutput> {
            let p: fn(&str) -> &Path = Path::new;

            let s_parts = format!("parts:{}-{}", trg_start, trg_end);
            let s_tid = format!("{}", tid);

            let args = [
                p("-o"),
                dest,
                p("--split"),
                p(&s_parts),
                p("--no-chapters"),
                p("--no-global-tags"),
                p("--no-subtitles"),
                p("--no-attachments"),
                p(ty.as_mkvmerge_arg()),
                p(&s_tid),
                if matches!(ty, TrackType::Audio) {
                    p("--no-video")
                } else {
                    p("--no-audio")
                },
                src,
            ];

            tools.run(Tool::Mkvmerge, &args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_time_split() {
        [
            "00:00:01.000000000",
            "00:00:05.000000000",
            "00:00:01.000400000",
            "00:00:01.100000000",
        ]
        .iter()
        .for_each(|time| {
            let compare_with = |add: &str| {
                let s = format!("{}Timestamp used in split decision: {}", add, time);
                let found = REGEX_TIME_SPLIT
                    .captures_iter(&s)
                    .map(|cap| cap[1].to_owned())
                    .next()
                    .unwrap();
                assert_eq!(time, &found);
            };

            compare_with("\n");
            compare_with(" ");
            compare_with(".");
            compare_with(",");
            compare_with(":");
            compare_with("123");
        })
    }
}
