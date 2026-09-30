#![feature(test)]
extern crate test;

use std::io::Read;
use std::mem::MaybeUninit;
use test::{Bencher, black_box};

const READ_LIMIT: usize = 128 * 1024;

fn get_source_data() -> Vec<u8> {
    vec![42u8; READ_LIMIT]
}

#[bench]
fn bench_zeroed_array(b: &mut Bencher) {
    let source_data = black_box(get_source_data());

    b.iter(|| {
        let mut reader = &source_data[..];

        let mut bytes = [0u8; READ_LIMIT];
        let bytes_read = reader.read(&mut bytes).unwrap();

        let slice = &bytes[..bytes_read];

        black_box((bytes, slice));
    });
}

#[bench]
fn bench_maybe_uninit_array(b: &mut Bencher) {
    let source_data = black_box(get_source_data());

    b.iter(|| {
        let mut reader = &source_data[..];

        let mut uninit_bytes = MaybeUninit::<[u8; READ_LIMIT]>::uninit();

        let bytes_array_ref: &mut [u8; READ_LIMIT] =
            unsafe { std::mem::transmute(&mut uninit_bytes) };

        let bytes_read = reader.read(bytes_array_ref).unwrap();

        let slice =
            unsafe { std::slice::from_raw_parts(uninit_bytes.as_ptr() as *const u8, bytes_read) };

        black_box((uninit_bytes, slice));
    });
}
