// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    VariableSinglePoint,
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn identifier(&self) -> String {
        match self {
            Self::VariableSinglePoint => "var-s-pt".into(),
        }
    }

    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        ctx: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint => {
                // Decide whether the crossover should be performed or not.
                if let Some(rate) = rate {
                    if rng.gen::<f32>() > rate {
                        // No crossover, simply return the parents
                        return (parent_0.clone(), parent_1.clone());
                    }
                }

                // Create children vectors
                let mut c0: Vec<Vec<usize>> = Vec::with_capacity(ctx.num_times);
                let mut c1: Vec<Vec<usize>> = Vec::with_capacity(ctx.num_times);

                // Iterate over the timeslots of each parent
                for (a, b) in
                    parent_0.0.clone().into_iter().zip(parent_1.0.clone())
                {
                    // Get the shorter length of the two arrays
                    let max_split = a.len().min(b.len());

                    if max_split < 1 {
                        c0.push(a);
                        c1.push(b);
                        continue;
                    }

                    // Randomly generate a split index
                    let index = rng.gen_range(0..max_split);

                    // Split the parents
                    let (a_head, a_tail) = a.split_at(index);
                    let (b_head, b_tail) = b.split_at(index);

                    // Create childrens time slot vectors
                    let mut c0_ts = [a_head, b_tail].concat();
                    let mut c1_ts = [b_head, a_tail].concat();

                    // Remove duplicates
                    super::remove_duplicates(&mut c0_ts);
                    super::remove_duplicates(&mut c1_ts);

                    // Add time slot vectors to the children
                    c0.push(c0_ts);
                    c1.push(c1_ts);
                }

                (c0.into(), c1.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
