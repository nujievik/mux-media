use std::env::var;

fn main() {
    if var("CARGO_FEATURE_STATIC").is_err() {
        return;
    }

    todo!()
}
