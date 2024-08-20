#![allow(unused)]

use std::hash::Hash;

// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::{HashMap, HashSet};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

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

pub fn uniform<'a, T>(
    a: &'a [T],
    b: &'a [T],
    rate: f32,
) -> (Vec<&'a T>, Vec<&'a T>) {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0. ..=1.) > rate {
        return (a.iter().collect(), b.iter().collect());
    }

    // Create random true/false generator.
    let switch_genes = Uniform::<f32>::new(0., 1.);

    let zip =
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| {
                if switch_genes.sample(&mut rng) > 0.5 {
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

pub fn ordered<T: std::fmt::Debug + Clone + PartialEq + Eq>(
    a: Vec<T>,
    b: Vec<T>,
    rate: f32,
) -> (Vec<T>, Vec<T>) {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0. ..=1.) > rate {
        return (a.to_owned(), b.to_owned());
    }

    // Get chromosome len
    let clen = a.len();

    // Create two random points/indices for the middle part of the ordered
    // crossover.
    let i0 = rng.gen_range(0..(clen - 1));
    let i1 = rng.gen_range((i0 + 1)..clen); // This ensures that i1 > i0

    println!("i0 i1 = {} {}", i0, i1);

    // Create first child
    let c0: Vec<T> = {
        // Middle part comes from parent a
        let middle = &a[i0..i1];
        println!("middle    = {middle:?}");

        // Get a view of parent b without the values from `middle`.
        let remainder = b
            .iter()
            .filter_map(|x| {
                if !middle.contains(x) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();

        // The part after the middle (called tail) of this child is now filled
        // from the calculated remainder.
        println!("remainder = {remainder:?}");
        let tail = &remainder[0..(clen - i1)];

        // The part befor the middle (called head) of this child is now filled
        // with the remaining values from the remainder.
        let head = &remainder[(clen - i1)..];

        // Concatenate the three parts to get the full chromosome
        [head, middle, tail].concat()
    };

    // Create second child
    let c1: Vec<T> = {
        // Middle part comes from parent b
        let middle = &b[i0..i1];

        // Get a view of parent a without the values from `middle`.
        let remainder = a
            .iter()
            .filter_map(|x| {
                if !middle.contains(x) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();

        // The part after the middle (called tail) of this child is now filled
        // from the calculated remainder.
        let tail = &remainder[0..(clen - i1)];

        // The part befor the middle (called head) of this child is now filled
        // with the remaining values from the remainder.
        let head = &remainder[(clen - i1)..];

        // Concatenate the three parts to get the full chromosome
        [head, middle, tail].concat()
    };

    // Return
    (c0, c1)
}

pub fn pmx<'a, T: Eq + Hash>(
    a: &'a [T],
    b: &'a [T],
    rate: f32,
) -> (Vec<&'a T>, Vec<&'a T>) {
    // Get chromosome length
    let clen = a.len();

    // Randomness
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, clen);

    // Calculate lower and upper "cut" indices
    let (lower, upper) = {
        let mut l = dist.sample(&mut rng);
        let mut r = dist.sample(&mut rng);

        while l.abs_diff(r) < 2 {
            l = dist.sample(&mut rng);
            r = dist.sample(&mut rng);
        }

        if l < r {
            (l, r)
        } else {
            (r, l)
        }
    };

    // Split a into parts
    let a_l = &a[0..lower];
    let a_m = &a[lower..upper];
    let a_r = &a[upper..];

    // Split b into parts
    let b_l = &b[0..lower];
    let b_m = &b[lower..upper];
    let b_r = &b[upper..];

    // Create matcher
    let matcher = PmxMatcher::new(a_m, b_m);

    // Create child 0
    let c0_l = a_l.iter().map(|x| matcher.calc_x_to_y(x)).collect::<Vec<_>>();
    let c0_m = b_m;
    let c0_r = a_r.iter().map(|x| matcher.calc_x_to_y(x)).collect::<Vec<_>>();

    let mut c0 = c0_l.to_vec();
    c0.extend(c0_m);
    c0.extend(c0_r);

    // Create child 1
    let c1_l = b_l.iter().map(|x| matcher.calc_y_to_x(x)).collect::<Vec<_>>();
    let c1_m = a_m;
    let c1_r = b_r.iter().map(|x| matcher.calc_y_to_x(x)).collect::<Vec<_>>();

    let mut c1 = c1_l.to_vec();
    c1.extend(c1_m);
    c1.extend(c1_r);

    // Return
    (c0, c1)
}

// Helper Structs //////////////////////////////////////////////////////////////
struct PmxMatcher<'a, T: Eq + Hash> {
    x_to_y: HashMap<&'a T, &'a T>,
    y_to_x: HashMap<&'a T, &'a T>,
}

impl<'a, T: Eq + Hash> PmxMatcher<'a, T> {
    pub fn new(x: &'a [T], y: &'a [T]) -> Self {
        assert_eq!(x.len(), y.len());

        let mut x_to_y = HashMap::new();
        let mut y_to_x = HashMap::new();

        for i in 0..x.len() {
            let a = &x[i];
            let b = &y[i];

            x_to_y.insert(b, a);
            y_to_x.insert(a, b);
        }

        Self { x_to_y, y_to_x }
    }

    pub fn calc_x_to_y(&self, input: &'a T) -> &'a T {
        let mut result = input;

        while let Some(x) = self.x_to_y.get(&result) {
            result = *x;
        }

        result
    }

    pub fn calc_y_to_x(&self, input: &'a T) -> &'a T {
        let mut result = input;

        while let Some(x) = self.y_to_x.get(&result) {
            result = *x;
        }

        result
    }
}

// Tests ///////////////////////////////////////////////////////////////////////
// TODO: write tests for the functions above

////////////////////////////////////////////////////////////////////////////////
