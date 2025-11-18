use super::*;
use crate::{Stream, markers::*};

impl MediaInfo<'_> {
    pub(crate) fn it_signs(&mut self, src: &Path, stream: &Stream) -> bool {
        if !stream.ty.is_sub() {
            return false;
        }

        return parse(stream.name.as_ref().map(|v| &**v))
            || parse(self.get(MIPathTail, src))
            || parse(self.get(MIRelativeUpmost, src));

        fn parse(opt_s: Option<&String>) -> bool {
            opt_s.is_some_and(|s| {
                str_to_words(s).any(|s| {
                    let s = s.to_lowercase();
                    matches!(s.as_str(), "signs" | "надписи")
                })
            })
        }

        fn str_to_words(s: &str) -> impl Iterator<Item = &str> {
            use lazy_regex::{Lazy, Regex, regex};
            static REGEX_WORD: &Lazy<Regex> = regex!(r"[a-zA-Z]+|[а-яА-ЯёЁ]+");
            REGEX_WORD.find_iter(s).map(|mat| mat.as_str().trim())
        }
    }
}
