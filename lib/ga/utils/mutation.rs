#![allow(unused)]

// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashSet;
use rand::{prelude::Distribution, Rng};

// Functions ///////////////////////////////////////////////////////////////////
pub fn randomize_n_genes<'a, T, D: rand::distributions::Distribution<T>>(
    amount: usize,
    chromosome: &'a mut [T],
    rate: f32,
    random_gene_value: D,
) {
    // Randomness
    let mut rng = rand::thread_rng();

    // Check
    if rng.gen_range(0. ..=1.) > rate {
        return;
    };

    // Perform mutation
    let random_gene_index = rand::distributions::Uniform::new(0, chromosome.len());

    for _ in 0..amount {
        let index = random_gene_index.sample(&mut rng);
        chromosome[index] = random_gene_value.sample(&mut rng);
    }
}

// Tests ///////////////////////////////////////////////////////////////////////
// TODO: write tests for the functions above

////////////////////////////////////////////////////////////////////////////////
