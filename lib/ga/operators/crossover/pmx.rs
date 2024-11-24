// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashMap;
use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};
use std::hash::Hash;

// Function ////////////////////////////////////////////////////////////////////
/// PMX Crossover. If a crossover rate of 100% is desired, use `None`
/// as parameter for the `rate`. This will skip the random number sampling.
pub fn pmx<'a, T: Eq + Hash>(
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

    // Calculate lower and upper "cut" indices
    // Create two random indices for the middle part of the ordered crossover.
    let mut all_indices = (0..a.len()).collect::<Vec<usize>>();
    all_indices.shuffle(rng);

    let mut indices = all_indices.into_iter().take(2).collect::<Vec<_>>();
    indices.sort();

    assert_eq!(indices.len(), 2);
    let lower = indices[0];
    let upper = indices[1];

    // Split a into parts
    let a_l = &a[0..lower];
    let a_m = &a[lower..upper];
    let a_r = &a[upper..];

    // Split b into parts
    let b_l = &b[0..lower];
    let b_m = &b[lower..upper];
    let b_r = &b[upper..];

    // Create matcher
    let matcher = PmxMatcher::new(a_m, b_m);

    // Create child 0
    let c0_l = a_l.iter().map(|x| matcher.calc_x_to_y(x)).collect::<Vec<_>>();
    let c0_m = b_m;
    let c0_r = a_r.iter().map(|x| matcher.calc_x_to_y(x)).collect::<Vec<_>>();

    let mut c0 = c0_l.to_vec();
    c0.extend(c0_m);
    c0.extend(c0_r);

    // Create child 1
    let c1_l = b_l.iter().map(|x| matcher.calc_y_to_x(x)).collect::<Vec<_>>();
    let c1_m = a_m;
    let c1_r = b_r.iter().map(|x| matcher.calc_y_to_x(x)).collect::<Vec<_>>();

    let mut c1 = c1_l.to_vec();
    c1.extend(c1_m);
    c1.extend(c1_r);

    // Return
    (c0, c1)
}

// Helper Structs //////////////////////////////////////////////////////////////
struct PmxMatcher<'a, T: Eq + Hash> {
    x_to_y: HashMap<&'a T, &'a T>,
    y_to_x: HashMap<&'a T, &'a T>,
}

impl<'a, T: Eq + Hash> PmxMatcher<'a, T> {
    pub fn new(x: &'a [T], y: &'a [T]) -> Self {
        assert_eq!(x.len(), y.len());

        let mut x_to_y = HashMap::new();
        let mut y_to_x = HashMap::new();

        for i in 0..x.len() {
            let a = &x[i];
            let b = &y[i];

            x_to_y.insert(b, a);
            y_to_x.insert(a, b);
        }

        Self { x_to_y, y_to_x }
    }

    pub fn calc_x_to_y(&self, input: &'a T) -> &'a T {
        let mut result = input;

        while let Some(x) = self.x_to_y.get(&result) {
            result = *x;
        }

        result
    }

    pub fn calc_y_to_x(&self, input: &'a T) -> &'a T {
        let mut result = input;

        while let Some(x) = self.y_to_x.get(&result) {
            result = *x;
        }

        result
    }
}

////////////////////////////////////////////////////////////////////////////////
