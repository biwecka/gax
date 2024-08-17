
use ndarray::Array2;
use rand::prelude::Distribution;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cost(usize);
impl ga::encoding::ObjectiveValue for Cost {}

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


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chromosome(Vec<usize>);

impl Chromosome {
    pub fn as_slice(&self) -> &[usize] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        self.0.as_mut_slice()
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
}


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
    fn derive(&self, chromsome: &Chromosome) -> Self {
        todo!()
    }

    fn evaluate(&self) -> Cost {
        todo!()
    }
}

