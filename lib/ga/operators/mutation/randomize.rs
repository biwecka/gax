// Imports /////////////////////////////////////////////////////////////////////
use rand::{rngs::ThreadRng, Rng};

// Functions ///////////////////////////////////////////////////////////////////

/// Randomizes the genes based on a single random number distribution, which
/// is passed as parameter and creates valid gene values for all genes.
///
/// Genes may have different value ranges. For this case check out the
/// [`randomize_multi_dist`] functions.
///
pub fn randomize_single_dist<'a, T, D: rand::distributions::Distribution<T>>(
    chromosome: &'a mut [T],
    rate: f32,
    generator: &D,
    rng: &mut ThreadRng,
) {
    for gene in chromosome.iter_mut() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Mutate the gene
        *gene = generator.sample(rng);
    }
}

/// This mutation implementation takes into account, that genes at different
/// loci may have different ranges of valid values. Therefore this function
/// takes the `generators` parameter, which contains a separate random number
/// generator for each gene. Therefore the index of the gene in the chromosome
/// corresponds to the index for the respective random number generator.
pub fn randomize_multi_dist<'a, T, D: rand::distributions::Distribution<T>>(
    chromosome: &'a mut [T],
    rate: f32,
    generators: &[D],
    rng: &mut ThreadRng,
) {
    assert_eq!(chromosome.len(), generators.len());

    for (i, gene) in chromosome.iter_mut().enumerate() {
        // Decide wether to mutate or not
        if rng.gen_range(0. ..=1.) > rate {
            continue;
        }

        // Mutate the gene
        *gene = generators[i].sample(rng);
    }
}

////////////////////////////////////////////////////////////////////////////////
