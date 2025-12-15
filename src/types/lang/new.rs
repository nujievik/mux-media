use super::{Lang, LangCode};
use crate::{IsDefault, MuxError, Result};
use std::{env, str::FromStr};

/// Returns a [`Lang::Code`].
#[macro_export]
macro_rules! lang {
    ($x:ident) => {
        $crate::Lang::Code($crate::LangCode::$x)
    };
}

impl Lang {
    pub(crate) fn new(s: impl AsRef<str>) -> Lang {
        let s = s.as_ref();
        match get_code(s) {
            Some(c) => Lang::Code(c),
            None => Lang::Other(s.into()),
        }
    }
}

impl LangCode {
    pub(crate) fn init() -> Self {
        Self::get_from_system_locale().unwrap_or_default()
    }

    pub(crate) fn get(s: &str) -> Option<LangCode> {
        get_code(s)
    }

    fn get_from_system_locale() -> Option<Self> {
        let locale = env::var("LC_ALL")
            .ok()
            .or_else(|| env::var("LANG").ok())
            .or_else(|| env::var("LC_MESSAGES").ok())
            .or_else(|| get_system_locale_fallback())?;

        return get_code(&locale);

        fn get_system_locale_fallback() -> Option<String> {
            #[cfg(windows)]
            {
                use std::ffi::OsString;
                use std::os::windows::ffi::OsStringExt;
                use winapi::um::winnls::GetUserDefaultLocaleName;

                const LOCALE_NAME_MAX_LENGTH: usize = 85;
                let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH];

                let len = unsafe {
                    GetUserDefaultLocaleName(buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32)
                };

                if len > 0 {
                    let os_str = OsString::from_wide(&buffer[..(len as usize - 1)]);
                    os_str.into_string().ok()
                } else {
                    None
                }
            }

            #[cfg(unix)]
            {
                None
            }
        }
    }
}

impl Default for Lang {
    fn default() -> Lang {
        Lang::Code(LangCode::default())
    }
}
impl IsDefault for Lang {
    fn is_default(&self) -> bool {
        matches!(self, Lang::Code(c) if c.is_default())
    }
}

impl FromStr for Lang {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<Lang> {
        Ok(Lang::new(s))
    }
}
impl FromStr for LangCode {
    type Err = MuxError;

    fn from_str(s: &str) -> Result<LangCode> {
        get_code(s).ok_or_else(|| err!("Not found a valid language code"))
    }
}

