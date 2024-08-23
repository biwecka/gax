// Imports /////////////////////////////////////////////////////////////////////
use rand::{rngs::ThreadRng, Rng};

// Functions ///////////////////////////////////////////////////////////////////

/// Randomizes the genes based on a single random number distribution, which
/// is passed as parameter and creates valid gene values for all genes.
///
/// Genes may have different value ranges. For this case check out the
/// [`randomize_multi_dist`] functions.
///
pub fn swap<D: rand::distributions::Distribution<usize>>(
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

////////////////////////////////////////////////////////////////////////////////
