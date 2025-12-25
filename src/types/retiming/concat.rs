use super::Retiming;
use crate::Result;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

impl Retiming<'_, '_> {
    pub(crate) fn try_concat(&self, splits: &Vec<PathBuf>, txt: &Path, out: &Path) -> Result<()> {
        todo!();
        /*
        write_txt(splits, &txt)?;

        let args = [
            p!("-y"),
            p!("-f"),
            p!("concat"),
            p!("-safe"),
            p!("0"),
            p!("-i"),
            txt,
            p!("-c"),
            p!("copy"),
            out,
        ];
        let _ = self.tools.run(Tool::Ffmpeg, &args)?;

        return Ok(());

        fn write_txt(splits: &Vec<PathBuf>, txt: &Path) -> Result<()> {
            let mut f = File::create(txt)?;
            for p in splits {
                match p.to_str() {
                    Some(s) => writeln!(f, "file '{}'", s)?,
                    None => return Err(err!("Unsupported utf-8 symbol in path '{}'", p.display())),
                }
            }
            Ok(())
        }
        */
    }
}
