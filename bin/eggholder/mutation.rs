use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::chromosome::Chromosome;

#[allow(unused)]
pub enum Mutation {
    RandomUniform,
    RandomGauss,
}

impl Mutation {
    pub fn exec(&self, c: &mut Chromosome, rate: f32, std_dev: f32) {
        match self {
            Mutation::RandomUniform => {
                let mut rng = rand::thread_rng();
                let uniform_dist = Uniform::new_inclusive(-512., 512.);

                // x0
                if rng.gen::<f32>() <= rate {
                    let new_x0 = uniform_dist.sample(&mut rng);
                    c.set_x0(new_x0);
                }

                // x1
                if rng.gen::<f32>() <= rate {
                    let new_x1 = uniform_dist.sample(&mut rng);
                    c.set_x1(new_x1);
                }
            }

            Mutation::RandomGauss => {
                let mut rng = rand::thread_rng();
                let normal_dist = rand_distr::Normal::new(0., std_dev).unwrap();

                // x0
                let mut offset = normal_dist.sample(&mut rng);
                let mut x0_new = c.x0() + offset as f64;

                #[allow(clippy::manual_range_contains)]
                while x0_new < -512. || x0_new > 512. {
                    offset = normal_dist.sample(&mut rng);
                    x0_new = c.x0() + offset as f64;
                }
                c.set_x0(x0_new);

                // x1
                let mut offset = normal_dist.sample(&mut rng);
                let mut x1_new = c.x1() + offset as f64;

                #[allow(clippy::manual_range_contains)]
                while x1_new < -512. || x1_new > 512. {
                    offset = normal_dist.sample(&mut rng);
                    x1_new = c.x1() + offset as f64;
                }
                c.set_x1(x1_new);
            }
        }
    }
}
