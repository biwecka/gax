use rand::{distributions::Uniform, prelude::Distribution};
use rayon::prelude::*;
use xhstt::db::Database;

use crate::encoding::chromosome::Chromosome;

#[allow(unused)]
pub fn random_single_swap(
    mut children: Vec<Chromosome>,
    mutation_propability: f32,
    db: &Database,
) -> Vec<Chromosome> {
    // Get chromosome length
    let chr_len = db.events().len();

    // // Randomness
    // let mut rng = rand::thread_rng();
    let index = Uniform::new(0, chr_len);
    let probabilty = Uniform::new_inclusive(0., 1.);

    // // Iterate and mutate
    // for child in children.iter_mut() {
    //     if probabilty.sample(&mut rng) > mutation_propability {
    //         continue;
    //     }

    //     let i0 = index.sample(&mut rng);
    //     let i1 = index.sample(&mut rng);

    //     let val0 = child.0[i0];
    //     let val1 = child.0[i1];

    //     child.0[i0] = val1;
    //     child.0[i1] = val0;
    // }

    // // Return
    // children


    children
        .into_par_iter()
        .map(|mut c| {
            let mut rng = rand::thread_rng();
            // let probabilty = Uniform::new_inclusive(0., 1.);
            if probabilty.sample(&mut rng) > mutation_propability {
                return c;
            }

            // let index = Uniform::new(0, chr_len);
            let i0 = index.sample(&mut rng);
            let i1 = index.sample(&mut rng);

            let val0 = c.0[i0];
            let val1 = c.0[i1];

            c.0[i0] = val1;
            c.0[i1] = val0;

            c
        })
        .collect()
}

#[allow(unused)]
pub fn random_multi_swap(
    mut children: Vec<Chromosome>,
    mutation_propability: f32,
    db: &Database,
) -> Vec<Chromosome> {
    // Get chromosome length
    let chr_len = db.events().len();

    // // Randomness
    // let mut rng = rand::thread_rng();
    let index = Uniform::new(0, chr_len);
    let probabilty = Uniform::new_inclusive(0., 1.);

    // // Iterate and mutate
    // for child in children.iter_mut() {
    //     if probabilty.sample(&mut rng) > mutation_propability {
    //         continue;
    //     }

    //     let i0 = index.sample(&mut rng);
    //     let i1 = index.sample(&mut rng);

    //     let val0 = child.0[i0];
    //     let val1 = child.0[i1];

    //     child.0[i0] = val1;
    //     child.0[i1] = val0;
    // }

    // // Return
    // children


    children
        .into_par_iter()
        .map(|mut c| {
            let mut rng = rand::thread_rng();
            // let probabilty = Uniform::new_inclusive(0., 1.);
            if probabilty.sample(&mut rng) > mutation_propability {
                return c;
            }

            for _ in 0..20 {
                // let index = Uniform::new(0, chr_len);
                let i0 = index.sample(&mut rng);
                let i1 = index.sample(&mut rng);

                let val0 = c.0[i0];
                let val1 = c.0[i1];

                c.0[i0] = val1;
                c.0[i1] = val0;
            }

            c
        })
        .collect()
}


#[allow(unused)]
pub fn inversion(
    children: Vec<Chromosome>,
    mutation_propability: f32,
    // db: &Database,
) -> Vec<Chromosome> {
    // Randomness


    children
        .into_par_iter()
        .map(|mut c| {
            let mut rng = rand::thread_rng();
            let probabilty = Uniform::new_inclusive(0., 1.);

            if probabilty.sample(&mut rng) > mutation_propability {
                return c;
            }

            c.0.reverse();
            c
        })
        .collect()

    // // Iterate and mutate
    // for child in children.iter_mut() {
    //     if probabilty.sample(&mut rng) > mutation_propability {
    //         continue;
    //     }

    //     child.0.reverse();
    // }

    // // Return
    // children
}