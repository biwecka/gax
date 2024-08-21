// Imports /////////////////////////////////////////////////////////////////////
use rand::{rngs::ThreadRng, Rng};

// Function ////////////////////////////////////////////////////////////////////
/// Single point crossover. If a crossover rate of 100% is desired, use `None`
/// as parameter for the `rate`. This will skip the random number sampling.
pub fn single_point<'a, T>(
    a: &'a [T],
    b: &'a [T],
    rate: Option<f32>,
    rng: &mut ThreadRng,
) -> (Vec<&'a T>, Vec<&'a T>) {
    // Decide whether the crossover should be performed or not.
    if let Some(rate) = rate {
        if rng.gen::<f32>() > rate {
            // No crossover, simply return the parents
            return (a.iter().collect(), b.iter().collect());
        }
    }

    // Generate index for single point crossover
    let split_index = rng.gen_range(1..a.len() - 1);

    // This flag is switched over if the iteration reaches the `split_index`.
    // A value of `false` means that the genes are simply copied from the
    // parents to the respective offspring.
    // A value of `true` means that the genes are switched and then copied
    // to the offspring.
    let mut switch_genes = false;

    // Perform crossover
    let zip = a
        .iter()
        .zip(b.iter())
        .enumerate()
        .map(|(index, (a, b))| {
            // Update flag
            if index == split_index {
                switch_genes = !switch_genes;
            }

            // Crossover
            if switch_genes {
                (b, a)
            } else {
                (a, b)
            }
        })
        .collect::<Vec<(&T, &T)>>();

    // Collect the genes
    let child_0 = zip.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let child_1 = zip.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    // Return
    (child_0, child_1)
}

////////////////////////////////////////////////////////////////////////////////
