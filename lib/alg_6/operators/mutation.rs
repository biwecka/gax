// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Randomize n Genes modifies n genes of the chromosome randomly.
    /// Parameters:
    /// 1) f32      representing the mutation rate
    /// 2) usize    representing the number of genes to modify
    RandomizeNGenes(f32, usize),
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(&self, chromosome: &mut Chromosome, context: &Context) {
        match self {
            Mutation::RandomizeNGenes(rate, n) => {
                ga::utils::mutation::randomize_n_genes(
                    *n,
                    chromosome.as_mut_slice(),
                    *rate,
                    context.rand_time,
                );
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
