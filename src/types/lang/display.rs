use super::{Lang, LangCode};
use std::fmt;

impl LangCode {
    /// Prints the list of supported language codes to stdout.
    pub(crate) fn print_list_langs() {
        println!("{}", LIST_LANGS)
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = match self {
            Lang::Code(c) => c.as_ref(),
            Lang::Other(s) => s,
        };
        write!(f, "{}", s)
    }
}
impl fmt::Display for LangCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <Self as AsRef<str>>::as_ref(self))
    }
}

static LIST_LANGS: &str = r#"English language name      | Triple code    | Duo code       | Alt triple code
---------------------------+----------------+----------------+----------------
Abkhazian                  | abk            | ab             |
Afar                       | aar            | aa             |
Afrikaans                  | afr            | af             |
Akan                       | aka            | ak             |
Albanian                   | alb            | sq             | sqi
Amharic                    | amh            | am             |
Arabic                     | ara            | ar             |
Aragonese                  | arg            | an             |
Armenian                   | arm            | hy             | hye
Assamese                   | asm            | as             |
Avaric                     | ava            | av             |
Avestan                    | ave            | ae             |
Aymara                     | aym            | ay             |
Azerbaijani                | aze            | az             |
Bambara                    | bam            | bm             |
Bashkir                    | bak            | ba             |
Basque                     | baq            | eu             | eus
Belarusian                 | bel            | be             |
Bengali                    | ben            | bn             |
Bislama                    | bis            | bi             |
Bosnian                    | bos            | bs             |
Breton                     | bre            | br             |
Bulgarian                  | bul            | bg             |
Burmese                    | bur            | my             | mya
Catalan                    | cat            | ca             |
Chamorro                   | cha            | ch             |
Chechen                    | che            | ce             |
Chinese                    | chi            | zh             | zho
Church Slavic              | chu            | cu             |
Chuvash                    | chv            | cv             |
Cornish                    | cor            | kw             |
Corsican                   | cos            | co             |
Cree                       | cre            | cr             |
Croatian                   | hrv            | hr             |
Czech                      | cze            | cs             | ces
Danish                     | dan            | da             |
Dhivehi                    | div            | dv             |
Dutch                      | dut            | nl             | nld
Dzongkha                   | dzo            | dz             |
English                    | eng            | en             |
Esperanto                  | epo            | eo             |
Estonian                   | est            | et             |
Ewe                        | ewe            | ee             |
Faroese                    | fao            | fo             |
Fijian                     | fij            | fj             |
Finnish                    | fin            | fi             |
French                     | fre            | fr             | fra
Fulah                      | ful            | ff             |
Galician                   | glg            | gl             |
Ganda                      | lug            | lg             |
Georgian                   | geo            | ka             | kat
German                     | ger            | de             | deu
Greek (modern, 1453-)      | gre            | el             | ell
Guarani                    | grn            | gn             |
Gujarati                   | guj            | gu             |
Haitian                    | hat            | ht             |
Hausa                      | hau            | ha             |
Hebrew                     | heb            | he             |
Herero                     | her            | hz             |
Hindi                      | hin            | hi             |
Hiri Motu                  | hmo            | ho             |
Hungarian                  | hun            | hu             |
Icelandic                  | ice            | is             | isl
Ido                        | ido            | io             |
Igbo                       | ibo            | ig             |
Indonesian                 | ind            | id             |
Interlingua (IALA)         | ina            | ia             |
Interlingue                | ile            | ie             |
Inuktitut                  | iku            | iu             |
Inupiaq                    | ipk            | ik             |
Irish                      | gle            | ga             |
Italian                    | ita            | it             |
Japanese                   | jpn            | ja             |
Javanese                   | jav            | jv             |
Kalaallisut                | kal            | kl             |
Kannada                    | kan            | kn             |
Kanuri                     | kau            | kr             |
Kashmiri                   | kas            | ks             |
Kazakh                     | kaz            | kk             |
Khmer                      | khm            | km             |
Kikuyu                     | kik            | ki             |
Kinyarwanda                | kin            | rw             |
Kirghiz                    | kir            | ky             |
Komi                       | kom            | kv             |
Kongo                      | kon            | kg             |
Korean                     | kor            | ko             |
Kuanyama                   | kua            | kj             |
Kurdish                    | kur            | ku             |
Lao                        | lao            | lo             |
Latin                      | lat            | la             |
Latvian                    | lav            | lv             |
Limburgan                  | lim            | li             |
Lingala                    | lin            | ln             |
Lithuanian                 | lit            | lt             |
Luba-Katanga               | lub            | lu             |
Luxembourgish              | ltz            | lb             |
Macedonian                 | mac            | mk             | mkd
Malagasy                   | mlg            | mg             |
Malay (macrolanguage)      | may            | ms             | msa
Malayalam                  | mal            | ml             |
Maltese                    | mlt            | mt             |
Manx                       | glv            | gv             |
Maori                      | mao            | mi             | mri
Marathi                    | mar            | mr             |
Marshallese                | mah            | mh             |
Mongolian                  | mon            | mn             |
Nauru                      | nau            | na             |
Navajo                     | nav            | nv             |
Ndonga                     | ndo            | ng             |
Nepali (macrolanguage)     | nep            | ne             |
North Ndebele              | nde            | nd             |
Northern Sami              | sme            | se             |
Norwegian Bokmål           | nob            | nb             |
Norwegian Nynorsk          | nno            | nn             |
Norwegian                  | nor            | no             |
Nyanja                     | nya            | ny             |
Occitan (post 1500)        | oci            | oc             |
Ojibwa                     | oji            | oj             |
Oriya (macrolanguage)      | ori            | or             |
Oromo                      | orm            | om             |
Ossetian                   | oss            | os             |
Pali                       | pli            | pi             |
Panjabi                    | pan            | pa             |
Persian                    | per            | fa             | fas
Polish                     | pol            | pl             |
Portuguese                 | por            | pt             |
Pushto                     | pus            | ps             |
Quechua                    | que            | qu             |
Romanian                   | rum            | ro             | ron
Romansh                    | roh            | rm             |
Rundi                      | run            | rn             |
Russian                    | rus            | ru             |
Samoan                     | smo            | sm             |
Sango                      | sag            | sg             |
Sanskrit                   | san            | sa             |
Sardinian                  | srd            | sc             |
Scottish Gaelic            | gla            | gd             |
Serbian                    | srp            | sr             |
Serbo-Croatian             | hbs            | sh             |
Shona                      | sna            | sn             |
Sichuan Yi                 | iii            | ii             |
Sindhi                     | snd            | sd             |
Sinhala                    | sin            | si             |
Slovak                     | slo            | sk             | slk
Slovenian                  | slv            | sl             |
Somali                     | som            | so             |
South Ndebele              | nbl            | nr             |
Southern Sotho             | sot            | st             |
Spanish                    | spa            | es             |
Sundanese                  | sun            | su             |
Swahili (macrolanguage)    | swa            | sw             |
Swati                      | ssw            | ss             |
Swedish                    | swe            | sv             |
Tagalog                    | tgl            | tl             |
Tahitian                   | tah            | ty             |
Tajik                      | tgk            | tg             |
Tamil                      | tam            | ta             |
Tatar                      | tat            | tt             |
Telugu                     | tel            | te             |
Thai                       | tha            | th             |
Tibetan                    | tib            | bo             | bod
Tigrinya                   | tir            | ti             |
Tonga (Tonga Islands)      | ton            | to             |
Tsonga                     | tso            | ts             |
Tswana                     | tsn            | tn             |
Turkish                    | tur            | tr             |
Turkmen                    | tuk            | tk             |
Twi                        | twi            | tw             |
Uighur                     | uig            | ug             |
Ukrainian                  | ukr            | uk             |
Urdu                       | urd            | ur             |
Uzbek                      | uzb            | uz             |
Venda                      | ven            | ve             |
Vietnamese                 | vie            | vi             |
Volapük                    | vol            | vo             |
Walloon                    | wln            | wa             |
Welsh                      | wel            | cy             | cym
Western Frisian            | fry            | fy             |
Wolof                      | wol            | wo             |
Xhosa                      | xho            | xh             |
Yiddish                    | yid            | yi             |
Yoruba                     | yor            | yo             |
Zhuang                     | zha            | za             |
Zulu                       | zul            | zu             |                "#;
