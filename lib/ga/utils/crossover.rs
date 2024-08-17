#![allow(unused)]

// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashSet;
use rand::{prelude::Distribution, Rng};

// Functions ///////////////////////////////////////////////////////////////////
pub fn variable_single_point<'a, T>(
    a: &'a [T],
    b: &'a [T],
    rate: f32,
) -> (Vec<&'a T>, Vec<&'a T>) {
    // Randomness
    let mut rng = rand::thread_rng();

    if rng.gen_range(0. ..=1.) > rate {
        return (a.iter().collect(), b.iter().collect());
    };

    // Split Index
    let interval = rand::distributions::Uniform::new_inclusive(1, a.len() - 2);
    let split_index = interval.sample(&mut rng);

    // Perform Crossover
    let mut switch_genes = false;
    let zip = a
        .iter()
        .zip(b.iter())
        .enumerate()
        .map(|(index, (a, b))| {
            // Check if genes must be switched
            if index == split_index {
                switch_genes = !switch_genes;
            }

            // Switch genes or not
            if switch_genes {
                (b, a)
            } else {
                (a, b)
            }
        })
        .collect::<Vec<(&T, &T)>>();

    let child_0 = zip.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let child_1 = zip.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    // Return
    (child_0, child_1)
}

pub fn variable_multi_point<'a, T>(
    points: usize,
    a: &'a [T],
    b: &'a [T],
    rate: f32,
) -> (Vec<&'a T>, Vec<&'a T>) {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0. ..=1.) > rate {
        return (a.iter().collect(), b.iter().collect());
    };

    let split_points =
        rand::distributions::Uniform::new_inclusive(1, (a.len() - 1) / 2);

    // Generating the same value twice will result in fewer splits.
    let mut splits_set = HashSet::new();
    for _ in 0..points {
        let new_split = split_points.sample(&mut rng) * 2;
        splits_set.insert(new_split);
    }

    let mut switch = false;
    let zip = a
        .iter()
        .zip(b.iter())
        .enumerate()
        .map(|(i, (a, b))| {
            if splits_set.contains(&i) {
                switch = !switch;
            }

            if switch {
                (b, a)
            } else {
                (a, b)
            }
        })
        .collect::<Vec<(&T, &T)>>();

    let x = zip.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = zip.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    (x, y)
}

// Tests ///////////////////////////////////////////////////////////////////////
// TODO: write tests for the functions above

////////////////////////////////////////////////////////////////////////////////
