// Imports /////////////////////////////////////////////////////////////////////
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Distribution;

// Functions ///////////////////////////////////////////////////////////////////

/// Swaps the genes based on a single random number distribution, which
/// is passed as parameter and creates valid gene values for all genes.
pub fn swap_uniform_dist<D: rand::distributions::Distribution<usize>>(
    chromosome: &mut [usize],
    rate: f32,
    generator: &D,
    rng: &mut ThreadRng,
) {
    for i in 0..chromosome.len() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Generate swap index
        let mut si = generator.sample(rng);
        while si == i {
            si = generator.sample(rng);
        }

        // Swap genes
        chromosome.swap(i, si);
    }
}

pub fn swap_uniform_dist_u8<D: rand::distributions::Distribution<u8>>(
    chromosome: &mut [u8],
    rate: f32,
    generator: &D,
    rng: &mut ThreadRng,
) {
    for i in 0..chromosome.len() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Generate swap index
        let mut si = generator.sample(rng);
        while si == i as u8 {
            si = generator.sample(rng);
        }

        // Swap genes
        chromosome.swap(i, si as usize);
    }
}

pub fn swap_normal_dist(
    chromosome: &mut [usize],
    rate: f32,
    generator: &rand_distr::Normal<f32>,
    rng: &mut ThreadRng,
) {
    for i in 0..chromosome.len() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Generate the swap index. As the random number generator is a normal
        // distribution with a mean of 0, the generated number is added to the
        // current index.
        let mut offset = generator.sample(rng).round() as i32;
        let mut si = i as i32 + offset;

        while offset == 0 || si < 0 || si >= chromosome.len() as i32 {
            offset = generator.sample(rng).round() as i32;
            si = i as i32 + offset;
        }

        // Swap genes
        chromosome.swap(i, si as usize);
    }
}

pub fn swap_normal_dist_u8(
    chromosome: &mut [u8],
    rate: f32,
    generator: &rand_distr::Normal<f32>,
    rng: &mut ThreadRng,
) {
    for i in 0..chromosome.len() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Generate the swap index. As the random number generator is a normal
        // distribution with a mean of 0, the generated number is added to the
        // current index.
        let mut offset = generator.sample(rng).round() as i32;
        let mut si = i as i32 + offset;

        while offset == 0 || si < 0 || si >= chromosome.len() as i32 {
            offset = generator.sample(rng).round() as i32;
            si = i as i32 + offset;
        }

        // Swap genes
        chromosome.swap(i, si as usize);
    }
}

////////////////////////////////////////////////////////////////////////////////
