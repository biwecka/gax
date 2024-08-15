// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;

// mod main_plotters_examples;

////////////////////////////////////////////////////////////////////////////////

pub struct Algorithm<M, T>
where
    M: Multiplier<T>,
{
    multiplier: M,
    t: PhantomData<T>,
}

impl<M, T> Algorithm<M, T>
where
    M: Multiplier<T>,
{
    pub fn init(multiplier: M) -> Self {
        Self { multiplier, t: PhantomData }
    }

    pub fn run(&self) -> Result<(), ()> {
        loop {
            let x = 10.;
            print!("x={x}   --- mul -->   ");
            let y = self.multiplier.multiply(x);
            println!("y={y}");

            std::thread::sleep(std::time::Duration::from_secs(2));
        }

        Ok(())
    }
}

pub trait Multiplier<T> {
    fn multiply(&self, a: f32) -> f32;
    fn x(t: T);
}

pub enum Mul {
    M2,
    M4,
}

impl Multiplier<i32> for Mul {
    fn multiply(&self, a: f32) -> f32 {
        match self {
            Self::M2 => a * 2.,
            Self::M4 => a * 4.,
        }
    }

    fn x(t: i32) {}
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    // let alg = Algorithm::init(
    //     Mul::M4
    // );
    // let _ = alg.run();

    n_queens::run();
}

////////////////////////////////////////////////////////////////////////////////
