use super::*;
use matroska::Matroska;

#[derive(Default)]
pub struct CacheMatroska(HashMap<ArcPathBuf, Option<Matroska>>);

impl CacheMatroska {
    pub fn get(&mut self, src: &ArcPathBuf) -> Option<&Matroska> {
        if self.0.get(src).is_none() {
            let v = matroska::open(src).ok();
            self.0.insert(src.clone(), v);
        }
        self.immut(src)
    }

    pub fn immut(&self, src: &Path) -> Option<&Matroska> {
        self.0.get(src).map_or(None, |v| v.as_ref())
    }
}
