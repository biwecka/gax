// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashMap;
use rand::seq::SliceRandom;
use std::ops::AddAssign;

use super::Context;

// Genotype ////////////////////////////////////////////////////////////////////
/// The chromosome stores a vector of event indices, where the order of these
/// indices in the chromosome defines the order in which the corresponding
/// events are scheduled. The values of the vector fields are event indices.
/// Therefore, the encoding is a "permutation encoding".
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(pub Vec<u8>);

impl From<Vec<u8>> for Chromosome {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl Chromosome {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }

    #[allow(unused)]
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }

    #[allow(unused)]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl From<Vec<&u8>> for Chromosome {
    fn from(value: Vec<&u8>) -> Self {
        Self(value.into_iter().cloned().collect())
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

        for _ in 0..amount {
            let mut chromosome = (0..ctx.num_events as u8).collect::<Vec<u8>>();
            chromosome.shuffle(&mut rng);

            chromosomes.push(chromosome.into());
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
