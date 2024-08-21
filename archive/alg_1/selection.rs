// Imports /////////////////////////////////////////////////////////////////////
use crate::{fitness::Cost, population::Chromosome};
use rand::prelude::Distribution;

// Functions ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub fn roulette_wheel(
    pair_count: usize,
    current_generation: &[(Chromosome, Cost)],
) -> Vec<((Chromosome, Cost), (Chromosome, Cost))> {
    // Calculate total cost
    let total_cost: usize = current_generation.iter().map(|(_, c)| c.0).sum();

    // Create the roulette wheel...
    let mut roulette_wheel =
        Vec::<f32>::with_capacity(current_generation.len());

    // ...by first calculating the cost relative to the populations total cost.
    for chromosome in current_generation {
        roulette_wheel.push(chromosome.1 .0 as f32 / total_cost as f32);
    }

    // ...and then accumulate the values
    let mut acc: f32 = 0.;
    for section in roulette_wheel.iter_mut() {
        *section += acc;
        acc = *section;
    }

    // ...additionally the last one must be set equal to 1 (due to rounding
    // errors).
    let last = roulette_wheel.last_mut().unwrap();
    *last = 1.;

    // Now selection can begin
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    let mut parents = vec![];
    for _ in 0..pair_count * 2 {
        // Random value (roulette wheel value)
        let value = interval.sample(&mut rng);

        for (i, section) in roulette_wheel.iter().enumerate() {
            if &value < section {
                parents.push(current_generation[i].clone());
                break;
            }
        }
    }

    // Create pairs from parents
    let pairs = parents
        .chunks(2)
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
        .collect::<Vec<((Chromosome, Cost), (Chromosome, Cost))>>();

    // Return
    pairs
}

#[allow(unused)]
pub fn rank(
    pair_count: usize,
    current_generation: &[(Chromosome, Cost)],
) -> Vec<((Chromosome, Cost), (Chromosome, Cost))> {
    // Get population size
    let pop_size = current_generation.len();

    // Define probability function (linear ranking)
    let probability = |rank: usize, pop_size: usize| -> f32 {
        let p = pop_size as f32;
        let r = rank as f32;
        (2. * (p - r)) / (p * (p + 1.))
    };

    // Create the rank-weighted "roulette_wheel
    let mut selector = Vec::<f32>::with_capacity(current_generation.len());
    for (i, _chromosome) in current_generation.iter().enumerate() {
        selector.push(probability(i, pop_size));
    }
    let last = selector.last_mut().unwrap();
    *last = 1.;

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
        .chunks(2)
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
        .collect::<Vec<((Chromosome, Cost), (Chromosome, Cost))>>();

    // Return
    pairs
}

////////////////////////////////////////////////////////////////////////////////
