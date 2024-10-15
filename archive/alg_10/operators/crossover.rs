// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context, EventGene};
use rand::{rngs::ThreadRng, seq::SliceRandom};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Uniform crossover
    Uniform,

    /// Trade(n) crossover: trades `n * 2` time allocations between two
    /// chromosomes.
    /// `n * 2` because 1 trade = giving up 1 time & receiving 1 time
    Trade(usize),
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        _ctx: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::Uniform => {
                let (c0, c1) = ga::operators::crossover::uniform(
                    &parent_0.0,
                    &parent_1.0,
                    rate,
                    rng,
                );

                let x0 = c0.into_iter().cloned().collect::<Vec<_>>();
                let x1 = c1.into_iter().cloned().collect::<Vec<_>>();

                (x0.into(), x1.into())
            }

            Crossover::Trade(n) => {
                let (p0, p1): (Vec<EventGene>, Vec<EventGene>) = parent_0
                    .0
                    .iter()
                    .zip(parent_1.0.iter())
                    .map(|(p0, p1)| {
                        // Get the time allocation vector of both parents
                        let mut y_e_0 = p0.times.clone();
                        let mut y_e_1 = p1.times.clone();

                        // Negate both
                        let y_e_0_inv = !y_e_0.clone();
                        let y_e_1_inv = !y_e_1.clone();

                        // Calc possible trades from e_0 to e_1
                        let trade_0_to_1 = y_e_0.clone() & y_e_1_inv;

                        // Calc possible trades from e_1 to e_0
                        let trade_1_to_0 = y_e_1.clone() & y_e_0_inv;

                        // Calculate the maximum number of doable trades
                        let num_trades = trade_0_to_1
                            .count_ones()
                            .min(trade_1_to_0.count_ones())
                            .min(*n);

                        // Perform trades from e_0 to e_1
                        let mut trade_indices =
                            trade_0_to_1.iter_ones().collect::<Vec<_>>();

                        trade_indices.shuffle(&mut rand::thread_rng());

                        for i in trade_indices.into_iter().take(num_trades) {
                            y_e_0.set(i, false);
                            y_e_1.set(i, true);
                        }

                        // Perform trades from e_1 to e_0
                        let mut trade_indices =
                            trade_1_to_0.iter_ones().collect::<Vec<_>>();

                        trade_indices.shuffle(&mut rand::thread_rng());

                        for i in trade_indices.into_iter().take(num_trades) {
                            y_e_1.set(i, false);
                            y_e_0.set(i, true);
                        }

                        // Calculate new event genes from time allocation
                        let c0 = EventGene::from_time_allocation(y_e_0);
                        let c1 = EventGene::from_time_allocation(y_e_1);

                        (c0, c1)
                    })
                    .unzip();

                (p0.into(), p1.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
