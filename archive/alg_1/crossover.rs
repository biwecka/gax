// Imports /////////////////////////////////////////////////////////////////////
use crate::{fitness::Cost, population::Chromosome, stats::Stats};
use rand::{distributions::Uniform, prelude::Distribution};

// Functions ///////////////////////////////////////////////////////////////////
#[allow(unused)]
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

#[allow(unused)]
pub fn changing_multi_point(
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
        let mut split_indices = vec![];
        for _ in 0..5 {
            // Get random (don't allow duplicates)
            let mut random = dist.sample(&mut rng);
            while split_indices.contains(&random) {
                random = dist.sample(&mut rng);
            }

            split_indices.push(random);
        }

        split_indices.sort();

        let i0 = split_indices[0];
        let i1 = split_indices[1] - i0;
        let i2 = split_indices[2] - i0 - i1;
        let i3 = split_indices[3] - i0 - i1 - i2;
        let i4 = split_indices[4] - i0 - i1 - i2 - i3;

        let (p0_0, rest) = p0.0.split_at(i0);
        let (p0_1, rest) = rest.split_at(i1);
        let (p0_2, rest) = rest.split_at(i2);
        let (p0_3, rest) = rest.split_at(i3);
        let (p0_4, p0_5) = rest.split_at(i4);

        let (p1_0, rest) = p0.0.split_at(i0);
        let (p1_1, rest) = rest.split_at(i1);
        let (p1_2, rest) = rest.split_at(i2);
        let (p1_3, rest) = rest.split_at(i3);
        let (p1_4, p1_5) = rest.split_at(i4);

        let c0 = [p0_0, p1_1, p0_2, p1_3, p0_4, p1_5].concat();
        let c1 = [p1_0, p0_1, p1_2, p0_3, p1_4, p0_5].concat();

        assert_eq!(c0.len(), chr_len);
        assert_eq!(c1.len(), chr_len);

        children.push(Chromosome(c0));
        children.push(Chromosome(c1));
    }

    // Return
    children
}

////////////////////////////////////////////////////////////////////////////////
