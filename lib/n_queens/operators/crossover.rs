// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::Genotype;
use rand::prelude::Distribution;

// Enum ////////////////////////////////////////////////////////////////////////
pub enum Crossover {
    VariableSinglePoint,
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
            Self::VariableSinglePoint => {
                fixed(a, b, rate)
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
    if probabilty.sample(&mut rng) > rate { return (a.clone(), b.clone()) };

    // Split
    let interval = rand::distributions::Uniform::new_inclusive(1, a.0.len() - 2);
    let split_index = interval.sample(&mut rng);
    let (a0, a1) = a.0.split_at(split_index);
    let (b0, b1) = b.0.split_at(split_index);

    // Return
    (
        Genotype([a0, b1].concat()),
        Genotype([b0, a1].concat()),
    )

}

////////////////////////////////////////////////////////////////////////////////
