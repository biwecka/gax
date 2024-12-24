// Imports /////////////////////////////////////////////////////////////////////
use crate::eggholder;
use rand::{distributions::Uniform, prelude::Distribution};

// Chromosome //////////////////////////////////////////////////////////////////
/// The chromosome only contains two values, which represent the input
/// variables - x1 and x2 - of the eggholder function.
#[derive(Clone, Debug)]
pub struct Chromosome(f64, f64);

impl Chromosome {
    /// Static helper function to generat a given amount of random chromosomes.
    pub fn generate(amount: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let uniform_dist = Uniform::new_inclusive(-512., 512.);

        let mut chromosomes = vec![];
        for _ in 0..amount {
            chromosomes.push(Chromosome(
                uniform_dist.sample(&mut rng),
                uniform_dist.sample(&mut rng),
            ));
        }

        chromosomes
    }

    /// Evaluate a chromosome.
    pub fn eval(&self) -> f64 {
        eggholder(self.0, self.1)
    }

    /// Getter for the `x0` value.
    pub fn x0(&self) -> f64 {
        self.0
    }

    /// Getter for the `x1` value.
    pub fn x1(&self) -> f64 {
        self.1
    }

    /// Setter for `x0`.
    pub fn set_x0(&mut self, x0: f64) {
        self.0 = x0;
    }

    /// Setter for `x1`.
    pub fn set_x1(&mut self, x1: f64) {
        self.1 = x1;
    }
}

/// "From" implementation to enable seamless conversion of a f64-tuple into
/// a [`Chromosome`].
impl From<(f64, f64)> for Chromosome {
    fn from(value: (f64, f64)) -> Self {
        Self(value.0, value.1)
    }
}

////////////////////////////////////////////////////////////////////////////////
