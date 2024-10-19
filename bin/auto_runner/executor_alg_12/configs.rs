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
#[rustfmt::skip]
pub fn configs() -> Vec<Config> {
    vec![
        // >>> Try out all selection methods with VarSPt+UniSw <<<<<<<<<<<<<<<<<

        // Mu(0.01) | Roulette | VarSPt | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(4) | VarSPt | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(4),
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(8) | VarSPt | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(2.0) | VarSPt | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(2.0),
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(1.5) | VarSPt | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },


        // >>> Try out all selection methods with PMX+UniSw <<<<<<<<<<<<<<<<<<<<

        // Mu(0.01) | Roulette | PMX | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(4) | Pmx | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(4),
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(8) | Pmx | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(2.0) | Pmx | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(2.0),
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(1.5) | Pmx | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },


        // >>> Try out all selection methods with Ordered+UniSw <<<<<<<<<<<<<<<<

        // Mu(0.01) | Roulette | Ordered | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(4) | Ordered | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(4),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | Tournament(8) | Ordered | UniSw | El(1)
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

        // Mu(0.01) | LinRnk(2.0) | Ordered | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(2.0),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(1.5) | Ordered | UniSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },


        // >>> Try out some configs with GaussSwap but without dynamic <<<<<<<<<

        // Mu(0.01) | Tournament(8) | Ordered | GaussSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.015) | Tournament(8) | Ordered | GaussSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.015,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(1.5) | Ordered | GaussSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.015) | LinRnk(1.5) | Ordered | GaussSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.015,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },

        // Mu(0.01) | LinRnk(1.5) | Pmx | GaussSw | El(1)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Pmx,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: None,
        },



        // >>>>>>>>>>>>>>>>>>>>> ------------ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>>   DYNAMICS   <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        // >>>>>>>>>>>>>>>>>>>>> ------------ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        // >>> Try out some with: Mu:GaussSwap + Dyn:GausRandEvent <<<<<<<<<<<<<

        // Mu(0.01) | LinRnk(1.5) | Ordered | GaussSw | El(1)
        //
        // -> dyn: GaussRandEvent(0.01)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::GaussRandEvent(0.01),
            ]),
        },

        // Mu(0.015) | LinRnk(1.5) | Ordered | GaussSw | El(1)
        //
        // -> dyn: GaussRandEvent(0.01)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.015,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::GaussRandEvent(0.01),
            ]),
        },

        // Mu(0.005) | LinRnk(1.5) | Ordered | GaussSw | El(1)
        //
        // -> dyn: GaussRandEvent(0.01)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.005,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::GaussRandEvent(0.01),
            ]),
        },

        // Mu(0.01) | Tournament(8) | Ordered | GaussSw | El(1)
        //
        // -> dyn: GaussRandEvent(0.01)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::GaussRandEvent(0.01),
            ]),
        },

        // Mu(0.01) | Tournament(8) | Pmx | GaussSw | El(1)
        //
        // -> dyn: GaussRandEvent(0.01)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Pmx,
                mutation: Mutation::GaussSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::GaussRandEvent(0.01),
            ]),
        },



        // >>> Try out other dynamics <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        // Mu(0.01) | Tournament(8) | Ordered | UniSw | El(1)
        //
        // -> dyn: MutRateCos(0.01, 0.005, 200, None)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::MutRateCos(0.01, 0.005, 200, None)
            ]),
        },

        // Mu(0.01) | LinRnk(1.5) | Ordered | UniSw | El(1)
        //
        // -> dyn: MutRateCos(0.01, 0.005, 200, None)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::MutRateCos(0.01, 0.005, 200, None)
            ]),
        },


        // Mu(0.01) | Tournament(8) | Ordered | UniSw | El(1)
        //
        // -> dyn: VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::Tournament(8),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
            ]),
        },

        // Mu(0.01) | LinRnk(1.5) | Ordered | UniSw | El(1)
        //
        // -> dyn: VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.5),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
            ]),
        },

        // Mu(0.01) | LinRnk(1.0) | Ordered | UniSw | El(1)
        //
        // -> dyn: IncLinearRankSelectionPressure(20, 0.01, 2.5, 2_000)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.0),
                crossover: Crossover::Ordered,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::IncLinearRankSelectionPressure(20, 0.01, 2.5, 2_000)
            ]),
        },


        // Mu(0.015) | LinRnk(1.4) | Pmx | UniSw | Full
        //
        // The state machine dynamic overwrites the parameters anyways in its
        // setup function. Because of that, the state machine dynamic cannot
        // be tested with other parameters (as it overwrites them anyways).
        //
        // -> dyn: StateMachine
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.015,
                selection: Select::LinearRank(1.4),
                crossover: Crossover::Pmx,
                mutation: Mutation::UniformSwap,
                replacement: Replace::Full,
            },

            dynamics: Some(vec![
                Dynamic::StateMachine
            ]),
        },


        // >>> Check if VarSPt is better with dynamic <<<<<<<<<<<<<<<<<<<<<<<<<<

        // Mu(0.01) | Roulette | VarSPt | UniSw | El(1)
        //
        // -> dyn: MutRateCos(0.01, 0.005, 200, None)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::MutRateCos(0.01, 0.005, 200, None)
            ])
        },

        // Mu(0.01) | Roulette | VarSPt | UniSw | El(1)
        //
        // -> dyn: VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::RouletteWheel,
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::VarMutRateTargetMeanSin(1.10, 1., 0.05, 200)
            ])
        },

        // Mu(0.01) | LinRnk(1.0) | VarSPt | UniSw | El(1)
        //
        // -> dyn: IncLinearRankSelectionPressure(20, 0.01, 2.5, 2_000)
        Config {
            params: AutoRunParameters {
                population_size: 1_000,
                mutation_rate: 0.01,
                selection: Select::LinearRank(1.0),
                crossover: Crossover::VariableSinglePoint,
                mutation: Mutation::UniformSwap,
                replacement: Replace::EliteAbsolute(1),
            },

            dynamics: Some(vec![
                Dynamic::IncLinearRankSelectionPressure(20, 0.01, 2.5, 2_000)
            ])
        },
    ]
}

////////////////////////////////////////////////////////////////////////////////
