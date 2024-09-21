// Imports /////////////////////////////////////////////////////////////////////
use super::Context;
use bitvec::prelude::*;
use hashbrown::HashSet;
use rand_distr::Distribution;

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
pub struct Chromosome(pub Vec<BitVec<u32, Lsb0>>);

// impl From<Vec<Vec<usize>>> for Chromosome {
//     fn from(value: Vec<Vec<usize>>) -> Self {
//         Self(value)
//     }
// }

// impl ga::encoding::Genotype<Context> for Chromosome {
//     fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
//         // Get source of randomness
//         let rng = rand::thread_rng();

//         // Initialize the vector of chromosomes (result).
//         let mut chromosomes: Vec<Self> = Vec::with_capacity(amount);

//         // // Calculate the average events per timeslot
//         // let ept =
//         //     (ctx.num_events as f32 / ctx.num_times as f32).round() as usize;

//         // for _ in 0..amount {
//         //     // Create the inner value of the chromosome
//         //     let mut time_slots: Vec<Vec<usize>> =
//         //         Vec::with_capacity(ctx.num_times);

//         //     for _ in 0..ctx.num_times {
//         //         time_slots.push(
//         //             ctx.rand_event.sample_iter(rng.clone()).take(ept).collect(),
//         //         );
//         //     }

//         //     chromosomes.push(time_slots.into());
//         // }

//         // chromosomes
//     }

//     fn calc_diversity<Ov: ga::encoding::ObjectiveValue>(
//         population: &[(Self, Ov)],
//     ) -> Vec<usize> {
//         let mut map = HashMap::<(Self, Ov), usize>::new();
//         for i in population {
//             map.entry(i.clone()).or_default().add_assign(1);
//         }

//         let mut arr: Vec<((Self, Ov), usize)> = map.into_iter().collect();
//         arr.sort_by_key(|((_, x), _)| x.clone());

//         // Return
//         arr.into_iter().map(|(_, x)| x).collect()
//     }
// }

////////////////////////////////////////////////////////////////////////////////

pub struct EventGene {
    // Bitvector to store all time assignments of this event's sub-events.
    times: BitVec<u32, Lsb0>,

    // Bitvector that only stores the starting times of the event's sub-events.
    start_times: BitVec<u32, Lsb0>,

    // List of (d, bitvector), where d represents a duration and the bitvector
    // stores only the starting times of sub-events with a duration of d.
    sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)>,
}

impl EventGene {
    pub fn generate(duration: usize, ctx: &Context) -> Self {
        let mut times = bitvec![u32, Lsb0; 0; ctx.num_times];
        let mut start_times = bitvec![u32, Lsb0; 0; ctx.num_times];
        let mut sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)> = vec![];

        // Generate random times. These values represent indices, at which the
        // `times` bitvector will be set to 1.
        let mut rng = rand::thread_rng();

        let mut time_indices = vec![];
        while time_indices.len() < duration {
            let i = ctx.rand_time.sample(&mut rng);
            if !time_indices.contains(&i) {
                time_indices.push(i);
            }
        }

        // >>> Create times bitvector <<<
        for i in time_indices.iter() {
            times.set(*i, true);
        }

        // >>> Calculate sub-event start time bitvectors <<<

        // Group the time indices into groups of consecutive numbers.
        let grouped_time_indices = group_consecutive_numbers(time_indices);

        // Get a list of durations of the sub-events
        let duration_set = HashSet::<usize>::from_iter(
            grouped_time_indices.iter().map(|sub| sub.len()),
        );
        let mut durations: Vec<usize> = duration_set.into_iter().collect();

        durations.sort();

        for d in durations {
            // Get all sub-events of duration d
            let sub_events = grouped_time_indices
                .iter()
                .filter(|sub| sub.len() == d)
                .collect::<Vec<_>>();

            // Get the first index of each sub-event of duration d
            let indices =
                sub_events.iter().filter_map(|x| x.first()).collect::<Vec<_>>();

            // Create bitvec
            let mut bv = bitvec![u32, Lsb0; 0; ctx.num_times];
            for i in indices {
                bv.set(*i, true);
            }

            // Add to sub-event start times
            sub_event_start_times.push((d, bv));
        }

        // >>> Calculate start times bitvector <<<
        for (_, bv) in sub_event_start_times.iter() {
            start_times |= bv;
        }

        // Create event gene
        let e = Self { times, start_times, sub_event_start_times };

        // Check data correctness
        assert!(Self::check_rule_1(&e));
        assert!(Self::check_rule_2(&e));
        assert!(Self::check_rule_3(&e));
        assert!(Self::check_rule_4(&e));
        assert!(Self::check_rule_5(&e));
        assert!(Self::check_rule_6(&e));

