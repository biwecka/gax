// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Moves a sub event to new, random (fitting) starting time
    MoveSubEvent,

    /// Move single time allocation
    MoveSingleTimeAlloc,

    None,
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(
        &self,
        c: &mut Chromosome,
        rate: f32,
        rng: &mut ThreadRng,
        ctx: &Context,
    ) {
        match self {
            Self::MoveSubEvent => move_sub_event(c, rate, rng, ctx),
            Self::MoveSingleTimeAlloc => {
                move_single_time_alloc(c, rate, rng, ctx)
            }
            Self::None => {}
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
fn move_sub_event(
    c: &mut Chromosome,
    rate: f32,
    rng: &mut ThreadRng,
    _ctx: &Context,
) {
    // Iterate over all genes
    for bits in c.0.iter_mut() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Select random sub event (duration)
        let (d, k_ed) = bits
            .blocks() // Calculate sub events
            .into_iter()
            .enumerate()
            .filter(|(_, b)| !b.is_zero()) // Remove durations with no events
            .choose(rng) // Randomly choose a duration
            .unwrap();

        // Select random event (index)
        let i = k_ed.ones().choose(rng).unwrap();

        // Remove the sub event from the time allocation
        bits.unset_block(i, d as u32);

        // Get all starting indices of `0` blocks of length `d`.
        let new_index = bits.holes(d as u32).choose(rng).unwrap();

        // Unset the old index, and set the new index
        bits.unset(i);
        bits.set_block(new_index, d as u32);
    }
}

fn move_single_time_alloc(
    c: &mut Chromosome,
    rate: f32,
    rng: &mut ThreadRng,
    _ctx: &Context,
) {
    // Iterate over all genes (events)
    for bits in c.0.iter_mut() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Get random time allocation
        let alloc = bits.ones().choose(rng).unwrap();

        // Get random free time
        let free = bits.zeros().choose(rng).unwrap();

        bits.unset(alloc);
        bits.set(free);
    }
}

////////////////////////////////////////////////////////////////////////////////
