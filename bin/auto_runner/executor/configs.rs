// Imports /////////////////////////////////////////////////////////////////////
use alg_12::{
    dynamics::Dynamic,
    operators::{Crossover, Mutation},
    AutoRunParameters,
};
use ga::process::{replacement::Replace, selection::Select};

// Config Struct ///////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Config {
    pub params: AutoRunParameters,
    pub dynamics: Option<Vec<Dynamic>>,
}

// Configurations //////////////////////////////////////////////////////////////
pub fn configs() -> Vec<Config> {
    vec![
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap, // <- w/o dynam.
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap, // <- w/o dynam.
                replacement: Replace::EliteAbsolute(10), // <- !!!
            },

            dynamics: None,
        },
    ]
}

////////////////////////////////////////////////////////////////////////////////
