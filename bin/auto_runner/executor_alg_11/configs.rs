// Imports /////////////////////////////////////////////////////////////////////
use alg_11::{
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
#[rustfmt::skip]
pub fn configs() -> Vec<Config> {
    vec![
        // >>> Try out all the best configurations without dynamics <<<<<<<<<<<<

        // Mu(0.01) | LinRnk(1.8) | Trade(1) | Trade | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.8),
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(2.0) | Trade(1) | Trade | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(2.0),
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Roulette | Trade(1) | Trade | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(4) | Trade(1) | Trade | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(4),
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(8) | Trade(1) | Trade | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },


        // >>>>>>>>>>>>>>>>>>>>> ------------ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>   DYNAMICS   <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>> ------------ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        // Mu(0.015) | LinRnk(1.4) | Trade(1) | Trade | Full
        //
        // -> dyn: StateMachine
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.015,
                selection: Select::LinearRank(1.4),
                crossover: Crossover::Trade(1),
                mutation: Mutation::Trade,
                replacement: Replace::Full,
            },

            dynamics: Some(vec![
                Dynamic::StateMachine
            ]),
        },
    ]
}

////////////////////////////////////////////////////////////////////////////////
