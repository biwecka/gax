// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use rand_distr::Distribution;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Moves a sub event to new, random (fitting) starting time
    MoveSubEvent,

    /// Move single time allocation
    MoveSingleTimeAlloc,

    /// Trade single time allocation
    Trade,

    /// No mutation.
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
            Self::Trade => trade(c, rate, rng, ctx),
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

fn trade(c: &mut Chromosome, rate: f32, rng: &mut ThreadRng, ctx: &Context) {
    // Iterate over all genes (events)
    for i0 in 0..c.0.len() {
        // Decide wether to mutate or not
        if rng.gen::<f32>() > rate {
            continue;
        }

        // Randomly choose a trade partner (index)
        let i1 = ctx.rand_event.sample(rng);

        // Copy the time allocations of the indexed bits
        let mut b0 = c.0[i0];
        let mut b1 = c.0[i1];

        // >>> TRADE CALCULATIONS <<<
        // Negate both
        let b0_inv = !b0;
        let b1_inv = !b1;

        // Calc possible trades from 0 to 1
        let trade_0_to_1 = b0 & b1_inv;

        // Calc possible trades from 1 to 0
        let trade_1_to_0 = b1 & b0_inv;

        // Get trade indices
        let ti_0_to_1 = match trade_0_to_1.ones().choose(rng) {
            Some(i) => i,
            None => continue,
        };

        let ti_1_to_0 = match trade_1_to_0.ones().choose(rng) {
            Some(i) => i,
            None => continue,
        };

        // Perform trade from 0 to 1
        b0.unset(ti_0_to_1);
        b1.set(ti_0_to_1);

        // Perform trade from 1 to 0
        b1.unset(ti_1_to_0);
        b0.set(ti_1_to_0);

        // >>> TRADE - END <<<

        // Apply the changed bits to the chromosome again
        c.0[i0] = b0;
        c.0[i1] = b1;
    }
}

////////////////////////////////////////////////////////////////////////////////
