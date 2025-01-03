// Imports /////////////////////////////////////////////////////////////////////
use bitvec::prelude::*;
use rand::seq::SliceRandom;
use rand_distr::Distribution;
use std::collections::HashSet;

// Main ////////////////////////////////////////////////////////////////////////

// This main method contains an implementation for trading one time allocation
// for another between two event genes.
//
// This can be used as crossover operator `Crossover::Trade(n)`.
fn main() {
    let y_e_0 = bitvec![u32, Lsb0; 0, 0, 1, 1, 1, 0, 1, 1, 0, 1];
    let y_e_1 = bitvec![u32, Lsb0; 0, 1, 1, 0, 1, 1, 0, 0, 1, 0];

    // Negate both
    let y_e_0_inv = !y_e_0.clone();
    let y_e_1_inv = !y_e_1.clone();

    // Calc possible trades from e_0 -> e_1
    let trades_0_to_1 = y_e_0 & y_e_1_inv;
    println!(
        "0 -> 1: {} / {:?}",
        trades_0_to_1,
        trades_0_to_1.iter_ones().collect::<Vec<_>>()
    );

    // Calc possible trades from e_1 -> e_0
    let trades_1_to_0 = y_e_1 & y_e_0_inv;
    println!(
        "1 -> 0: {} / {:?}",
        trades_1_to_0,
        trades_1_to_0.iter_ones().collect::<Vec<_>>()
    );
}

// This main method contains an implementation for moving a single time
// allocation to another random position.
//
// This operatoin can be used as `Mutation::MoveSingleTimeAlloc`.
#[allow(unused)]
fn mainx() {
    let eg = EventGene::from_sub_events(vec![
        (1, bitvec![u32, Lsb0; 0, 1, 0, 0, 0, 0, 0]),
        (2, bitvec![u32, Lsb0; 0, 0, 1, 0, 0, 1, 0]),
    ])
    .unwrap();

    println!("{}", eg);

    // Define time slot move (original and target index). THESE CAN BE
    // GENERATED RANDOMLY. The `mv_from` should be set, and `mv_to` position
    // should be unset.
    let mv_from = 3;
    let mv_to = 4;

    // Get teh Y_e vector and manipulate it
    let mut y_e = eg.times.clone();
    y_e.set(mv_from, false);
    y_e.set(mv_to, true);

    println!("Y_e = {}", y_e);

    // Create the new event gene
    let eg_new = EventGene::from_time_allocation(y_e).unwrap();
    println!("{}", eg_new);
}

