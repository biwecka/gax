// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Cost, Genotype};
use hashbrown::HashSet;
use rand::prelude::Distribution;

// Enum ////////////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Selection {
    RouletteWheel,
    Tournament(usize),
    Random,
    LinearRank,
}

impl Selection {
    pub fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Genotype, Cost)],
    ) -> (Vec<&'a (Genotype, Cost)>, usize) {
        match self {
            Self::RouletteWheel => roulette_wheel(amount, individuals),
            Self::Tournament(n) => tournament(*n, amount, individuals),
            Self::Random => random(amount, individuals),
            Self::LinearRank => linear_rank(amount, individuals),
        }
    }
}

// Implementations /////////////////////////////////////////////////////////////
fn roulette_wheel<'a>(
    amount: usize,
    individuals: &'a [(Genotype, Cost)],
) -> (Vec<&'a (Genotype, Cost)>, usize) {
    // Extract cost
    let costs: Vec<usize> = individuals.iter().map(|(_, c)| *c).collect();

    // Calculate max
    let max_cost: usize = *costs.iter().max().unwrap();

    // Invert
    let inverted_costs: Vec<usize> =
        costs.iter().map(|x| (max_cost - x) * (max_cost - x)).collect();

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
    let mut selection: Vec<&(Genotype, Cost)> = vec![];
    let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    for _ in 0..amount {
        // Random value
        let value = interval.sample(&mut rng);

        for (i, section) in roulette_wheel.iter().enumerate() {
            if &value <= section {
                selection.push(&individuals[i]);
                selected_indices.insert(i);
                break;
            }
        }
    }

    // Return
    (selection, selected_indices.len())
}

fn tournament<'a>(
    tournament_size: usize,
    amount: usize,
    individuals: &'a [(Genotype, Cost)],
) -> (Vec<&'a (Genotype, Cost)>, usize) {
    let mut selection: Vec<&(Genotype, Cost)> = vec![];
    let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval =
        rand::distributions::Uniform::new_inclusive(0, individuals.len() - 1);

    for _ in 0..amount {
        // Create tournament participant list
        let mut tournament: Vec<(&'a (Genotype, Cost), usize)> = vec![];

        // Pick participants
        for _ in 0..tournament_size {
            let index = interval.sample(&mut rng);
            tournament.push((&individuals[index], index));
        }

        // Get the best from the tournament
        tournament.sort_by_key(|(_, x)| *x);

        // Store tournament winner and register index
        let winner = tournament.first().unwrap();
        selection.push(winner.0);
        selected_indices.insert(winner.1);
    }

    // Return
    (selection, selected_indices.len())
}

fn random<'a>(
    amount: usize,
    individuals: &'a [(Genotype, Cost)],
) -> (Vec<&'a (Genotype, Cost)>, usize) {
    let mut selection: Vec<&(Genotype, Cost)> = vec![];
    let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval =
        rand::distributions::Uniform::new_inclusive(0, individuals.len() - 1);

    for _ in 0..amount {
        let index = interval.sample(&mut rng);
        selection.push(&individuals[index]);
        selected_indices.insert(index);
    }

    // Return
    (selection, selected_indices.len())
}

fn linear_rank<'a>(
    amount: usize,
    individuals: &'a [(Genotype, Cost)],
) -> (Vec<&'a (Genotype, Cost)>, usize) {
    // Get population size
    let pop_size = individuals.len();

    // Define probability function (linear ranking)
    let probability = |rank: usize, pop_size: usize| -> f32 {
        let p = pop_size as f32;
        let r = rank as f32;
        (2. * (p - r)) / (p * (p + 1.))
    };

    // Calculate proportion
    let proportions: Vec<f32> = individuals
        .iter()
        .enumerate()
        .map(|(i, _)| probability(i, pop_size))
        .collect();

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
    let mut selection: Vec<&(Genotype, Cost)> = vec![];
    let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    for _ in 0..amount {
        // Random value
        let value = interval.sample(&mut rng);

        for (i, section) in roulette_wheel.iter().enumerate() {
            if &value <= section {
                selection.push(&individuals[i]);
                selected_indices.insert(i);
                break;
            }
        }
    }

    // Return
    (selection, selected_indices.len())
}

////////////////////////////////////////////////////////////////////////////////
