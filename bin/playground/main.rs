#![feature(test)]

use bitvec::prelude::*;

fn main() {
    let bits: BitVec<u8, Lsb0> = bitvec![
        u8, Lsb0; // Lsb -> 64 | Msb -> 2
        0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // bits.set(6, true);

    // Load the bits as a little-endian number
    let le_number = bits.load_le::<u16>(); // Little-endian interpretation

    // Load the bits as a big-endian number
    let be_number = bits.load_be::<u16>(); // Big-endian interpretation

    println!("Bits (Lsb0): {:?}", bits);
    println!("Little-endian interpretation: {}", le_number); // Output: 64
    println!("Big-endian interpretation: {}", be_number);    // Output: 16384

    println!("\n---------------------------------------------------------\n\n");

    let bits = bitvec_simd::BitVec::from_bool_iterator(
        vec![
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ]
            .into_iter()
            .map(|x| x > 0)
    );

    println!("{:?}", bits);


    println!("\n---------------------------------------------------------\n\n");

    let mut bit = fixedbitset::FixedBitSet::with_capacity(8);
    bit.set(0, true);
    bit.set(2, true);
    println!("{:b}", bit);

    let num = bit.ones().fold(0u64, |acc, indices| acc | 1 << indices);
    dbg!(num);


}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use bitvec::prelude::*;

    #[bench]
    fn bitvec(bench: &mut Bencher) {
        // Optionally include some setup
        let a: BitVec<u32, Lsb0> = bitvec![
            u32, Lsb0;
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 1, 0, 1, 0, 0, 0, 1,
        ];

        let b: BitVec<u32, Lsb0> = bitvec![
            u32, Lsb0;
            0, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 1, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 1, 1,
            0, 1, 0, 1, 0, 0, 0, 1,
        ];

        let c: BitVec<u32, Lsb0> = bitvec![
            u32, Lsb0;
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 1, 0, 0, 0, 0,
        ];

        let d: BitVec<u32, Lsb0> = bitvec![
            u32, Lsb0;
            0, 0, 0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 1, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 1, 0, 0, 0, 1,
        ];

        let values = vec![a, b, c, d];

        let mut x: BitVec<u32, Lsb0> = bitvec![u32, Lsb0;];

        bench.iter(|| {
            for i in 0..10_000_000 {
                let index = i % 4;
                black_box(x.clone() & &values[index]);
                // black_box(x.clone() & &values[index]);
            }
        });
    }

    #[bench]
    fn bitvec_simd(bench: &mut Bencher) {
        // Optionally include some setup
        let a = bitvec_simd::BitVec::from_bool_iterator(
            vec![
                0, 1, 1, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 0, 1, 0,
                1, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 1, 0,
                0, 1, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 1,
                0, 1, 0, 1, 0, 0, 0, 1,
            ]
                .into_iter()
                .map(|x| x > 0)
        );

        let b = bitvec_simd::BitVec::from_bool_iterator(
            vec![
                0, 1, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 1, 1, 0,
                1, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 1, 0, 1, 1,
                0, 1, 0, 1, 0, 0, 0, 1,
            ]
                .into_iter()
                .map(|x| x > 0)
        );

        let c = bitvec_simd::BitVec::from_bool_iterator(
            vec![
                0, 1, 1, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 1, 0, 0, 0, 1, 0,
                1, 0, 0, 1, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0,
                0, 1, 0, 1, 0, 0, 0, 0,
            ]
                .into_iter()
                .map(|x| x > 0)
        );

        let d = bitvec_simd::BitVec::from_bool_iterator(
            vec![
                0, 0, 0, 0, 1, 1, 1, 1,
                0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 1, 0, 0, 0, 1, 0,
                1, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 1, 0, 1, 0,
                0, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 1,
                0, 0, 0, 1, 0, 0, 0, 1,
            ]
                .into_iter()
                .map(|x| x > 0)
        );

        let values = vec![a, b, c, d];

        let mut x = bitvec_simd::BitVec::from_bool_iterator(
            vec![
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]
                .into_iter()
                .map(|x| x > 0)
        );

        bench.iter(|| {
            for i in 0..10_000_000 {
                let index = i % 4;
                black_box(x.clone() & &values[index]);
            }
        });
    }

    #[bench]
    fn fixedbitset(bench: &mut Bencher) {
        let mut a = fixedbitset::FixedBitSet::with_capacity(64);
        vec![
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 1, 0, 1, 0, 0, 0, 1,
        ]
            .into_iter()
            .enumerate()
            .for_each(|(index, bit)| a.set(index, bit > 0));

        let mut b = fixedbitset::FixedBitSet::with_capacity(64);
        vec![
            0, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 1, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 1, 1,
            0, 1, 0, 1, 0, 0, 0, 1,
        ]
            .into_iter()
            .enumerate()
            .for_each(|(index, bit)| b.set(index, bit > 0));

        let mut c = fixedbitset::FixedBitSet::with_capacity(64);
        vec![
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 1, 0, 0, 0, 0,
        ]
            .into_iter()
            .enumerate()
            .for_each(|(index, bit)| c.set(index, bit > 0));

        let mut d = fixedbitset::FixedBitSet::with_capacity(64);
        vec![
            0, 0, 0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 1, 0, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 1, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 1, 0, 0, 0, 1,
        ]
            .into_iter()
            .enumerate()
            .for_each(|(index, bit)| d.set(index, bit > 0));

        let values = vec![a, b, c, d];

        let mut x = fixedbitset::FixedBitSet::with_capacity(64);

        bench.iter(|| {
            for i in 0..10_000_000 {
                let index = i % 4;
                black_box(x.intersection(&values[index]));
            }
        });
    }
}
