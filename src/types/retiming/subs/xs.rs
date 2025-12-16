use super::SubType;
use crate::Result;
use std::{fs, io::Write, path::Path};
use time::Time;

#[derive(Debug)]
pub enum Subs {
    Srt(rsubs_lib::SRT),
    Ssa(rsubs_lib::SSA),
    Vtt(rsubs_lib::VTT),
}

impl Subs {
    pub fn new(src: &Path, ty: SubType) -> Result<Subs> {
        let s = fs::read_to_string(src)?;
        let subs = match ty {
            SubType::Srt => Subs::Srt(rsubs_lib::SRT::parse(s)?),
            SubType::Ssa => Subs::Ssa(rsubs_lib::SSA::parse(s)?),
            SubType::Vtt => Subs::Vtt(rsubs_lib::VTT::parse(s)?),
        };
        Ok(subs)
    }
}

macro_rules! box_iter {
    ($sub:ident, $field:ident) => {
        Box::new(
            $sub.$field
                .iter()
                .enumerate()
                .map(|(i, f)| (i, f.start.into(), f.end.into())),
        )
    };
}

impl Subs {
    pub fn iter_i_start_end(&self) -> Box<dyn Iterator<Item = (usize, Time, Time)> + '_> {
        match self {
            Self::Srt(sub) => box_iter!(sub, lines),
            Self::Ssa(sub) => box_iter!(sub, events),
            Self::Vtt(sub) => box_iter!(sub, lines),
        }
    }

    pub fn try_write(&self, dest: &Path) -> Result<()> {
        let mut file = fs::File::create(dest)?;
        match self {
            Self::Srt(s) => write!(file, "{}", s),
            Self::Ssa(s) => write!(file, "{}", s),
            Self::Vtt(s) => write!(file, "{}", s),
        }?;
        Ok(())
    }
}
