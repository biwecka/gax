// Modules /////////////////////////////////////////////////////////////////////
pub mod encoding;
pub mod operators;
pub mod process;
pub mod utils;

// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;
use encoding::{Context, Genotype, ObjectiveValue, Phenotype};
use operators::{Crossover, Mutation};
use process::{rejection::Rejection, selection::Selection};

// #[cfg(feature = "log")]
// use rerun::Archetype;

// Algorithm /////////////////////////////////////////////////////////////////////

pub struct Algorithm {}
impl Algorithm {
    pub fn builder() -> Builder {
        Builder {}
    }
}

/// TODO: builder pattern for parameters
pub struct Parameters<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,

    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
> {
    // General
    pub population_size: usize,

    // Operators
    pub crossover: Cr,
    pub mutation: Mu,

    // Process
    pub selection: Se,
    pub rejection: Re,

    // PhantomData
    pub t: PhantomData<T>,
    pub objective_value: PhantomData<Ov>,
    pub context: PhantomData<Ctx>,
    pub genotype: PhantomData<Ge>,
}

impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,

    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
> Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re> {
    #[allow(unused)]
    fn x(&self) {
        self.selection.exec(1, &vec![]);
    }
}


// Builder /////////////////////////////////////////////////////////////////////
pub struct Builder {}
impl Builder {
    pub fn set_encoding<
        Ov: ObjectiveValue,
        Ctx: Context,
        Ge: Genotype<Ctx>,
        Ph: Phenotype<Ov, Ctx, Ge>,
    >(
        self,
        context: Ctx,
        phenotype: Ph,
    ) -> BuilderWithEncoding<Ov, Ctx, Ge, Ph> {
        BuilderWithEncoding {
            objective_value: PhantomData,
            context,
            genotype: PhantomData,
            phenotype,
        }
    }
}

pub struct BuilderWithEncoding<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> {
    objective_value: PhantomData<Ov>,
    #[allow(unused)] context: Ctx,
    genotype: PhantomData<Ge>,
    phenotype: Ph,
}

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> BuilderWithEncoding<Ov, Ctx, Ge, Ph> {
    pub fn set_parameters<
        Cr: Crossover<Ctx, Ge>,
        Mu: Mutation<Ctx, Ge>,
        T : From<Ov>,
        Se: Selection<Ov, Ctx, Ge, T>,
        Re: Rejection<Ov, Ctx, Ge>,
    >(
        self,
        parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re>
    ) -> BuilderWithEncodingAndParameters<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re>{
        BuilderWithEncodingAndParameters {
            objective_value: self.objective_value,
            context: self.context,
            genotype: self.genotype,
            phenotype: self.phenotype,

            parameters
        }
    }
}



pub struct BuilderWithEncodingAndParameters<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
> {
    objective_value: PhantomData<Ov>,
    #[allow(unused)] context: Ctx,
    genotype: PhantomData<Ge>,
    phenotype: Ph,
    parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re>,
}


////////////////////////////////////////////////////////////////////////////////
