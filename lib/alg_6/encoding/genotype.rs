// Imports /////////////////////////////////////////////////////////////////////
use super::context::Context;
use hashbrown::HashMap;
use rand::prelude::Distribution;
use std::ops::AddAssign;

// Genotype ////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(Vec<usize>);

impl Chromosome {
    pub fn as_slice(&self) -> &[usize] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        self.0.as_mut_slice()
    }

    #[allow(unused)]
    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }

    // pub fn clone_inner(&self) -> Vec<usize> {
    //     self.0.clone()
    // }
}

impl From<Vec<&usize>> for Chromosome {
    fn from(value: Vec<&usize>) -> Self {
        Self(value.into_iter().cloned().collect())
    }
}

impl From<Vec<usize>> for Chromosome {
    fn from(value: Vec<usize>) -> Self {
        Self(value)
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

        for _ in 0..amount {
            let mut chromosome = Vec::<usize>::with_capacity(ctx.num_events);
            for event_idx in 0..ctx.num_events {
                // Get the random value generator for this gene
                let rand_time = ctx.rand_time[event_idx];

                // Generate random time and add it to the chromosome
                chromosome.push(rand_time.sample(&mut rng));
            }

            chromosomes.push(Self(chromosome))
        }

        // Return
        chromosomes
    }

    fn calc_diversity<Ov: ga::encoding::ObjectiveValue>(
        population: &[(Self, Ov)],
    ) -> Vec<usize> {
        let mut map = HashMap::<(Self, Ov), usize>::new();
        for i in population {
            map.entry(i.clone()).or_default().add_assign(1);
        }

        let mut arr: Vec<((Self, Ov), usize)> = map.into_iter().collect();
        arr.sort_by_key(|((_, x), _)| x.clone());

        // Return
        arr.into_iter().map(|(_, x)| x).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////
