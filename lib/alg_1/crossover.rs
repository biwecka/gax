// Imports /////////////////////////////////////////////////////////////////////
use crate::{fitness::Cost, population::Chromosome, stats::Stats};
use rand::{distributions::Uniform, prelude::Distribution};

// Functions ///////////////////////////////////////////////////////////////////
pub fn changing_single_point(
    parent_pairs: Vec<((Chromosome, Cost), (Chromosome, Cost))>,
    stats: &Stats,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = stats.events.len();

    // Randomness
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, chr_len);

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        let split_index = dist.sample(&mut rng);
        let p0_parts = p0.0.split_at(split_index);
        let p1_parts = p1.0.split_at(split_index);

        let c0 = [p0_parts.0, p1_parts.1].concat();
        let c1 = [p1_parts.0, p0_parts.1].concat();

        assert_eq!(c0.len(), chr_len);
        assert_eq!(c1.len(), chr_len);

        children.push(Chromosome(c0));
        children.push(Chromosome(c1));
    }

    // Return
    children
}

pub fn fixed_single_point(
    parent_pairs: Vec<((Chromosome, Cost), (Chromosome, Cost))>,
    stats: &Stats,
    split_index: usize,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = stats.events.len();

    // Randomness
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, chr_len);
    // let split_index = dist.sample(&mut rng);

    assert!(split_index > 0);
    assert!(split_index < chr_len);

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        let p0_parts = p0.0.split_at(split_index);
        let p1_parts = p1.0.split_at(split_index);

        let c0 = [p0_parts.0, p1_parts.1].concat();
        let c1 = [p1_parts.0, p0_parts.1].concat();

        assert_eq!(c0.len(), chr_len);
        assert_eq!(c1.len(), chr_len);

        children.push(Chromosome(c0));
        children.push(Chromosome(c1));
    }

    // Return
    children
}

////////////////////////////////////////////////////////////////////////////////
