use super::*;

impl Streams {
    /// Whether save stream with index and language.
    pub fn is_save(&self, i: &usize, lang: &Lang) -> bool {
        if self.no_flag {
            return false;
        }

        if self.is_default() {
            return true;
        }

        if self.idxs.as_ref().is_some_and(|xs| xs.contains(i)) {
            return !self.inverse;
        }

        if self
            .ranges
            .as_ref()
            .is_some_and(|xs| xs.iter().any(|x| x.contains(i)))
        {
            return !self.inverse;
        }

        if self.langs.as_ref().is_some_and(|xs| xs.contains(lang)) {
            return !self.inverse;
        }

        self.inverse
    }
}
