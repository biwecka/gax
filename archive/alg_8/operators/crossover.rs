use ndarray::{Array, Array2};
use rand::rngs::ThreadRng;

// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    VariableSinglePoint,

    /// Variable n-point crossover takes two arguments:
    /// 1) usize    representing the amount of crossover points
    VariableNPoint(usize),

    /// PMX
    Pmx,

    /// Ordered Crossover
    Ordered,
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn identifier(&self) -> String {
        match self {
            Self::VariableSinglePoint => "var-s-pt".into(),
            Self::VariableNPoint(n) => format!("var-{n}-pt"),
            Self::Pmx => "pmx".into(),
            Self::Ordered => "ord".into(),
        }
    }
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint => {
                let p0 = &parent_0.0;
                let p1 = &parent_1.0;

                let mut c0 = Array2::<u8>::default(p0.dim());
                let mut c1 = Array2::<u8>::default(p1.dim());

                for row_idx in 0..p0.shape()[0] {
                    let p0row = p0.row(row_idx);
                    let p1row = p1.row(row_idx);

                    let (a, b) = ga::operators::crossover::single_point(
                        p0row.as_slice().unwrap(),
                        p1row.as_slice().unwrap(),
                        rate,
                        rng,
                    );

                    // Convert vectors to ndarray::Array (to be able to use
                    // them with the `assign` method).
                    let x = Array::from_iter(a.into_iter().cloned());
                    let y = Array::from_iter(b.into_iter().cloned());

                    c0.row_mut(row_idx).assign(&x);
                    c1.row_mut(row_idx).assign(&y);
                }

                (c0.into(), c1.into())
            }

            Crossover::VariableNPoint(num_points) => {
                let p0 = &parent_0.0;
                let p1 = &parent_1.0;

                let mut c0 = Array2::<u8>::default(p0.dim());
                let mut c1 = Array2::<u8>::default(p1.dim());

                for row_idx in 0..p0.shape()[0] {
                    let p0row = p0.row(row_idx);
                    let p1row = p1.row(row_idx);

                    let (a, b) = ga::operators::crossover::multi_point(
                        p0row.as_slice().unwrap(),
                        p1row.as_slice().unwrap(),
                        rate,
                        *num_points,
                        rng,
                    );

                    // Convert vectors to ndarray::Array (to be able to use
                    // them with the `assign` method).
                    let x = Array::from_iter(a.into_iter().cloned());
                    let y = Array::from_iter(b.into_iter().cloned());

                    c0.row_mut(row_idx).assign(&x);
                    c1.row_mut(row_idx).assign(&y);
                }

                (c0.into(), c1.into())
            }

            Crossover::Pmx => {
                let p0 = &parent_0.0;
                let p1 = &parent_1.0;

                let mut c0 = Array2::<u8>::default(p0.dim());
                let mut c1 = Array2::<u8>::default(p1.dim());

                for row_idx in 0..p0.shape()[0] {
                    let p0row = p0.row(row_idx);
                    let p1row = p1.row(row_idx);

                    let (a, b) = ga::operators::crossover::pmx(
                        p0row.as_slice().unwrap(),
                        p1row.as_slice().unwrap(),
                        rate,
                        rng,
                    );

                    // Convert vectors to ndarray::Array (to be able to use
                    // them with the `assign` method).
                    let x = Array::from_iter(a.into_iter().cloned());
                    let y = Array::from_iter(b.into_iter().cloned());

                    c0.row_mut(row_idx).assign(&x);
                    c1.row_mut(row_idx).assign(&y);
                }

                (c0.into(), c1.into())
            }

            Crossover::Ordered => {
                let p0 = &parent_0.0;
                let p1 = &parent_1.0;

                let mut c0 = Array2::<u8>::default(p0.dim());
                let mut c1 = Array2::<u8>::default(p1.dim());

                for row_idx in 0..p0.shape()[0] {
                    let p0row = p0.row(row_idx);
                    let p1row = p1.row(row_idx);

                    let (a, b) = ga::operators::crossover::ordered(
                        p0row.to_vec(),
                        p1row.to_vec(),
                        rate,
                        rng,
                    );

                    // Convert vectors to ndarray::Array (to be able to use
                    // them with the `assign` method).
                    let x = Array::from_iter(a.into_iter());
                    let y = Array::from_iter(b.into_iter());

                    c0.row_mut(row_idx).assign(&x);
                    c1.row_mut(row_idx).assign(&y);
                }

                (c0.into(), c1.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
