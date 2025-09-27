use super::Retiming;
use crate::{Result, TrackOrderItemRetimed, TrackType};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

impl Retiming<'_, '_> {
    pub(super) fn try_video(&self, src: &Path, track: u64) -> Result<TrackOrderItemRetimed> {
        if track == self.track && src == self.base.as_path() {
            self.try_base_video()
        } else {
            Err("Unsupported retiming more than 1 video track at a time".into())
        }
    }

    fn try_base_video(&self) -> Result<TrackOrderItemRetimed> {
        let ty = TrackType::Video;

        let parts: Vec<PathBuf> = self
            .parts
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                if p.no_retiming {
                    return Ok(PathBuf::from(&p.src));
                }
                let dest = self
                    .temp_dir
                    .join(format!("{}-vid-base-{}.mkv", self.thread, i));

                self.try_split(&p.src, self.track, ty, &dest, p.start, p.end)
                    .map(|_| dest)
            })
            .collect::<Result<_>>()?;

        let no_retiming: Vec<_> = self.parts.iter().map(|p| p.no_retiming).collect();

        Ok(TrackOrderItemRetimed {
            ty,
            parts,
            no_retiming,
        })
    }
}
