// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Cost, Genotype};
use hashbrown::HashSet;
use rand::prelude::Distribution;

// Enum ////////////////////////////////////////////////////////////////////////
pub enum Selection {
    RouletteWheel,
}

impl Selection {
    pub fn exec<'a>(
        &self,
        amount: usize,
        individuals: &'a [(Genotype, Cost)],
    ) -> (Vec<&'a (Genotype, Cost)>, usize) {
        match self {
            Self::RouletteWheel => roulette_wheel(amount, individuals),
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
    let inverted_costs: Vec<usize> = costs
        .iter()
        .map(|x| max_cost - x)
        .collect();


    // Calc total cost and highest cost
    let total_cost: usize = inverted_costs.iter().map(|x| x).sum();

    // Calculate proportion
    let proportions: Vec<f32> = inverted_costs
        .iter()
        .map(|x| *x as f32 / total_cost as f32)
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