// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::Genotype;
use hashbrown::HashSet;
use rand::prelude::Distribution;

// Enum ////////////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    VariableSinglePoint,
    VariableMultiPoint(usize),
}

impl Crossover {
    pub fn exec(
        &self,
        a: &Genotype,
        b: &Genotype,
        rate: f32,
        //context: &Context,
    ) -> (Genotype, Genotype) {
        match self {
            Self::VariableSinglePoint => fixed(a, b, rate),
            Self::VariableMultiPoint(points) => {
                variable_multi_point(*points, a, b, rate)
            }
        }
    }
}

// Implementations /////////////////////////////////////////////////////////////
fn fixed(
    a: &Genotype,
    b: &Genotype,
    rate: f32,
    // ctx: &Context,
) -> (Genotype, Genotype) {
    // Randomness
    let mut rng = rand::thread_rng();
    let probabilty = rand::distributions::Uniform::new_inclusive(0., 1.);
    if probabilty.sample(&mut rng) > rate {
        return (a.clone(), b.clone());
    };

    // Split
    let interval =
        rand::distributions::Uniform::new_inclusive(1, a.0.len() - 2);
    let split_index = interval.sample(&mut rng);
    let (a0, a1) = a.0.split_at(split_index);
    let (b0, b1) = b.0.split_at(split_index);

    // Return
    (Genotype([a0, b1].concat()), Genotype([b0, a1].concat()))
}

fn variable_multi_point(
    points: usize,
    a: &Genotype,
    b: &Genotype,
    rate: f32,
) -> (Genotype, Genotype) {
    let mut rng = rand::thread_rng();

    let probabilty = rand::distributions::Uniform::new_inclusive(0., 1.);
    if probabilty.sample(&mut rng) > rate {
        return (a.clone(), b.clone());
    };

    let split_points =
        rand::distributions::Uniform::new_inclusive(1, (a.0.len() - 1) / 2);

    let mut splits_set = HashSet::new();
    while splits_set.len() < points {
        let new_split = split_points.sample(&mut rng) * 2;
        splits_set.insert(new_split);
    }

    let mut switch = false;
    let zip: Vec<(&usize, &usize)> =
        a.0.iter()
            .zip(b.0.iter())
            .enumerate()
            .map(|(i, (a, b))| {
                if splits_set.contains(&i) {
                    switch = !switch;
                }

                if switch {
                    (b, a)
                } else {
                    (a, b)
                }
            })
            .collect();

    let x = zip.iter().map(|(x, _)| *x.to_owned()).collect::<Vec<usize>>();
    let y = zip.iter().map(|(_, y)| *y.to_owned()).collect::<Vec<usize>>();

    (Genotype(x), Genotype(y))
}

////////////////////////////////////////////////////////////////////////////////
