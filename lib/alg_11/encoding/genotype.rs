// Imports /////////////////////////////////////////////////////////////////////
use std::ops::AddAssign;
use bits::Bits32;
use hashbrown::HashMap;
use rand_distr::Distribution;

use super::Context;

// Genotype ////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(pub Vec<Bits32>);

impl From<Vec<Bits32>> for Chromosome {
    fn from(value: Vec<Bits32>) -> Self {
        Self(value)
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

        for _ in 0..amount {
            let mut genes: Vec<Bits32> = Vec::with_capacity(ctx.num_events);
            for event_idx in 0..ctx.num_events {
                genes.push(Bits32::generate(ctx.durations[event_idx], ctx))
            }

            chromosomes.push(genes.into());
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

trait Gene {
    fn generate(duration: u8, ctx: &Context) -> Self;
}

impl Gene for Bits32 {
    fn generate(duration: u8, ctx: &Context) -> Self {
        // Get source of randomness
        let mut rng = rand::thread_rng();

        // Generate `duration` amount of random indices in (0..num_times).
        let mut indices: Vec<u32> = Vec::with_capacity(duration as usize);
        for _ in 0..duration {
            indices.push(ctx.rand_time.sample(&mut rng));
        }

        // Create bits32
        let mut bits = Bits32::new(ctx.num_times as u32, 0);
        for i in indices {
            bits.set(i);
        }

        // Return
        bits
    }
}


////////////////////////////////////////////////////////////////////////////////
