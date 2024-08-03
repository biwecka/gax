use rand::{distributions::Uniform, prelude::Distribution};
use xhstt::db::Database;

use crate::encoding::chromosome::Chromosome;




pub fn dynamic_single_point(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = db.events().len();

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

pub fn static_single_point(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = db.events().len();

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        let p0_parts = p0.0.split_at(325);
        let p1_parts = p1.0.split_at(325);

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