// This main method contains an implementation for moving a random sub_event
// to another random position. The algorithm is designed in a way, that it
// always results in a VALID event gene!
//
// This operation can be used as `Mutation::MoveSubEvent`
#[allow(unused)]
fn mainx2() {
    let eg = EventGene::from_sub_events(vec![
        (1, bitvec![u32, Lsb0; 0, 1, 0, 0, 0, 0, 0]),
        (2, bitvec![u32, Lsb0; 0, 0, 1, 0, 0, 1, 0]),
    ])
    .unwrap();

    println!("{}", eg);

    // Define which sub_event to move. THESE CAN BE GENERATED RANDOMLY.
    // But when generating those indices, it's important to only use `mv_i`
    // where the K_e,d=2 vector is actually set to 1 !!!
    let mv_d = 2;
    let mv_i = 5;

    // Remove sub_event from Y_e
    let mut y_e = eg.times.clone();
    for i in mv_i..(mv_i + mv_d) {
        y_e.set(i, false);
    }
    println!("Y_e = {}", y_e);

    // Calculate helper vector. This vector has bit `i` set, if the input
    // vector has its bits UNSET from `i` until `i+d-1`.
    let h = {
        let len = y_e.len();
        let mut h = bitvec![u32, Lsb0; 0; len];

        for index in 0..len {
            if index + mv_d > len {
                break;
            };
            let range = index..(index + mv_d);
            let slice = &y_e[range];

            let only_ones = slice.count_zeros() == slice.len();
            if only_ones {
                h.set(index, true);
            }
        }

        // Return
        h
    };

    println!("h   = {}", h);

    // Randomly choose an index from the helper vector
    let mut indices = h.iter_ones().collect::<Vec<_>>();
    indices.shuffle(&mut rand::thread_rng());

    if let Some(i) = indices.first() {
        let mut x = eg.sub_event_start_times.clone();
        let y = x.iter_mut().find(|(d, _)| *d == mv_d).unwrap();
        y.1.set(mv_i, false);
        y.1.set(*i, true);

        let eg_new = EventGene::from_sub_events(x).unwrap();
        println!("{}", eg_new);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventGene {
    // Bitvector to store all time assignments of this event's sub-events.
    pub times: BitVec<u32, Lsb0>,

    // Bitvector that only stores the starting times of the event's sub-events.
    start_times: BitVec<u32, Lsb0>,

    // List of (d, bitvector), where d represents a duration and the bitvector
    // stores only the starting times of sub-events with a duration of d.
    pub sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)>,
}

impl std::fmt::Display for EventGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EventGene {{")?;

        writeln!(f, "  Y_e   : {}", self.times)?;
        writeln!(f, "  S_e   : {}", self.start_times)?;
        writeln!(f, "  K_e,d : [")?;
        for sub in &self.sub_event_start_times {
            writeln!(f, "    {:>2} => {}", sub.0, sub.1)?;
        }

        writeln!(f, "  ]")?;
        write!(f, "}}")
    }
}

impl EventGene {
    pub fn generate(duration: u8, num_times: usize) -> Self {
        let mut times = bitvec![u32, Lsb0; 0; num_times];
        let mut start_times = bitvec![u32, Lsb0; 0; num_times];
        let mut sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)> = vec![];

        // Generate random times. These values represent indices, at which the
        // `times` bitvector will be set to 1.
        let mut rng = rand::thread_rng();
        let rand_time = rand::distributions::Uniform::new(0, num_times);

        let mut time_indices = vec![];
        while time_indices.len() < duration as usize {
            let i = rand_time.sample(&mut rng);
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
            let mut bv = bitvec![u32, Lsb0; 0; num_times];
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

    #[allow(clippy::result_unit_err)]
    pub fn from_time_allocation(times: BitVec<u32, Lsb0>) -> Result<Self, ()> {
        // Extract the number of time slots
        let num_times = times.len();

        // Declare variables
        let mut start_times = bitvec![u32, Lsb0; 0; num_times];
        let mut sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)> = vec![];

        // >>> Calculate sub-event start time bitvectors <<<

        // Group the time indices into groups of consecutive numbers.
        let grouped_time_indices = group_consecutive_numbers(
            times.iter_ones().collect::<Vec<usize>>(),
        );

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
            let mut bv = bitvec![u32, Lsb0; 0; num_times];
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
        Ok(e)
    }

    #[allow(clippy::result_unit_err)]
    pub fn from_sub_events(
        sub_events: Vec<(usize, BitVec<u32, Lsb0>)>,
    ) -> Result<Self, ()> {
        // Extract num_times
        let num_times = match sub_events.first() {
            Some(x) => x.1.len(),
            None => return Err(()),
        };

        // Make sure, all given values are of length `num_times`
        for val in &sub_events {
            if val.1.len() != num_times {
                return Err(());
            }
        }

        // Declare variables
        let mut times = bitvec![u32, Lsb0; 0; num_times];
        let mut start_times = bitvec![u32, Lsb0; 0; num_times];
        let mut sub_event_start_times: Vec<(usize, BitVec<u32, Lsb0>)> = vec![];

        // >>> Copy given values as sub-event start times <<<
        for val in sub_events {
            sub_event_start_times.push(val.clone());
        }

        // >>> Calculate start_times bitvector <<<
        for (_, bv) in sub_event_start_times.iter() {
            start_times |= bv;
        }

        // >>> Calculate times bitvector <<<
        for (d, k_ed) in sub_event_start_times.iter() {
            for i in k_ed.iter_ones() {
                for k in i..(i + d) {
                    times.set(k, true);
                }
            }
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
        Ok(e)
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
    /// (-> K_e,d\[i\] = 1), make sure that no other starting time can be set
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
