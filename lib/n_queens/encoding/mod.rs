// Imports /////////////////////////////////////////////////////////////////////
use ndarray::Array2;
use rand::prelude::Distribution;

// Objective Value /////////////////////////////////////////////////////////////

/// Define cost as objective value for this encoding. The [`usize`] type by
/// default implements the [`ga::encoding::ObjectiveValue`] trait.
pub type Cost = usize;

// Context /////////////////////////////////////////////////////////////////////

/// Define a context for solving the problem.
pub struct Context {
    pub board_size: usize,
    pub random_position: rand::distributions::Uniform<usize>,
}

impl Context {
    pub fn init(board_size: usize) -> Self {
        let random_position = rand::distributions::Uniform::<usize>::new(0, board_size);

        Self {
            board_size,
            random_position,
        }
    }
}

// Genotype ////////////////////////////////////////////////////////////////////

/// The genotype encodes the positions of the queens on the chess board.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Genotype(pub Vec<usize>);

impl Genotype {
    pub fn gnerate(size: usize, ctx: &Context) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut chromosomes: Vec<Self> = vec![];

        for _ in 0..size {
            let mut chromosome = Vec::<usize>::with_capacity(ctx.board_size);
            for _ in 0..ctx.board_size {
                chromosome.push(ctx.random_position.sample(&mut rng));
            }

            chromosomes.push(Self(chromosome));
        }

        // Return
        chromosomes
    }
}

// Phenotype ///////////////////////////////////////////////////////////////////

/// The phenotype represents the applied genotype.
#[derive(Clone)]
pub struct Phenotype {
    board: Array2<u8>,
}

impl Phenotype {
    pub fn init(board_size: usize) -> Self {
        Self {
            board: Array2::default((board_size, board_size))
        }
    }

    pub fn derive(&self, chromosome: &Genotype) -> Self {
        let Phenotype { mut board, .. } = self.clone();

        for (i, pos) in chromosome.0.iter().enumerate() {
            board[[i, *pos]] = 1;
        }

        // Return
        Self { board }
    }

    pub fn evaluate(&self) -> Cost {
        let mut errors = 0;

        // Check rows
        // for row in self.board.rows() {
        //     errors += row.sum() - 1;
        // }

        // Check columns
        for col in self.board.columns() {
            let sum = col.sum();
            if sum > 1 {
                errors += sum - 1;
            }
        }

        // Diagonals ( \ and / )
        let size = self.board.shape()[0];

        let mut temp = 0;
        for i in 0..size {
            temp += self.board[[i, i]];
        }
        if temp > 1 {
            errors += temp - 1;
        }
        temp = 0;

        for i in 0..size {
            temp += self.board[[size - i - 1, i]];
        }
        if temp > 1 {
            errors += temp - 1;
        }
        // temp = 0;


        // Small diagonals ( \ )
        for offset in 1..size {
            let mut sum_lower = 0;
            for m in offset..size {
                for n in 0..(size - offset) {
                    sum_lower += self.board[[m, n]];
                }
            }
            if sum_lower > 1 {
                errors += sum_lower - 1;
            }

            let mut sum_upper = 0;
            for m in 0..(size - offset) {
                for n in offset..size {
                    sum_upper += self.board[[m, n]];
                }
            }
            if sum_upper > 1 {
                errors += sum_upper - 1;
            }
        }

        // Small diagonals ( / )
        for offset in 1..size {
            let mut sum_lower = 0;
            for m in (offset..size).rev() {
                for n in offset..size {
                    sum_lower += self.board[[m, n]];
                }
            }
            if sum_lower > 1 {
                errors += sum_lower - 1;
            }

            let mut sum_upper = 0;
            for m in (0..(size - offset)).rev() {
                for n in 0..(size - offset) {
                    sum_upper += self.board[[m, n]];
                }
            }
            if sum_upper > 1 {
                errors += sum_upper - 1;
            }
        }

        // Return
        errors.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
