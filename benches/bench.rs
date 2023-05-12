#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// https://bheisler.github.io/criterion.rs/book/index.html

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn f(_i: usize) {}
pub fn bench(c: &mut Criterion) { c.bench_function("bench", |b| b.iter(|| f(black_box(1)))); }

criterion_group!(benches, bench);
criterion_main!(benches);
