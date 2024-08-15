// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::chromosome::Chromosome;
use rand::prelude::Distribution;
use rayon::prelude::*;

// Functions ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub fn roulette_wheel(
    pair_count: usize,
    current_generation: &Vec<(Chromosome, usize)>,
) -> Vec<((Chromosome, usize), (Chromosome, usize))> {
    // Extract cost
    let costs: Vec<usize> =
        current_generation.iter().map(|(_, c)| *c).collect();

    // Calculate max
    let max_cost: usize = *costs.iter().max().unwrap();

    // Invert
    let inverted_costs: Vec<usize> =
        costs.iter().map(|x| max_cost - x).collect();

    // Calc total cost and highest cost
    let total_cost: usize = inverted_costs.iter().map(|x| x).sum();

    // Calculate proportion
    let proportions: Vec<f32> =
        inverted_costs.iter().map(|x| *x as f32 / total_cost as f32).collect();

    // Accumulate
    let mut acc: f32 = 0.;
    let mut roulette_wheel = vec![];
    for item in proportions {
        let value = item + acc;
        roulette_wheel.push(value);

        acc = value;
    }

    let last = roulette_wheel.last_mut().unwrap();
    *last = 1.;

    // Selection
    let mut selection: Vec<&(Chromosome, usize)> = vec![];
    // let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    for _ in 0..(pair_count * 2) {
        // Random value
        let value = interval.sample(&mut rng);

        for (i, section) in roulette_wheel.iter().enumerate() {
            if &value <= section {
                selection.push(&current_generation[i]);
                // selected_indices.insert(i);
                break;
            }
        }
    }

    // Return
    // (selection, selected_indices.len())
    selection
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect::<Vec<_>>()
}

// #[allow(unused)]
// pub fn roulette_wheel(
//     pair_count: usize,
//     current_generation: &Vec<(Chromosome, usize)>,
// ) -> Vec<((Chromosome, usize), (Chromosome, usize))> {
//     // Calculate total cost
//     let total_cost: usize = current_generation.iter().map(|(_, c)| c).sum();

//     // Create the roulette wheel...
//     let mut roulette_wheel =
//         Vec::<f32>::with_capacity(current_generation.len());

//     // ...by first calculating the cost relative to the populations total cost.
//     for chromosome in current_generation {
//         roulette_wheel.push((chromosome.1 as f32 / total_cost as f32));
//     }

//     // ...and then accumulate the values
//     let mut acc: f32 = 0.;
//     for section in roulette_wheel.iter_mut() {
//         *section = *section + acc;
//         acc = *section;
//     }

//     // ...additionally the last one must be set equal to 1 (due to rounding
//     // errors).
//     let last = roulette_wheel.last_mut().unwrap();
//     *last = 1.;

//     // Now selection can begin
//     let mut rng = rand::thread_rng();
//     let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

//     let mut parents = vec![];
//     for _ in 0..pair_count * 2 {
//         // Random value (roulette wheel value)
//         let value = interval.sample(&mut rng);

//         for (i, section) in roulette_wheel.iter().enumerate() {
//             if &value < section {
//                 parents.push(current_generation[i].clone());
//                 break;
//             }
//         }
//     }

//     // Create pairs from parents
//     let pairs = parents
//         // .chunks(2)
//         .par_chunks(2)
//         .filter_map(|chunk| {
//             if chunk.len() == 2 {
//                 Some((
//                     (chunk[0].0.clone(), chunk[0].1.clone()),
//                     (chunk[1].0.clone(), chunk[1].1.clone()),
//                 ))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<((Chromosome, usize), (Chromosome, usize))>>();

//     // Return
//     pairs
// }

#[allow(unused)]
pub fn rank(
    pair_count: usize,
    current_generation: &Vec<(Chromosome, usize)>,
) -> Vec<((Chromosome, usize), (Chromosome, usize))> {
    // Get population size
    let pop_size = current_generation.len();

    // Define probability function (linear ranking)
    let probability = |rank: usize, pop_size: usize| -> f32 {
        let p = pop_size as f32;
        let r = rank as f32;
        (2. * (p - r + 1.)) / (p * (p + 1.))
        // 0.01 * ((1. - 0.01) as f32).powi(rank as i32 - 1)
    };

    // Create the rank-weighted "roulette_wheel
    let mut selector = Vec::<f32>::with_capacity(current_generation.len());
    for (i, _chromosome) in current_generation.iter().enumerate() {
        selector.push(probability(i, pop_size));
    }

    // Now selection can begin
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    let mut parents = vec![];
    for _ in 0..pair_count * 2 {
        // Random value
        let value = interval.sample(&mut rng);

        for (i, selection) in selector.iter().enumerate() {
            if &value < selection {
                parents.push(current_generation[i].clone());
                break;
            }
        }
    }

    // Create pairs from parents
    let pairs = parents
        // .chunks(2)
        .par_chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((
                    (chunk[0].0.clone(), chunk[0].1.clone()),
                    (chunk[1].0.clone(), chunk[1].1.clone()),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<((Chromosome, usize), (Chromosome, usize))>>();

    // Return
    pairs
}