fn get_code(s: &str) -> Option<LangCode> {
    fn str_to_ascii_words(s: &str) -> impl Iterator<Item = &str> {
        use lazy_regex::{Lazy, Regex, regex};
        static REGEX_ASCII_WORD: &Lazy<Regex> = regex!(r"[a-zA-Z]+");
        REGEX_ASCII_WORD.find_iter(s).map(|mat| mat.as_str())
    }

    let mut buf = [0u8; 3];
    str_to_ascii_words(s).find_map(|s| {
        let len = s.len();
        if !matches!(len, 2 | 3) {
            return None;
        }
        for (dst, src) in buf[..len].iter_mut().zip(s.bytes()) {
            *dst = src.to_ascii_lowercase();
        }

        let code = match &buf[..len] {
            b"und" => LangCode::Und,
            b"aa" | b"aar" => LangCode::Aar,
            b"ab" | b"abk" => LangCode::Abk,
            b"af" | b"afr" => LangCode::Afr,
            b"ak" | b"aka" => LangCode::Aka,
            b"sq" | b"sqi" | b"alb" => LangCode::Alb,
            b"am" | b"amh" => LangCode::Amh,
            b"ar" | b"ara" => LangCode::Ara,
            b"an" | b"arg" => LangCode::Arg,
            b"hy" | b"hye" | b"arm" => LangCode::Arm,
            b"as" | b"asm" => LangCode::Asm,
            b"av" | b"ava" => LangCode::Ava,
            b"ae" | b"ave" => LangCode::Ave,
            b"ay" | b"aym" => LangCode::Aym,
            b"az" | b"aze" => LangCode::Aze,
            b"ba" | b"bak" => LangCode::Bak,
            b"bm" | b"bam" => LangCode::Bam,
            b"eu" | b"eus" | b"baq" => LangCode::Baq,
            b"be" | b"bel" => LangCode::Bel,
            b"bn" | b"ben" => LangCode::Ben,
            b"bi" | b"bis" => LangCode::Bis,
            b"bs" | b"bos" => LangCode::Bos,
            b"br" | b"bre" => LangCode::Bre,
            b"bg" | b"bul" => LangCode::Bul,
            b"my" | b"mya" | b"bur" => LangCode::Bur,
            b"ca" | b"cat" => LangCode::Cat,
            b"ch" | b"cha" => LangCode::Cha,
            b"ce" | b"che" => LangCode::Che,
            b"zh" | b"zho" | b"chi" => LangCode::Chi,
            b"cu" | b"chu" => LangCode::Chu,
            b"cv" | b"chv" => LangCode::Chv,
            b"kw" | b"cor" => LangCode::Cor,
            b"co" | b"cos" => LangCode::Cos,
            b"cr" | b"cre" => LangCode::Cre,
            b"cs" | b"ces" | b"cze" => LangCode::Cze,
            b"da" | b"dan" => LangCode::Dan,
            b"dv" | b"div" => LangCode::Div,
            b"nl" | b"nld" | b"dut" => LangCode::Dut,
            b"dz" | b"dzo" => LangCode::Dzo,
            b"en" | b"eng" => LangCode::Eng,
            b"eo" | b"epo" => LangCode::Epo,
            b"et" | b"est" => LangCode::Est,
            b"ee" | b"ewe" => LangCode::Ewe,
            b"fo" | b"fao" => LangCode::Fao,
            b"fj" | b"fij" => LangCode::Fij,
            b"fi" | b"fin" => LangCode::Fin,
            b"fr" | b"fra" | b"fre" => LangCode::Fre,
            b"fy" | b"fry" => LangCode::Fry,
            b"ff" | b"ful" => LangCode::Ful,
            b"ka" | b"kat" | b"geo" => LangCode::Geo,
            b"de" | b"der" | b"ger" => LangCode::Ger,
            b"gd" | b"gla" => LangCode::Gla,
            b"ga" | b"gle" => LangCode::Gle,
            b"gl" | b"glg" => LangCode::Glg,
            b"gb" | b"glv" => LangCode::Glv,
            b"el" | b"ell" | b"gre" => LangCode::Gre,
            b"gn" | b"grn" => LangCode::Grn,
            b"gu" | b"guj" => LangCode::Guj,
            b"ht" | b"hat" => LangCode::Hat,
            b"ha" | b"hau" => LangCode::Hau,
            b"sh" | b"hbs" => LangCode::Hbs,
            b"he" | b"heb" => LangCode::Heb,
            b"hz" | b"her" => LangCode::Her,
            b"hi" | b"hin" => LangCode::Hin,
            b"ho" | b"hmo" => LangCode::Hmo,
            b"hr" | b"hrv" => LangCode::Hrv,
            b"hu" | b"hun" => LangCode::Hun,
            b"ig" | b"ibo" => LangCode::Ibo,
            b"is" | b"isl" | b"ice" => LangCode::Ice,
            b"io" | b"ido" => LangCode::Ido,
            b"ii" | b"iii" => LangCode::Iii,
            b"iu" | b"iku" => LangCode::Iku,
            b"ie" | b"ile" => LangCode::Ile,
            b"ia" | b"ina" => LangCode::Ina,
            b"id" | b"ind" => LangCode::Ind,
            b"ik" | b"ipk" => LangCode::Ipk,
            b"it" | b"ita" => LangCode::Ita,
            b"jv" | b"jav" => LangCode::Jav,
            b"ja" | b"jpn" => LangCode::Jpn,
            b"kl" | b"kal" => LangCode::Kal,
            b"kn" | b"kan" => LangCode::Kan,
            b"ks" | b"kas" => LangCode::Kas,
            b"kr" | b"kau" => LangCode::Kau,
            b"kk" | b"kaz" => LangCode::Kaz,
            b"km" | b"khm" => LangCode::Khm,
            b"ki" | b"kik" => LangCode::Kik,
            b"rw" | b"kin" => LangCode::Kin,
            b"ky" | b"kir" => LangCode::Kir,
            b"kv" | b"kom" => LangCode::Kom,
            b"kg" | b"kon" => LangCode::Kon,
            b"ko" | b"kor" => LangCode::Kor,
            b"kj" | b"kua" => LangCode::Kua,
            b"ku" | b"kur" => LangCode::Kur,
            b"lo" | b"lao" => LangCode::Lao,
            b"la" | b"lat" => LangCode::Lat,
            b"lv" | b"lav" => LangCode::Lav,
            b"li" | b"lim" => LangCode::Lim,
            b"ln" | b"lin" => LangCode::Lin,
            b"lt" | b"lit" => LangCode::Lit,
            b"lb" | b"ltz" => LangCode::Ltz,
            b"lu" | b"lub" => LangCode::Lub,
            b"lg" | b"lug" => LangCode::Lug,
            b"mk" | b"mkd" | b"mac" => LangCode::Mac,
            b"mh" | b"mah" => LangCode::Mah,
            b"ml" | b"mal" => LangCode::Mal,
            b"mi" | b"mri" | b"mao" => LangCode::Mao,
            b"mr" | b"mar" => LangCode::Mar,
            b"ms" | b"msa" | b"may" => LangCode::May,
            b"mg" | b"mlg" => LangCode::Mlg,
            b"mt" | b"mlt" => LangCode::Mlt,
            b"mn" | b"mon" => LangCode::Mon,
            b"na" | b"nau" => LangCode::Nau,
            b"nv" | b"nav" => LangCode::Nav,
            b"nr" | b"nbl" => LangCode::Nbl,
            b"nd" | b"nde" => LangCode::Nde,
            b"ng" | b"ndo" => LangCode::Ndo,
            b"ne" | b"nep" => LangCode::Nep,
            b"nn" | b"nno" => LangCode::Nno,
            b"nb" | b"nob" => LangCode::Nob,
            b"no" | b"nor" => LangCode::Nor,
            b"ny" | b"nya" => LangCode::Nya,
            b"oc" | b"oci" => LangCode::Oci,
            b"oj" | b"oji" => LangCode::Oji,
            b"or" | b"ori" => LangCode::Ori,
            b"om" | b"orm" => LangCode::Orm,
            b"os" | b"oss" => LangCode::Oss,
            b"pa" | b"pan" => LangCode::Pan,
            b"fa" | b"fas" | b"per" => LangCode::Per,
            b"pi" | b"pli" => LangCode::Pli,
            b"pl" | b"pol" => LangCode::Pol,
            b"pt" | b"por" => LangCode::Por,
            b"ps" | b"pus" => LangCode::Pus,
            b"qu" | b"que" => LangCode::Que,
            b"rm" | b"roh" => LangCode::Roh,
            b"ro" | b"ron" | b"rum" => LangCode::Rum,
            b"rn" | b"run" => LangCode::Run,
            b"ru" | b"rus" => LangCode::Rus,
            b"sg" | b"sag" => LangCode::Sag,
            b"sa" | b"san" => LangCode::San,
            b"si" | b"sin" => LangCode::Sin,
            b"sk" | b"slk" | b"slo" => LangCode::Slo,
            b"sl" | b"slv" => LangCode::Slv,
            b"se" | b"sme" => LangCode::Sme,
            b"sm" | b"smo" => LangCode::Smo,
            b"sn" | b"sna" => LangCode::Sna,
            b"sd" | b"snd" => LangCode::Snd,
            b"so" | b"som" => LangCode::Som,
            b"st" | b"sot" => LangCode::Sot,
            b"es" | b"spa" => LangCode::Spa,
            b"sc" | b"srd" => LangCode::Srd,
            b"sr" | b"srp" => LangCode::Srp,
            b"ss" | b"ssw" => LangCode::Ssw,
            b"su" | b"sun" => LangCode::Sun,
            b"sw" | b"swa" => LangCode::Swa,
            b"sv" | b"swe" => LangCode::Swe,
            b"ty" | b"tah" => LangCode::Tah,
            b"ta" | b"tam" => LangCode::Tam,
            b"tt" | b"tat" => LangCode::Tat,
            b"te" | b"tel" => LangCode::Tel,
            b"tg" | b"tgk" => LangCode::Tgk,
            b"tl" | b"tgl" => LangCode::Tgl,
            b"th" | b"tha" => LangCode::Tha,
            b"bo" | b"bod" | b"tib" => LangCode::Tib,
            b"ti" | b"tir" => LangCode::Tir,
            b"to" | b"ton" => LangCode::Ton,
            b"tn" | b"tsn" => LangCode::Tsn,
            b"ts" | b"tso" => LangCode::Tso,
            b"tk" | b"tuk" => LangCode::Tuk,
            b"tr" | b"tur" => LangCode::Tur,
            b"tw" | b"twi" => LangCode::Twi,
            b"ug" | b"uig" => LangCode::Uig,
            b"uk" | b"ukr" => LangCode::Ukr,
            b"ur" | b"urd" => LangCode::Urd,
            b"uz" | b"uzb" => LangCode::Uzb,
            b"ve" | b"ven" => LangCode::Ven,
            b"vi" | b"vie" => LangCode::Vie,
            b"vo" | b"vol" => LangCode::Vol,
            b"cy" | b"cym" | b"wel" => LangCode::Wel,
            b"wa" | b"wln" => LangCode::Wln,
            b"wo" | b"wol" => LangCode::Wol,
            b"xh" | b"xho" => LangCode::Xho,
            b"yi" | b"yid" => LangCode::Yid,
            b"yo" | b"yor" => LangCode::Yor,
            b"za" | b"zha" => LangCode::Zha,
            b"zu" | b"zul" => LangCode::Zul,
            _ => return None,
        };

        Some(code)
    })
}
