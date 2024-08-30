// Imports /////////////////////////////////////////////////////////////////////
// use super::context::Context;
use hashbrown::HashMap;
use ndarray::Array2;
use rand_distr::Distribution;
use std::ops::AddAssign;

use super::Context;

// Genotype ////////////////////////////////////////////////////////////////////
/// This chromosome is a wrapper around a 2D-matrix which contains the
/// allocation of events to time slots.
///
/// The matrix might for example look like this:
///
///     event_0 event_1 event_2 event_3 event_4 ...  
/// t0    1       1  
/// t1            1       1       1  
/// t2    1                               1  
/// t3  
/// t4  
/// ...  
///
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Chromosome(pub Array2<u8>);

impl Chromosome {
    /// Get the time allocation of an event. This function returns two values
    /// as a tuple:
    /// 1) amount of time slots allocated to the event
    /// 2) boolean value: true, if the time allocation is coherent; false, if
    ///    the time allocation is NOT coherent.
    /// 3) boolean value: true, if the time allocation including the event's
    ///    duration overflow the maximum time slot index; false if not
    pub fn get_event_time_allocation(&self, event_idx: usize) -> (u8, bool) {
        let col = self.0.column(event_idx);

        let sum = col.sum();

        let coherent = 'x: {
            let mut prev = col.first().unwrap();
            let mut sequence_end = false;

            for val in col.iter().skip(1) {
                // If we already observed the end of a 1s-sequence, and are
                // currently observing the start of another 1s-sequence, we
                // return false
                if sequence_end && prev == &0 && val == &1 {
                    break 'x false;
                }

                // As soon as we detect the end of a 1s-sequence, we set the
                // corresponding flag to true.
                if prev == &1 && val == &0 {
                    sequence_end = true;
                }

                // Update "previous"
                prev = val;
            }

            true
        };

        (sum, coherent)
    }
}

impl From<Array2<u8>> for Chromosome {
    fn from(value: Array2<u8>) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");

        for row in self.0.outer_iter() {
            s += "[";
            for item in row.iter() {
                if *item > 0 {
                    s += &format!(" {}", item);
                } else {
                    s += " ·";
                }
            }
            s += " ]\n";
        }

        write!(f, "Chromosome:\n{}", s)
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        // Get source of randomness
        let mut rng = rand::thread_rng();

        // Initialize the vector of chromosomes (result).
        let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

        // Calculate the average events per timeslot
        let ept =
            (ctx.num_events as f32 / ctx.num_times as f32).round() as usize;

        for _ in 0..amount {
            // Create the matrix of the chromosome
            let mut matrix =
                Array2::<u8>::default((ctx.num_times, ctx.num_events));

            // Allocate `ept` amount of events per time (per row in the matrix)
            for mut row in matrix.rows_mut() {
                for _ in 0..ept {
                    // Generate random event index
                    let event_idx = ctx.rand_event.sample(&mut rng);
                    row[event_idx] = 1;
                }
            }

            chromosomes.push(Chromosome(matrix));
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
