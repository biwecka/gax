use std::{collections::HashMap, ops::AddAssign};

// Imports /////////////////////////////////////////////////////////////////////
use ndarray::Array2;
use rand::prelude::Distribution;

// Objective Value /////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cost(usize);
impl ga::encoding::ObjectiveValue for Cost {
    fn calc_average(values: &[Self]) -> f32 {
        let sum: usize = values.iter().map(|x| x.0).sum();
        sum as f32 / values.len() as f32
    }

    fn calc_distribution(values: &[Self]) -> Vec<usize> {
        // Calc worst objective value
        let max = values.iter().map(|x| x.0).max().unwrap();

        // Initialize array
        let mut arr = vec![0; max + 1];

        // Evaluate distribution
        for val in values {
            assert!(val.0 < arr.len());
            arr[val.0] += 1;
        }

        // Return
        arr
    }

    fn to_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for Cost {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Cost> for usize {
    fn from(value: Cost) -> Self {
        value.0
    }
}

// Context /////////////////////////////////////////////////////////////////////
pub struct Context {
    pub board_size: usize,
    pub random_position: rand::distributions::Uniform<usize>,
}
impl ga::encoding::Context for Context {}
impl Context {
    pub fn init(board_size: usize) -> Self {
        let random_position =
            rand::distributions::Uniform::<usize>::new(0, board_size);

        Self { board_size, random_position }
    }
}

// Genotype ////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(Vec<usize>);

impl Chromosome {
    pub fn as_slice(&self) -> &[usize] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        self.0.as_mut_slice()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }
}

impl From<Vec<&usize>> for Chromosome {
    fn from(value: Vec<&usize>) -> Self {
        Self(value.into_iter().cloned().collect())
    }
}

impl ga::encoding::Genotype<Context> for Chromosome {
    fn generate(amount: usize, ctx: &Context) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut chromosomes: Vec<Self> = vec![];

        for _ in 0..amount {
            let mut chromosome = Vec::<usize>::with_capacity(ctx.board_size);
            for _ in 0..ctx.board_size {
                chromosome.push(ctx.random_position.sample(&mut rng));
            }

            chromosomes.push(Self(chromosome));
        }

        // Return
        chromosomes
    }

    fn calc_diversity<Ov: ga::encoding::ObjectiveValue>(
        population: &[(Self, Ov)],
    ) -> Vec<usize> {
        let mut map = HashMap::<(Self, Ov), usize>::new();
        for i in population {
            map.entry(i.clone()).or_default().add_assign(1);
        }

        let mut arr: Vec<((Self, Ov), usize)> = map.into_iter().collect();
        arr.sort_by_key(|((_, x), _)| x.clone());

        // Return
        arr.into_iter().map(|(_, x)| x).collect()
    }
}

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    board: Array2<u8>,
}

impl Phenotype {
    pub fn blueprint(board_size: usize) -> Self {
        Self { board: Array2::default((board_size, board_size)) }
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromosome: &Chromosome, _ctx: &Context) -> Self {
        let Phenotype { mut board, .. } = self.clone();

        for (i, pos) in chromosome.iter().enumerate() {
            board[[i, *pos]] = 1;
        }

        // Return
        Self { board }
    }

    fn evaluate(&self, _ctx: &Context) -> Cost {
        let mut errors: u8 = 0;

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

            let mut m = offset..size;
            let mut n = 0..(size - offset);

            for (m, n) in m.zip(n) {
                sum_lower += self.board[[m, n]];
            }

            if sum_lower > 1 {
                errors += sum_lower - 1;
            }

            let mut sum_upper = 0;
            m = 0..(size - offset);
            n = offset..size;
            for (m, n) in m.zip(n) {
                sum_upper += self.board[[m, n]];
            }
            if sum_upper > 1 {
                errors += sum_upper - 1;
            }
        }

        // Small diagonals ( / )
        for offset in 1..size {
            let mut sum_lower = 0;
            let mut m = (offset..size).rev();
            let mut n = offset..size;

            for (m, n) in m.zip(n) {
                sum_lower += self.board[[m, n]];
            }

            if sum_lower > 1 {
                errors += sum_lower - 1;
            }

            let mut sum_upper = 0;
            m = (0..(size - offset)).rev();
            n = 0..(size - offset);
            for (m, n) in m.zip(n) {
                sum_upper += self.board[[m, n]];
            }

            if sum_upper > 1 {
                errors += sum_upper - 1;
            }
        }

        // Return
        Cost(errors as usize)
    }
}

////////////////////////////////////////////////////////////////////////////////
