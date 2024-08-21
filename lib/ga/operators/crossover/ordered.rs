// Imports /////////////////////////////////////////////////////////////////////
use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};

// Function ////////////////////////////////////////////////////////////////////
/// Ordered Crossover. If a crossover rate of 100% is desired, use `None`
/// as parameter for the `rate`. This will skip the random number sampling.
pub fn ordered<'a, T: Clone + PartialEq + Eq>(
    a: Vec<T>,
    b: Vec<T>,
    rate: Option<f32>,
    rng: &mut ThreadRng,
) -> (Vec<T>, Vec<T>) {
    // Decide whether the crossover should be performed or not.
    if let Some(rate) = rate {
        if rng.gen::<f32>() > rate {
            // No crossover, simply return the parents
            return (a.to_owned(), b.to_owned());
        }
    }

    // Get and store chromosome length
    let clen = a.len();

    // Create two random indices for the middle part of the ordered crossover.
    let mut all_indices = (0..clen).collect::<Vec<usize>>();
    all_indices.shuffle(rng);

    let mut indices = all_indices.into_iter().take(2).collect::<Vec<_>>();
    indices.sort();

    assert_eq!(indices.len(), 2);
    let i0 = indices[0];
    let i1 = indices[1];

    // Create first child
    let c0: Vec<T> = {
        // Middle part comes from parent a
        let middle = &a[i0..i1];

        // Get a view of parent b without the values from `middle`.
        let remainder = b
            .iter()
            .filter_map(|x| {
                if !middle.contains(x) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();

        // The part after the middle (called tail) of this child is now filled
        // from the calculated remainder.
        let tail = &remainder[0..(clen - i1)];

        // The part befor the middle (called head) of this child is now filled
        // with the remaining values from the remainder.
        let head = &remainder[(clen - i1)..];

        // Concatenate the three parts to get the full chromosome
        [head, middle, tail].concat()
    };

    // Create second child
    let c1: Vec<T> = {
        // Middle part comes from parent b
        let middle = &b[i0..i1];

        // Get a view of parent a without the values from `middle`.
        let remainder = a
            .iter()
            .filter_map(|x| {
                if !middle.contains(x) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();

        // The part after the middle (called tail) of this child is now filled
        // from the calculated remainder.
        let tail = &remainder[0..(clen - i1)];

        // The part befor the middle (called head) of this child is now filled
        // with the remaining values from the remainder.
        let head = &remainder[(clen - i1)..];

        // Concatenate the three parts to get the full chromosome
        [head, middle, tail].concat()
    };

    // Return
    (c0, c1)
}

////////////////////////////////////////////////////////////////////////////////
