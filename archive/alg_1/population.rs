// Imports /////////////////////////////////////////////////////////////////////
use crate::stats::Stats;
use rand::distributions::{Distribution, Uniform};

// Structs /////////////////////////////////////////////////////////////////////
/// A gene is a random number in the interval [0; times) which specifies
/// the time (by index) for an event.
#[derive(Clone, Debug)]
pub struct Gene(pub usize);
impl From<usize> for Gene {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// A chromosome is an array of genes.
/// The gene index (locus) specifies the event.
#[derive(Clone, Debug)]
pub struct Chromosome(pub Vec<Gene>);
impl From<Vec<Gene>> for Chromosome {
    fn from(value: Vec<Gene>) -> Self {
        Self(value)
    }
}
impl From<Vec<usize>> for Chromosome {
    fn from(value: Vec<usize>) -> Self {
        let genes: Vec<Gene> = value.into_iter().map(|x| x.into()).collect();
        Self(genes)
    }
}

////////////////////////////////////////////////////////////////////////////////

// Functions ///////////////////////////////////////////////////////////////////
pub fn initialize(size: usize, stats: &Stats) -> Vec<Chromosome> {
    // Setup uniform distribution and source of randomness
    let mut rng = rand::thread_rng();
    let random_gene_values = Uniform::new(0, stats.times);

    // Create population
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let mut chromosome = Vec::<usize>::with_capacity(stats.events.len());
        for _ in 0..stats.events.len() {
            chromosome.push(random_gene_values.sample(&mut rng));
        }

        population.push(chromosome.into());
    }

    // Return
    population
}

////////////////////////////////////////////////////////////////////////////////
