use rand::prelude::Distribution;
use crate::chromosome::Chromosome;

pub enum Selection {
    RouletteWheel,
    Tournament(usize),
}

impl Selection {
    pub fn exec<'a>(&self, amount: usize, individuals: &'a [(Chromosome, f64)]) -> Vec<&'a (Chromosome, f64)> {
        match self {
            Selection::RouletteWheel => {
                roulette_wheel_usize(amount, individuals)
            },

            Selection::Tournament(n) => {
                tournament_usize(*n, amount, individuals)
            }
        }
    }
}

fn roulette_wheel_usize(
    amount: usize,
    individuals: &[(Chromosome, f64)],
) -> Vec<&(Chromosome, f64)> {
    // Extract cost (convert objective value to usize)
    let costs: Vec<f64> = individuals
        .iter()
        .map(|(_, c)| *c)
        .collect();

    // Calculate max
    let mut max_cost: f64 = 1000.;
    for c in &costs {
        if *c < max_cost {
            max_cost = *c;
        }
    }

    // Invert
    let inverted_costs: Vec<f64> =
        costs.iter().map(|x| (max_cost - *x) * (max_cost - *x)).collect();

    // Calc total cost and highest cost
    let total_cost: f64 = inverted_costs.iter().sum();

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
    let mut selection: Vec<&(Chromosome, f64)> = vec![];
    // let mut selected_indices = HashSet::<usize>::new();
    let mut rng = rand::thread_rng();
    let interval = rand::distributions::Uniform::new_inclusive(0., 1.);

    for _ in 0..amount {
        // Random value
        let value = interval.sample(&mut rng);

        for (i, section) in roulette_wheel.iter().enumerate() {
            if &value <= section {
                selection.push(&individuals[i]);
                // selected_indices.insert(i);
                break;
            }
        }
    }

    // Return
    selection //, selected_indices.len())
}



fn tournament_usize(
    tournament_size: usize,
    amount: usize,
    individuals: &[(Chromosome, f64)],
) -> Vec<&(Chromosome, f64)> {
    let mut selection: Vec<&(Chromosome, f64)> = vec![];


    let mut rng = rand::thread_rng();
    let interval =
        rand::distributions::Uniform::new_inclusive(0, individuals.len() - 1);

    for _ in 0..amount {
        // Create tournament participant list
        let mut tournament: Vec<(&(Chromosome, f64), usize)> = vec![];

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
    }

    // Return
    selection
}
