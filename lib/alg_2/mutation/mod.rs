use rand::{distributions::Uniform, prelude::Distribution};
use xhstt::db::Database;

use crate::encoding::chromosome::Chromosome;

#[allow(unused)]
pub fn random_single(
    mut children: Vec<Chromosome>,
    mutation_propability: f32,
    db: &Database,
) -> Vec<Chromosome> {
    // let mut mutated_children = Vec::with_capacity(children.len());

    // Get chromosome length
    let chr_len = db.events().len();

    // Randomness
    let mut rng = rand::thread_rng();
    let index = Uniform::new(0, chr_len);
    let times = Uniform::new(0, db.times().len());
    let probabilty = Uniform::new_inclusive(0., 1.);

    // Iterate and mutate
    for child in children.iter_mut() {
        if probabilty.sample(&mut rng) > mutation_propability {
            continue;
        }

        let i = index.sample(&mut rng);
        let t = times.sample(&mut rng);

        child.0[i] = t as u8;
    }

    // Return
    children
}
