//! How to detect the features of the current CPU:
//! `rustc --print cfg -C target-cpu=native`.
//!
//! You can also use `is_x86_feature_detected!("avx2")` at runtime, to check
//! if the CPU supports a certain feature.
//!

#![feature(portable_simd)]
use std::simd::prelude::*;

fn main() {
    let vec: Vec<i8> = vec![-1, 0, 0, 1, 0, -1];
    let (prefix, simd, suffix) = vec.as_simd::<2>();
    println!("prefix    = {:?}", prefix);
    println!("simd      = {:?}", simd);
    println!("suffix    = {:?}", suffix);

    let prefix_ =
        prefix.iter().map(|x| if *x < 0 { 0 } else { *x }).collect::<Vec<i8>>();
    let simd_ = simd
        .iter()
        .copied()
        .map(|x| {
            let mask = x.simd_eq(Simd::splat(-1));

            mask.select(Simd::splat(0), x)
        })
        .collect::<Vec<Simd<i8, 2>>>();
    let suffix_ =
        suffix.iter().map(|x| if *x < 0 { 0 } else { *x }).collect::<Vec<i8>>();

    println!("prefix'   = {:?}", prefix_);
    println!("simd'     = {:?}", simd_);
    println!("suffix'   = {:?}", suffix_);

    let line = Line(vec![
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
    ]);
    let line2 = Line(vec![
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 1, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, 1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 1, 1, 0, 1, 1, 1, 0,
        -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1,
        1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0,
        1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, -1, -1, -1, 1, 0, 1, 0, -1, 1,
        -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
        1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0, 1, 1, 1, 0, -1, -1,
        -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 1, -1, 1, 0, 1, 1, 1,
        0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 0, 0, 1, 0,
        1, 1, 1, 1, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, 1, 1, 1, 1, 1,
        1, 0, 1, 1, 1, 0, -1, 1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, 1, 1, -1,
        0, 0, 1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 1, 0, 0, -1, -1, 1, 0, 1,
    ]);

    let start = std::time::Instant::now();
    // let mut results = vec![];
    for _ in 0..1_000_000 {
        let x = line.remove_negative_1_simd();
        let y = line2.remove_negative_1_simd();
        let _z = x.or_simd(&y);
    }

    // println!("results = {results:?}");

    let elapsed = start.elapsed();
    println!("elapsed = {:?}", elapsed);
}

#[allow(unused)]
struct Matrix {
    rows: u16,
    columns: u16,
    data: Vec<i8>,
}

impl Matrix {
    #[allow(unused)]
    pub fn init(rows: u16, columns: u16) -> Self {
        Self { rows, columns, data: vec![0; rows as usize * columns as usize] }
    }

    #[allow(unused)]
    pub fn get(&self, row: u16, column: u16) -> i8 {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index as usize]
    }

    #[allow(unused)]
    pub fn get_row(&self, row: u16) -> &[i8] {
        assert!(row < self.rows);

        let offset = row as usize * self.columns as usize;
        let end = offset + self.columns as usize - 1;

        &self.data[offset..=end]
    }

    #[allow(unused)]
    pub fn get_col(&self, column: u16) -> Line {
        assert!(column < self.columns);

        let mut indices = vec![];
        for i in
            (column..).step_by(self.columns as usize).take(self.rows as usize)
        {
            indices.push(i);
        }

        let mut result = vec![];
        for i in indices {
            result.push(self.data[i as usize]);
        }

        Line(result)
    }
}

#[allow(unused)]
struct LineRef<'a>(pub &'a [i8]);

#[derive(Debug)]
struct Line(pub Vec<i8>);

impl Line {
    #[allow(unused)]
    #[inline(always)]
    pub fn remove_negative_1_simd(&self) -> Self {
        let (prefix, simd, suffix) = self.0.as_simd::<32>();

        let prefix_new = prefix
            .iter()
            .map(|x| if *x <= 0 { 0 } else { *x })
            .collect::<Vec<i8>>();

        let suffix_new = suffix
            .iter()
            .map(|x| if *x <= 0 { 0 } else { *x })
            .collect::<Vec<i8>>();

        let simd_new = simd
            .iter()
            .map(|x| {
                // Create a mask with "1" at the position of each "-1".
                let mask_negative_1 = x.simd_eq(Simd::splat(-1));

                // Select 0 for each "1" in the mask, otherwise take value from x.
                mask_negative_1.select(Simd::splat(0), *x)
            })
            .collect::<Vec<Simd<i8, 32>>>();

        Self(
            [
                prefix_new,
                simd_new.into_iter().map(|x| x.to_array()).flatten().collect(),
                suffix_new,
            ]
            .concat(),
        )
    }

    pub fn remove_negative_1(&self) -> Self {
        let result = self
            .0
            .iter()
            .map(|x| if *x <= 0 { 0 } else { *x })
            .collect::<Vec<i8>>();

        Self(result)
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn or_simd(&self, other: &Self) -> Self {
        let (p0, simd0, s0) = self.0.as_simd::<32>();
        let (p1, simd1, s1) = other.0.as_simd::<32>();

        let p_res =
            p0.iter().zip(p1.iter()).map(|(a, b)| a | b).collect::<Vec<i8>>();
        let s_res =
            s0.iter().zip(s1.iter()).map(|(a, b)| a | b).collect::<Vec<i8>>();
        let simd_res = simd0
            .iter()
            .zip(simd1.iter())
            .map(|(a, b)| a | b)
            .collect::<Vec<Simd<i8, 32>>>();

        Self(
            [
                p_res,
                simd_res.into_iter().map(|x| x.to_array()).flatten().collect(),
                s_res,
            ]
            .concat(),
        )
    }

    pub fn or(&self, other: &Self) -> Self {
        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a | b)
            .collect::<Vec<i8>>();

        Self(result)
    }
}
