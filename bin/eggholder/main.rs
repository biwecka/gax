// Modules /////////////////////////////////////////////////////////////////////
mod chromosome;
mod crossover;
mod logger;
mod mutation;
mod selection;

// Imports /////////////////////////////////////////////////////////////////////
use chromosome::Chromosome;
use crossover::Crossover;
use logger::Logger;
use mutation::Mutation;
use rayon::prelude::*;
use selection::Selection;

// Functions ///////////////////////////////////////////////////////////////////
/// Eggholder test function
///
/// Defined as
///
/// `f(x_1, x_2) = -(x_2 + 47) * sin( sqrt( abs( x_2 + x_1/2 + 47 ) ) ) -
///                x_1 * sin( sqrt( abs( x_1 - (x_2 + 47) ) ) )`
///
/// where `x_i \in [-512, 512]`.
///
/// The global minimum is at * `f(x_1, x_2) = f(512, 404.2319) = -959.6407`.
pub fn eggholder(x1: f64, x2: f64) -> f64 {
    assert!(x1 >= -512.);
    assert!(x1 <= 512.);
    assert!(x2 >= -512.);
    assert!(x2 <= 512.);

    -(x2 + 47.0) * (x2 + x1 / 2.0 + 47.0).abs().sqrt().sin()
        - x1 * (x1 - (x2 + 47.0)).abs().sqrt().sin()
}

/// y: average
/// u: new sample
/// t: window size
pub fn pt1(y: f32, u: f32, t: f32) -> f32 {
    if t + 1.0 != 1.0 {
        y + ((u - y) / t)
    } else {
        u
    }
}

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Logger
    let logger = Logger::default();
    logger.draw_eggholder_function();

    // Parameters
    let pop_size = 500;
    let crossover_rate = 1.;
    let mutation_rate = 0.01;
    let mut mutation_sd = 0.01;

    let elite_size = 10;
    let selection = Selection::Tournament(10);
    let crossover = Crossover::InterchangeX2Coordinates;
    let mutation = Mutation::RandomGauss;

    let mut succes_rate = 0.1;
    let target_sura = 0.1;

    // Init Population
    let mut population = Chromosome::generate(pop_size)
        .into_iter()
        .map(|c| {
            let evaluation = c.eval();
            (c, evaluation)
        })
        .collect::<Vec<(Chromosome, f64)>>();

    population.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    // Runtime Data
    let mut generation = 0;
    let mut best_chromosome = Chromosome::generate(1).first().unwrap().clone();
    let mut best_value = 2000.;

    loop {
        // Inc generation counter
        generation += 1;

        // Select
        let selection_size = {
            let tmp = pop_size - elite_size;
            if tmp % 2 == 0 {
                tmp
            } else {
                tmp + 1
            }
        };

        let parents = selection.exec(selection_size, &population);

        // Crossover, Mutation
        let mut offspring = parents
            .par_chunks(2)
            // .chunks(2)
            .map(|parents| {
                // Extract parents
                let a = parents[0];
                let b = parents[1];

                // Crossover
                let (mut x0, mut x1) =
                    crossover.exec(&a.0, &b.0, crossover_rate);

                // Mutation
                mutation.exec(&mut x0, mutation_rate, mutation_sd);
                mutation.exec(&mut x1, mutation_rate, mutation_sd);

                // Evaluation
                let c0 = {
                    let evaluation = x0.eval();
                    (x0, evaluation)
                };

                let c1 = {
                    let evaluation = x1.eval();
                    (x1, evaluation)
                };

                vec![c0, c1]
            })
            .flatten()
            .collect::<Vec<(Chromosome, f64)>>();

        offspring.truncate(pop_size - elite_size);

        // Replace
        population.splice(elite_size.., offspring);

        // Sort new population
        population.par_sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        // Update "best" and calculate success rate
        let current_best = population.first().unwrap();
        if current_best.1 < best_value {
            let tmp = current_best.clone();
            best_chromosome = tmp.0;
            best_value = tmp.1;

            // Update success rate
            succes_rate = pt1(succes_rate, 1., 100.);

            // Reset standard deviation
            mutation_sd = 0.01;
        } else {
            succes_rate = pt1(succes_rate, 0., 100.);
        }

        // Modify standard deviation
        if succes_rate < target_sura {
            // Calc diff
            let diff = target_sura - succes_rate; // always positive

            // Multiply factor
            let addition = 0.1 * diff;

            // add to mutation rate
            mutation_sd += addition;

            if mutation_sd > 200. {
                mutation_sd = 0.01;
            }
        }

        // Print status
        println!(
            "[{:>7}] best:{:>4} (x:{:>3.4}, y:{:>3.4})",
            generation,
            best_value,
            best_chromosome.x0(),
            best_chromosome.x1()
        );

        logger.draw_population(generation, &population);
        logger.log_success_rate(generation, succes_rate, target_sura);
        logger.log_mutation_std_dev(generation, mutation_sd);

        // Terminate
        if generation >= 50_000 || best_value < -959.6406 {
            break;
        }

        // std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

////////////////////////////////////////////////////////////////////////////////
