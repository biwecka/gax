// Imports /////////////////////////////////////////////////////////////////////
use super::Context;
use hashbrown::HashMap;
use ndarray::Array2;
use rand_distr::Distribution;
use std::ops::AddAssign;

// Genotype ////////////////////////////////////////////////////////////////////
/// This chromosome encodes the event-to-time allocation like described in
/// Abramsons paper. The outer vector represents time slots, and is therefore
/// as long as there are time slots available (e.g. 30 time slots per week).
/// The inner vectors contain the events (event indices), which are allocated
/// to the time slot represented by the outer vector they are contained in.
///
/// The outer vector is of fixed length.
/// The inner vectors have different lengths and can vary in their length.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(pub Vec<Vec<usize>>);

impl From<Vec<Vec<usize>>> for Chromosome {
    fn from(value: Vec<Vec<usize>>) -> Self {
        Self(value)
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        // Get source of randomness
        let rng = rand::thread_rng();

        // Initialize the vector of chromosomes (result).
        let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

        // Calculate the average events per timeslot
        let ept =
            (ctx.num_events as f32 / ctx.num_times as f32).round() as usize;

        for _ in 0..amount {
            // Create the inner value of the chromosome
            let mut time_slots: Vec<Vec<usize>> =
                Vec::with_capacity(ctx.num_times);

            for _ in 0..ctx.num_times {
                time_slots.push(
                    ctx.rand_event.sample_iter(rng.clone()).take(ept).collect(),
                );
            }

            chromosomes.push(time_slots.into());
        }

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

pub struct IntermediateRepresentation(pub Array2<u8>);

impl IntermediateRepresentation {
    pub fn from_chromosome(chromosome: &Chromosome, ctx: &Context) -> Self {
        let mut matrix = Array2::<u8>::default((ctx.num_times, ctx.num_events));

        for (time_idx, events) in chromosome.0.iter().enumerate() {
            for event_idx in events {
                matrix[[time_idx, *event_idx]] = 1;
            }
        }

        Self(matrix)
    }

    /// Get the time allocation of an event. This function returns two values
    /// as a tuple:
    /// 1) amount of time slots allocated to the event
    /// 2) boolean value: true, if the time allocation is coherent; false, if
    ///    the time allocation is NOT coherent.
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

////////////////////////////////////////////////////////////////////////////////
