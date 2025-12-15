use mux_media::Extension;

fn case_permutations(s: &str) -> Vec<String> {
    fn helper(chars: &[char], current: String, result: &mut Vec<String>) {
        if chars.is_empty() {
            result.push(current);
        } else {
            let mut lower = current.clone();
            lower.push(chars[0].to_ascii_lowercase());
            helper(&chars[1..], lower, result);

            let mut upper = current;
            upper.push(chars[0].to_ascii_uppercase());
            helper(&chars[1..], upper, result);
        }
    }

    let mut result = Vec::new();
    helper(&s.chars().collect::<Vec<_>>(), String::new(), &mut result);
    result
}

#[test]
fn new() {
    for ext in <Extension as strum::IntoEnumIterator>::iter() {
        let s: &str = ext.as_ref();
        for permut in case_permutations(s) {
            assert_eq!(ext, Extension::new(permut.as_bytes()).unwrap());
        }
    }
}

#[test]
fn new_unsupported() {
    fn generate(count: usize) -> Vec<String> {
        use rand::{Rng, seq::IteratorRandom};
        use std::collections::HashSet;

        let mut rng = rand::rng();
        let charset = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut fake_exts = HashSet::with_capacity(count);
        let mut attempts = 0;

        while fake_exts.len() < count && attempts < count * 10 {
            attempts += 1;

            let len = rng.random_range(2..6);
            let candidate: String = (0..len)
                .map(|_| *charset.iter().choose(&mut rng).unwrap() as char)
                .collect();

            if Extension::new(candidate.as_bytes()).is_none() {
                fake_exts.insert(candidate);
            }
        }

        fake_exts.into_iter().collect()
    }

    let fake_exts = generate(1000);
    assert_ne!(0, fake_exts.len());
}
