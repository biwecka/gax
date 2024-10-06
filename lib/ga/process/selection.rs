// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};
use hashbrown::HashSet;
use rand::prelude::Distribution;

// Trait ///////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// selection methods. The trait ensures each selection method returns two
/// things:
/// 1) Vec<&'a (Ge, Ov)>    representing the selected individuals with their
///                         objective value
/// 2) usize                representing the amount of **distinct** selections.
///
/// The amount of distinct selections is part of the genetic algorithms metrics.
///
pub trait Selection<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    T,
>: Send + Sync
{
    fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize);
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Select {
    RouletteWheel,
    Tournament(usize),
    Random,

    // Linear rank selection with configurable selection pressure.
    // Parameters:
    // 1) selection pressure (>= 1.0)
    LinearRank(f32),
}

impl<Ov: ObjectiveValue + Into<usize>, Ctx: Context, Ge: Genotype<Ctx>>
    Selection<Ov, Ctx, Ge, usize> for Select
{
    fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize) {
        match self {
            Self::RouletteWheel => roulette_wheel_usize(amount, individuals),
            Self::Tournament(n) => tournament_usize(*n, amount, individuals),
            Self::Random => random_usize(amount, individuals),
            Self::LinearRank(sp) => linear_rank_usize(*sp, amount, individuals),
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////
fn roulette_wheel_usize<
    Ov: ObjectiveValue + Into<usize>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
>(
    amount: usize,
    individuals: &[(Ge, Ov)],
) -> (Vec<&(Ge, Ov)>, usize) {
    // Extract cost (convert objective value to usize)
    let costs: Vec<usize> = individuals
        .iter()
        .map(|(_, c)| c.to_owned().into()) // Convert `Ov` into usize
        .collect();

    // Calculate max
    let max_cost: usize = *costs.iter().max().unwrap();

    // Invert
    let inverted_costs: Vec<usize> =
        costs.iter().map(|x| (max_cost - *x) * (max_cost - *x)).collect();

    // Calc total cost and highest cost
    let total_cost: usize = inverted_costs.iter().sum();

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
    let mut selection: Vec<&(Ge, Ov)> = vec![];
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

fn tournament_usize<
    Ov: ObjectiveValue + Into<usize>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
>(
    tournament_size: usize,
    amount: usize,
    individuals: &[(Ge, Ov)],
) -> (Vec<&(Ge, Ov)>, usize) {
    let mut selection: Vec<&(Ge, Ov)> = vec![];
    let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval =
        rand::distributions::Uniform::new_inclusive(0, individuals.len() - 1);

    for _ in 0..amount {
        // Create tournament participant list
        let mut tournament: Vec<(&(Ge, Ov), usize)> = vec![];

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

fn random_usize<
    Ov: ObjectiveValue + Into<usize>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
>(
    amount: usize,
    individuals: &[(Ge, Ov)],
) -> (Vec<&(Ge, Ov)>, usize) {
    let mut selection: Vec<&(Ge, Ov)> = vec![];
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

/// Probability function for the "linear rank selection with configurable
/// selection pressure".
/// Parameters:
/// - `sp`: selection pressure
///         value = 1 -> random selection
///         value = 2 -> last rank has probability = 0
///         value > 2 -> probability is zero for ranks greater or equal than
///                      r_0 = n/2 + n/(2*(sp-1))
///
/// - `n` : population size
/// - `r` : rank
fn linear_selection_probability(r: usize, n: usize, sp: f32) -> f32 {
    // Convert inputs to f32
    let n = n as f32;
    let r = r as f32;

    // Calculate probability
    let p = (2. - sp) / n + 2. * (sp - 1.) * ((n - r) / (n.powi(2)));

    if p >= 0. {
        p
    } else {
        0.
    }
}

fn linear_rank_usize<
    Ov: ObjectiveValue + Into<usize>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
>(
    selection_pressure: f32,
    amount: usize,
    individuals: &[(Ge, Ov)],
) -> (Vec<&(Ge, Ov)>, usize) {
    // Get population size
    let pop_size = individuals.len();

    // Define probability function (linear ranking)
    // let probability = |rank: usize, pop_size: usize| -> f32 {
    //     let p = pop_size as f32;
    //     let r = rank as f32;
    //     (2. * (p - r)) / (p * (p + 1.))
    // };

    // Calculate proportion
    let proportions: Vec<f32> = individuals
        .iter()
        .enumerate()
        .map(|(i, _)| {
            linear_selection_probability(i, pop_size, selection_pressure)
        })
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
    let mut selection: Vec<&(Ge, Ov)> = vec![];
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
