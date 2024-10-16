// Imports /////////////////////////////////////////////////////////////////////
use bits::Bits32;
use bitvec::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Benchmarks //////////////////////////////////////////////////////////////////
pub fn criterion_benchmark(c: &mut Criterion) {
    let x = Bits32::new(28, 0b1101_0111_0100_1101_1110_1011_1111_1010);

    c.bench_function("groups(1..=6)", |b| {
        b.iter(|| {
            black_box(x.groups(1));
            black_box(x.groups(2));
            black_box(x.groups(3));
            black_box(x.groups(4));
            black_box(x.groups(5));
            black_box(x.groups(6));
        })
    });

    let bits32 = Bits32::new(28, 0b1101_0111_0100_1101_1110_1011_1111_1010);

    c.bench_function("bits32 AND", |b| {
        b.iter(|| {
            black_box(bits32 & 3_281_013);
        })
    });

    let mut bitvec = bitvec![u32, Lsb0; 0; 124];
    let bitvec2 = bitvec![u32, Lsb0; 0; 23523];

    c.bench_function("bitvec AND", |b| {
        b.iter(|| {
            black_box(bitvec.clone() & bitvec2.clone());
        })
    });

    c.bench_function("bits32 <<", |b| {
        b.iter(|| {
            black_box(bits32 << 1);
        })
    });

    c.bench_function("bitvec <<", |b| {
        b.iter(|| {
            bitvec.shift_left(1);
            black_box(&bitvec);
        })
    });
}

// Main ////////////////////////////////////////////////////////////////////////
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

////////////////////////////////////////////////////////////////////////////////
