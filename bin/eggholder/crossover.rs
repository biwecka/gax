// Imports /////////////////////////////////////////////////////////////////////
use crate::chromosome::Chromosome;
use rand::prelude::SliceRandom;
use rand::Rng;

// Crossover Operations ////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Crossover method which simply swaps the `x1` values of the parents to
    /// yield the children.
    InterchangeX1Coordinates,

    /// This crossover method takes all four coordinates and distributes them
    /// randomly to the children, which may insert x0 values as x1 and vice
    /// versa.
    InterchangeAllCoordinates,
}

impl Crossover {
    /// Execute the crossover method, which is specified by the [`Crossover`]
    /// variant.
    pub fn exec(
        &self,
        p0: &Chromosome,
        p1: &Chromosome,
        rate: f32,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::InterchangeX1Coordinates => {
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() > rate {
                    return (p0.to_owned(), p1.to_owned());
                }

                let child0 = (p0.x0(), p1.x1()).into();
                let child1 = (p0.x1(), p1.x0()).into();

                (child0, child1)
            }

            Crossover::InterchangeAllCoordinates => {
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() > rate {
                    return (p0.to_owned(), p1.to_owned());
                }

                let mut coordinates = [p0.x0(), p0.x1(), p1.x0(), p1.x1()];
                coordinates.shuffle(&mut rng);

                let child0 = (coordinates[0], coordinates[1]).into();
                let child1 = (coordinates[2], coordinates[3]).into();

                (child0, child1)
            }
        }
    }
}
