// Imports /////////////////////////////////////////////////////////////////////
use rand::{rngs::ThreadRng, Rng};

// Function ////////////////////////////////////////////////////////////////////

/// Multi point crossover. If a crossover rate of 100% is desired, use `None`
/// as parameter for the `rate`. This will skip the random number sampling.
pub fn uniform<'a, T>(
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

    // Perform crossover
    let zip = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| if rng.gen::<bool>() { (b, a) } else { (a, b) })
        .collect::<Vec<(&T, &T)>>();

    // Collect the genes
    let x = zip.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = zip.iter().map(|(_, y)| *y).collect::<Vec<_>>();

    // Return children
    (x, y)
}

////////////////////////////////////////////////////////////////////////////////
