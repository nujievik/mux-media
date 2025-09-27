use super::Retiming;
use crate::{Duration, Result, TrackOrderItemRetimed, TrackType};
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(crate) fn try_audio(
        &self,
        i: usize,
        src: &Path,
        track: u64,
    ) -> Result<TrackOrderItemRetimed> {
        let parts = if src == **self.base {
            self.try_base_audio(track)
        } else {
            self.try_external_audio(i, src, track)
        }?;

        Ok(TrackOrderItemRetimed {
            no_retiming: vec![false; parts.len()],
            parts,
            ty: TrackType::Audio,
        })
    }

    fn try_base_audio(&self, track: u64) -> Result<Vec<PathBuf>> {
        let mut len_offset = 0f64;

        // Not parallel! Tracking of len_offset for better sync.
        self.parts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-base-{}-{}.mka", self.thread, track, i));

                let start = match p.start.as_secs_f64() + len_offset {
                    f if f >= 0.0 => Duration::from_secs_f64(f),
                    _ => p.start,
                };
                let end = p.end;

                let dur = self.try_split_audio(&p.src, track, &dest, start, end)?;
                len_offset += p.end.as_secs_f64() - p.start.as_secs_f64() - dur;

                Ok(dest)
            })
            .collect()
    }

    fn try_external_audio(&self, idx: usize, src: &Path, track: u64) -> Result<Vec<PathBuf>> {
        let mut segments: Vec<PathBuf> = Vec::with_capacity(self.chapters.len());
        let mut len_offset = 0f64;

        // Not parallel! Tracking of len_offset for better sync.
        for p in self.parts.iter() {
            let uid = &self.chapters[p.i_start_chp].uid;
            for i_chp in p.i_start_chp..=p.i_end_chp {
                let chp = &self.chapters[i_chp];
                if uid != &chp.uid {
                    continue;
                }
                let dest = self
                    .temp_dir
                    .join(format!("{}-aud-{}-{}.mka", self.thread, idx, i_chp));

                let chp_nonuid = self.chapters_nonuid(i_chp);

                let start = chp.start.as_secs_f64() + p.start_offset + chp_nonuid;
                let end_offset = if i_chp == p.i_end_chp {
                    p.end_offset
                } else {
                    p.start_offset
                };
                let end = chp.end.as_secs_f64() + end_offset + chp_nonuid;

                let trg_start = Duration::from_secs_f64(start + len_offset);
                let trg_end = Duration::from_secs_f64(end);
                let dur = self.try_split_audio(src, track, &dest, trg_start, trg_end)?;

                len_offset += end - start - dur;
                segments.push(dest);
            }
        }

        Ok(segments)
    }

    fn try_split_audio(
        &self,
        src: &Path,
        track: u64,
        dest: &Path,
        trg_start: Duration,
        trg_end: Duration,
    ) -> Result<f64> {
        const ACCEPT_A_OFFSET: f64 = 0.1;
        const TY: TrackType = TrackType::Audio;

        let (mut start, start_offset, mut end, end_offset) =
            self.try_split(src, track, TY, dest, trg_start, trg_end)?;

        if start_offset.abs() > ACCEPT_A_OFFSET || end_offset.abs() > ACCEPT_A_OFFSET {
            let trg_start = match start.as_secs_f64() - start_offset {
                f if f >= 0.0 => Duration::from_secs_f64(f),
                _ => start,
            };
            let trg_end = Duration::from_secs_f64(end.as_secs_f64() - end_offset);

            if trg_start != start || trg_end != end {
                (start, _, end, _) = self.try_split(src, track, TY, dest, trg_start, trg_end)?;
            }
        }

        Ok(end.as_secs_f64() - start.as_secs_f64())
    }
}
