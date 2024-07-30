// Imports /////////////////////////////////////////////////////////////////////
use crate::{fitness::Cost, population::Chromosome};

// Functions ///////////////////////////////////////////////////////////////////
pub fn elite_best_n(
    n: usize,
    population: Vec<(Chromosome, Cost)>,
    children: Vec<(Chromosome, Cost)>
) -> Vec<Chromosome> {
    // Get elite
    let mut elite = population[0..n]
        .to_vec()
        .into_iter()
        .map(|(chromosome, _)| chromosome)
        .collect::<Vec<Chromosome>>();

    // Get "top" of children
    let mut new_pop = children[0..children.len() - n]
        .to_vec()
        .into_iter()
        .map(|(chromosome, _)| chromosome)
        .collect::<Vec<Chromosome>>();

    new_pop.append(&mut elite);

    // Return
    new_pop
}


////////////////////////////////////////////////////////////////////////////////
