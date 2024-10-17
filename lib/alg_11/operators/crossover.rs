// Imports /////////////////////////////////////////////////////////////////////

use bits::Bits32;
use rand::{seq::SliceRandom, Rng};

use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
#[derive(Clone)]
pub enum Crossover {
    /// Uniform crossover simply exchanges genes (single Bit32 structs) between
    /// chromosomes at the same position.
    Uniform,

    /// Trade(n) crossover: trades `n * 2` time allocations between two
    /// chromosomes.
    /// `n * 2` because 1 trade = giving up 1 time & receiving 1 time
    Trade(usize),
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn identifier(&self) -> String {
        match self {
            Self::Uniform => "uni".into(),
            Self::Trade(n) => format!("trd{n}"),
        }
    }

    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut rand::prelude::ThreadRng,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Self::Uniform => uniform(parent_0, parent_1, rate, rng),
            Self::Trade(n) => trade(*n, parent_0, parent_1, rate, rng),
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
fn uniform(
    p0: &Chromosome,
    p1: &Chromosome,
    rate: Option<f32>,
    rng: &mut rand::prelude::ThreadRng,
) -> (Chromosome, Chromosome) {
    let (c0, c1) = ga::operators::crossover::uniform(&p0.0, &p1.0, rate, rng);

    let x0 = c0.into_iter().cloned().collect::<Vec<_>>();
    let x1 = c1.into_iter().cloned().collect::<Vec<_>>();

    (x0.into(), x1.into())
}

fn trade(
    n: usize,
    p0: &Chromosome,
    p1: &Chromosome,
    rate: Option<f32>,
    rng: &mut rand::prelude::ThreadRng,
) -> (Chromosome, Chromosome) {
    // Decide whether the crossover should be performed or not.
    if let Some(rate) = rate {
        if rng.gen::<f32>() > rate {
            // No crossover, simply return the parents
            return (p0.to_owned(), p1.to_owned());
        }
    }

    let (p0, p1): (Vec<Bits32>, Vec<Bits32>) =
        p0.0.iter()
            .zip(p1.0.iter())
            .map(|(p0, p1)| {
                // Get the time allocation vector of both parents
                let mut y_e0 = *p0;
                let mut y_e1 = *p1;

                // Negate both
                let y_e0_inv = !y_e0;
                let y_e1_inv = !y_e1;

                // Calc possible trades from e_0 to e_1
                let trade_0_to_1 = y_e0 & y_e1_inv;

                // Calc possible trades from e_1 to e_0
                let trade_1_to_0 = y_e1 & y_e0_inv;

                // Calculate the maximum number of doable trades
                let num_trades = trade_0_to_1
                    .ones()
                    .count()
                    .min(trade_1_to_0.ones().count())
                    .min(n);

                // Perform trades from e0 to e1
                let mut trade_indices = trade_0_to_1.ones().collect::<Vec<_>>();
                trade_indices.shuffle(&mut rand::thread_rng());

                for i in trade_indices.into_iter().take(num_trades) {
                    y_e0.unset(i);
                    y_e1.set(i);
                }

                // Perform trades from e_1 to e_0
                let mut trade_indices = trade_1_to_0.ones().collect::<Vec<_>>();
                trade_indices.shuffle(&mut rand::thread_rng());

                for i in trade_indices.into_iter().take(num_trades) {
                    y_e1.unset(i);
                    y_e0.set(i);
                }

                (y_e0, y_e1)
            })
            .unzip();

    (p0.into(), p1.into())
}

////////////////////////////////////////////////////////////////////////////////
