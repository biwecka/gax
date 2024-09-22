// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::rngs::ThreadRng;

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Uniform crossover
    Uniform,
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        _ctx: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::Uniform => {
                let (c0, c1) = ga::operators::crossover::uniform(
                    &parent_0.0,
                    &parent_1.0,
                    rate,
                    rng,
                );

                let x0 = c0.into_iter().map(|x| x.clone()).collect::<Vec<_>>();
                let x1 = c1.into_iter().map(|x| x.clone()).collect::<Vec<_>>();

                (x0.into(), x1.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
