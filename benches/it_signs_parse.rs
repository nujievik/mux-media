#![feature(test)]
extern crate test;

use test::{Bencher, black_box};

fn str_to_words(s: &str) -> impl Iterator<Item = &str> {
    s.split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
}

fn parse_old(s: &str) -> bool {
    str_to_words(s).any(|s| {
        let s = s.to_lowercase();
        matches!(s.as_str(), "signs" | "надписи")
    })
}

macro_rules! matches_bytes {
    ($bytes:expr; $( $i:expr, $pattern:pat ),* $(,)?) => {{
        true $( && matches!($bytes[$i], $pattern) )*
    }};
}

fn parse_new(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        let b = bytes[i];

        // signs
        if matches!(b, b's' | b'S') {
            if i + 5 > len {
                return false;
            }

            if i + 5 == len || !is_alphabetic_byte(bytes[i + 5]) {
                if matches_bytes!(
                    bytes;
                i + 1, b'i' | b'I',
                i + 2, b'g' | b'G',
                i + 3, b'n' | b'N',
                i + 4, b's' | b'S'
                ) {
                    return true;
                }
            }
        }

        // russian надписи
        // н / Н
        if matches!(b, 0xD0) && i + 14 <= len {
            if matches!(bytes[i + 1], 0xBD | 0x9D)
                && (i + 14 == len || !is_alphabetic_byte(bytes[i + 14]))
            {
                if matches_bytes!(
                    &bytes[i..i+14];
                    // а / А
                    2, 0xD0, 3, 0xB0 | 0x90,
                    // д / Д
                    4, 0xD0, 5, 0xB4 | 0x94,
                    // п / П
                    6, 0xD0, 7, 0xBF | 0x9F,
                    // и / И
                    8, 0xD0, 9, 0xB8 | 0x98,
                    // и / И
                    12, 0xD0, 13, 0xB8 | 0x98
                ) && (
                    // с
                    (matches!(bytes[i + 10], 0xD1) && matches!(bytes[i + 11], 0x81))
                        ||
                        // С
                        (matches!(bytes[i + 10], 0xD0) && matches!(bytes[i + 11], 0xA1))
                ) {
                    return true;
                }
            }
        }

        i += 1;
        while i < len && is_alphabetic_byte(bytes[i]) {
            i += 1;
        }
        while i < len && !is_alphabetic_byte(bytes[i]) {
            i += 1;
        }
    }

    false
}

#[inline(always)]
fn is_alphabetic_byte(b: u8) -> bool {
    b.is_ascii_alphabetic() || matches!(b, 0xD0 | 0xD1)
}

const TEXT_SIGNS: &str = "This text contains Signs inside";
const TEXT_NADPISI: &str = "Здесь есть слово НаДпИсИ в тексте";
const TEXT_NO_MATCH: &str = "Совершенно другой текст без нужных слов";

#[bench]
fn bench_old_signs_match(b: &mut Bencher) {
    b.iter(|| parse_old(black_box(TEXT_SIGNS)));
}

#[bench]
fn bench_old_nadpisi_match(b: &mut Bencher) {
    b.iter(|| parse_old(black_box(TEXT_NADPISI)));
}

#[bench]
fn bench_old_no_match(b: &mut Bencher) {
    b.iter(|| parse_old(black_box(TEXT_NO_MATCH)));
}

#[bench]
fn bench_new_signs_match(b: &mut Bencher) {
    b.iter(|| parse_new(black_box(TEXT_SIGNS)));
}

#[bench]
fn bench_new_nadpisi_match(b: &mut Bencher) {
    b.iter(|| parse_new(black_box(TEXT_NADPISI)));
}

#[bench]
fn bench_new_no_match(b: &mut Bencher) {
    b.iter(|| parse_new(black_box(TEXT_NO_MATCH)));
}
