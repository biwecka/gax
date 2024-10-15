// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context, EventGene};
use bitvec::prelude::*;
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
    fn identifier(&self) -> String {
        match self {
            Self::MoveSubEvent => "mv-sub".into(),
            Self::MoveSingleTimeAlloc => "mv-time".into(),
            Self::None => "none".into(),
        }
    }

    fn exec(
        &self,
        chromosome: &mut Chromosome,
        rate: f32,
        rng: &mut ThreadRng,
        _ctx: &Context,
    ) {
        match self {
            Mutation::None => {}

            Mutation::MoveSubEvent => {
                // Iterate over all genes (events)
                for e in &mut chromosome.0 {
                    // Decide wether to mutate or not
                    if rng.gen::<f32>() > rate {
                        continue;
                    }

                    // Select random sub event
                    let (d, k_ed) =
                        e.sub_event_start_times.iter_mut().choose(rng).unwrap();

                    // Index of the sub event.
                    let i = k_ed.iter_ones().choose(rng).unwrap();

                    // Clone the time allocation vector
                    let mut y_e = e.times.clone();

                    // Remove the sub event from the time allocation vector
                    for k in i..(i + *d) {
                        y_e.set(k, false);
                    }

                    // Calculate helper vector: This vector has bit `i` set,
                    // if the input vector has its bits UNSET from `i` until
                    // `i + (d-1)`.
                    let h = {
                        let len = y_e.len();
                        let mut h = bitvec![u32, Lsb0; 0; len];

                        for index in 0..len {
                            if index + *d > len {
                                break;
                            };
                            let range = index..(index + *d);
                            let slice = &y_e[range];

                            let only_zeros = slice.count_ones() == 0;
                            if only_zeros {
                                h.set(index, true);
                            }
                        }

                        // Return
                        h
                    };

                    // Randomly choose an index from the helper vector
                    let new_index = h.iter_ones().choose(rng).unwrap();

                    // Unset the old index, and set the new index
                    k_ed.set(i, false);
                    k_ed.set(new_index, true);

                    // Calculate new event gene
                    let eg = EventGene::from_sub_events(
                        e.sub_event_start_times.clone(),
                    );

                    // Apply new event gene properties to mutable event gene
                    // reference
                    e.apply(eg);
                }
            }

            Mutation::MoveSingleTimeAlloc => {
                // Iterate over all genes (events)
                for e in &mut chromosome.0 {
                    // Decide wether to mutate or not
                    if rng.gen::<f32>() > rate {
                        continue;
                    }

                    // Get random time allocation
                    let alloc = e.times.iter_ones().choose(rng).unwrap();

                    // Get random free time
                    let free = e.times.iter_zeros().choose(rng).unwrap();

                    e.times.set(alloc, false);
                    e.times.set(free, true);

                    // Calculate new event gene
                    let eg = EventGene::from_time_allocation(e.times.clone());

                    // Apply new event gene properties to mutable event gene
                    // reference
                    e.apply(eg);
                }
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
