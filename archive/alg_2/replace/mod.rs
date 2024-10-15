// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::chromosome::Chromosome;

// Functions ///////////////////////////////////////////////////////////////////
pub fn elite_best_n(
    n: usize,
    population: Vec<(Chromosome, usize)>,
    children: Vec<(Chromosome, usize)>,
) -> Vec<Chromosome> {
    // Get elite
    let mut elite = population[0..n]
        .iter()
        .cloned()
        .map(|(chromosome, _)| chromosome)
        .collect::<Vec<Chromosome>>();

    // Get "top" of children
    let mut new_pop = children[0..children.len() - n]
        .iter()
        .cloned()
        .map(|(chromosome, _)| chromosome)
        .collect::<Vec<Chromosome>>();

    new_pop.append(&mut elite);

    // Return
    new_pop
}

////////////////////////////////////////////////////////////////////////////////
