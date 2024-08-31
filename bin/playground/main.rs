use rand_distr::Distribution;

fn main() {
    // Crossover implementation
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7];

    let index = 2;
    let (a_head, a_tail) = a.split_at(index);
    let (b_head, b_tail) = b.split_at(index);

    let x = vec![a_head, b_tail].concat();
    let y = vec![b_head, a_tail].concat();
    dbg!(x, y);

    // Sort and dedup removes all duplicate elements
    let mut test = vec![1, 2, 3, 3, 3, 3, 4, 5, 3, 6, 6, 7];
    test.sort();
    test.dedup(); //.partition_dedup also returns the duplicates

    dbg!(test);

    // Remove duplicates and maintain element order
    let mut test = vec![7, 6, 1, 2, 3, 3, 3, 3, 4, 5, 3, 6, 6, 7];
    remove_duplicates(&mut test);

    dbg!(test);

    // Test
    let uniform_dist = rand::distributions::Uniform::new(0, 10);
    let x: Vec<usize> =
        uniform_dist.sample_iter(rand::thread_rng()).take(10).collect();
    dbg!(x);

    // Final test
    let mut matrix = vec![vec![1, 5, 7, 9, 3], vec![5, 8, 9, 2, 1, 7, 8]];

    // Generate a list of events in this time slot which should be mutated
    // This list contains a tuple, where the first value is the event index
    // and the second value is the target time slot.
    let mut mutations: Vec<usize> = vec![];
    let mut target_index: Vec<usize> = vec![];

    let test_mutation_indices = vec![(1, 4, 0)];

    // for (time_idx, events) in matrix.iter_mut().enumerate() {
    for (time_idx, event_idx, target_time_idx) in test_mutation_indices {
        // for event_idx in events.clone() {
        // Decide wether to mutate or not

        // Generate a target time slot for the mutation
        // let mut swap_time = generator.sample(rng);
        // while swap_time == time_idx {
        //     swap_time = generator.sample(rng);
        // }

        // If so, add the event index and the index of this event in the
        // time allocation vector to the "list"
        mutations.push(matrix[time_idx][event_idx]);
        target_index.push(target_time_idx);
        // }

        // Remove the mutated events from this time slot
        matrix[time_idx].retain(|x| !mutations.contains(x));
    }

    // Add the mutated events to the target time slots
    for (i, target_index) in target_index.iter().enumerate() {
        matrix[*target_index].push(mutations[i]);

        // Remove duplicates
        remove_duplicates(&mut matrix[*target_index]);
    }

    dbg!(matrix);
}

fn remove_duplicates<T: Clone + std::hash::Hash + Eq>(vec: &mut Vec<T>) {
    let mut seen = std::collections::HashSet::new();
    vec.retain(|x| seen.insert(x.clone()));
}
