// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashSet;
use rand::prelude::Distribution;
use crate::encoding::{Context, Genotype, ObjectiveValue};

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
pub trait Selection<Ov: ObjectiveValue + Into<T>, Ctx: Context, Ge: Genotype<Ctx>, T> {
    fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize)
    // where
    //     T: From<&'a Ov>,
    ;
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Select {
    RouletteWheel,
}

impl<Ov: ObjectiveValue + Into<usize>, Ctx: Context, Ge: Genotype<Ctx>> Selection<Ov, Ctx, Ge, usize> for Select {
    fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize) {
        match self {
            Self::RouletteWheel => {
                roulette_wheel_usize(amount, individuals)
            }
        }
    }
}


// Functions ///////////////////////////////////////////////////////////////////
fn roulette_wheel_usize<'a, Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>(
    amount: usize,
    individuals: &'a [(Ge, Ov)],
) -> (Vec<&'a (Ge, Ov)>, usize)
where
    Ov: Into<usize>,
    // usize: From<Ov>,
{
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
