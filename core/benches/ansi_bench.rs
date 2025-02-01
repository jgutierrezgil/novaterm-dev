use criterion::{black_box, criterion_group, criterion_main, Criterion};
use core::ansi::parse_ansi;

fn bench_complex_ansi(c: &mut Criterion) {
    let data = b"\x1b[38;2;255;0;0mHola \x1b]8;;https://rust-lang.org\x07Mundo\x1b]8;;\x07\x1b[0m";
    
    c.bench_function("parse_complex_ansi", |b| {
        b.iter(|| parse_ansi(black_box(data)))
    });
}

criterion_group!(benches, bench_complex_ansi);
criterion_main!(benches);