        // Return
        e
    }

    /// Rule 1: Starting times must also be contained in time allocation.
    fn check_rule_1(e: &Self) -> bool {
        // S_e or Y_e  -> (starting_times | time_allocation)
        let x = e.start_times.clone() | e.times.clone();

        // Check
        x == e.times
    }

    /// Rule 2: Starting times must be a the actual start of a time assignment,
    /// meaning that the sub-event starts at `t` if `times[t] = 1` and
    /// `times[t-1] = 0`.
    fn check_rule_2(e: &Self) -> bool {
        let mut x = e.times.clone();
        x.shift_right(1); // >> 1
        x = !x; // not

        // and time allocation
        x &= e.times.clone();

        // or starting times
        x |= e.start_times.clone();

        // Check
        x == e.start_times
    }

    /// Rule 3: Combining all sub_event_start_times must yield start_times.
    fn check_rule_3(e: &Self) -> bool {
        let mut x = bitvec![u32, Lsb0; 0; e.times.len()];

        for (_, y) in &e.sub_event_start_times {
            x |= y.clone();
        }

        // Check
        x == e.start_times
    }

    /// Rule 4: Ensure that the sub-events only are of length d.
    fn check_rule_4(e: &Self) -> bool {
        for (d, mut bv) in e.sub_event_start_times.clone() {
            bv.shift_right(d);
            bv &= e.times.clone();
            bv |= e.start_times.clone();

            if bv != e.start_times {
                return false;
            }
        }

        true
    }

    /// Rule 5: Ensure that sub-events consist of d consecutive times.
    fn check_rule_5(e: &Self) -> bool {
        for (d, bv) in e.sub_event_start_times.clone() {
            // Copy original bv
            let mut x = bv.clone();

            // Calculate helper vector Y_e^d
            let h = {
                let len = e.times.len();
                let mut h = bitvec![u32, Lsb0; 0; len];

                for index in 0..len {
                    let range = index..(index + d).min(len);
                    let slice = &e.times[range];

                    let only_ones = slice.count_ones() == slice.len();
                    if only_ones {
                        h.set(index, true);
                    }
                }

                // Return
                h
            };

            x &= h;
            x |= bv.clone();

            if x != bv {
                return false;
            }
        }

        // Return
        true
    }

    /// Rule 6: If an event has a sub-event of duration d starting at i
    /// (-> K_e,d[i] = 1), make sure that no other starting time can be set
    /// within the duration of that sub-event.
    fn check_rule_6(e: &Self) -> bool {
        // Function to calculate the helper vector.
        let helper_vector = |k: usize, bv: BitVec<u32, Lsb0>| {
            // Important: The "first iteration" contains a `<< 0` and `not`
            // operation. `<< 0` is a noop, therefore only `not` is applied in
            // form of the `!` operator.
            let mut result = !bv.clone();

            for i in 1..=k {
                let mut tmp = bv.clone();

                // TODO: check order of the two operations below
                // tmp = !tmp;
                tmp.shift_left(i);
                tmp = !tmp;

                result &= tmp;
            }

            result
        };

        for (d1, bv_d1) in e.sub_event_start_times.clone() {
            // Calculate helper vector K_e,d^k
            let durations = e
                .sub_event_start_times
                .iter()
                .map(|(d, _)| *d)
                .filter(|x| *x != d1)
                .collect::<Vec<_>>();

            let first_d2 = match durations.first() {
                Some(x) => *x,
                None => continue,
            };

            // Get vector for this duration
            let bv_d2 = e
                .sub_event_start_times
                .iter()
                .find(|(d, _)| *d == first_d2)
                .unwrap()
                .1
                .clone();

            let mut tmp = helper_vector(d1 - 1, bv_d2);

            for d2 in durations.iter().skip(1) {
                // Get vector for this duration
                let bv_d2 = e
                    .sub_event_start_times
                    .iter()
                    .find(|(d, _)| d == d2)
                    .unwrap()
                    .1
                    .clone();

                // Calculate helper vector
                let h = helper_vector(d1 - 1, bv_d2);

                tmp &= h;
            }

            let mut x = bv_d1.clone();
            x &= tmp;

            if x != bv_d1 {
                return false;
            }
        }

        // Return
        true
    }
}

// Helpers /////////////////////////////////////////////////////////////////////
fn group_consecutive_numbers(mut numbers: Vec<usize>) -> Vec<Vec<usize>> {
    // Sort the input data and extract the first number
    numbers.sort();

    // Check if input data is empty. If so, return empty result. If not,
    // extract the first element
    let first = match numbers.first() {
        Some(x) => *x,
        None => return vec![],
    };

    // Define variables
    let mut groups: Vec<Vec<usize>> = vec![];

    let mut curr_group: Vec<usize> = vec![first];
    let mut last_num: usize = first;

    // Iterate numbers but skip the first one as this has been extracted already
    for n in numbers.into_iter().skip(1) {
        // If the last number and the current number are consecutive, then add
        // the current number to the current group and update the last number
        // helper variable.
        if last_num + 1 == n {
            curr_group.push(n);
            last_num = n;
        }
        // If the numbers are not consecutive, "close" the current group, add
        // it to the result and start a new group.
        else if !curr_group.is_empty() {
            // Close current group
            groups.push(curr_group);

            // Create new group
            curr_group = vec![n];
            last_num = n;
        }
    }

    // Add current group if it's not empty
    if !curr_group.is_empty() {
        groups.push(curr_group);
    }

    // Return
    groups
}

////////////////////////////////////////////////////////////////////////////////
