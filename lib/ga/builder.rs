// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;
use crate::{
    encoding::{Context, Encoding, Genotype, ObjectiveValue, Phenotype},
    operators::{Crossover, Mutation},
    parameters::Parameters,
    process::{
        rejection::Rejection, replacement::Replacement, selection::Selection,
        termination::Termination,
    }, Algorithm,
};

// Builder /////////////////////////////////////////////////////////////////////

/// Genetic algorithm builder.
pub struct Builder<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    //
    TsEn: TS_Encoding,
    TsPa: TS_Parameters,
> {
    encoding: TsEn,
    parameters: TsPa,

    // PhantomData
    objective_value: PhantomData<Ov>,
    context: PhantomData<Ctx>,
    genotype: PhantomData<Ge>,
    phenotype: PhantomData<Ph>,
    crossover: PhantomData<Cr>,
    mutation: PhantomData<Mu>,
    t: PhantomData<T>,
    selection: PhantomData<Se>,
    rejection: PhantomData<Re>,
    replacement: PhantomData<Rp>,
    termination: PhantomData<Te>,
}

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    //
    // TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> Builder<
    Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), ()
> {
    pub fn new() -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), ()
    > {
        Builder{
            encoding: (),
            parameters: (),

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            crossover: PhantomData,
            mutation: PhantomData,
            t: PhantomData,
            selection: PhantomData,
            rejection: PhantomData,
            replacement: PhantomData,
            termination: PhantomData,
        }
    }
}

// build -----------------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    //
    // TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> Builder<
    Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, W_Encoding<Ov, Ctx, Ge, Ph>,
    W_Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
> {
    pub fn build(
        self
    ) -> Algorithm<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te> {
        Algorithm {
            encoding: self.encoding.0,
            params: self.parameters.0,
        }
    }
}

// set_encoding ----------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    //
    // TsEn: TS_Encoding,
    TsPa: TS_Parameters,
> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), TsPa> {
    pub fn set_encoding(
        self,
        encoding: Encoding<Ov, Ctx, Ge, Ph>
    ) -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, W_Encoding<Ov, Ctx, Ge, Ph>,
        TsPa
    > {
        Builder {
            encoding: encoding.into(),
            parameters: self.parameters,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            crossover: PhantomData,
            mutation: PhantomData,
            t: PhantomData,
            selection: PhantomData,
            rejection: PhantomData,
            replacement: PhantomData,
            termination: PhantomData,
        }
    }
}

// set_parameters --------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    //
    TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn, ()> {
    pub fn set_parameters(
        self,
        parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
    ) -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn,
        W_Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
    > {
        Builder {
            encoding: self.encoding,
            parameters: parameters.into(),

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            crossover: PhantomData,
            mutation: PhantomData,
            t: PhantomData,
            selection: PhantomData,
            rejection: PhantomData,
            replacement: PhantomData,
            termination: PhantomData,
        }
    }
}



// Typestates //////////////////////////////////////////////////////////////////

// population_size -------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Encoding<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    // Cr: Crossover<Ctx, Ge>,
    // Mu: Mutation<Ctx, Ge>,
    // T: From<Ov>,
    // Se: Selection<Ov, Ctx, Ge, T>,
    // Re: Rejection<Ov, Ctx, Ge>,
    // Rp: Replacement<(Ge, Ov)>,
    // Te: Termination<Ov>,
>(Encoding<Ov, Ctx, Ge, Ph>);
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> From<Encoding<Ov, Ctx, Ge, Ph>> for W_Encoding<Ov, Ctx, Ge, Ph> {
    fn from(value: Encoding<Ov, Ctx, Ge, Ph>) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Encoding {}
impl TS_Encoding for () {}
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
>TS_Encoding for W_Encoding<Ov, Ctx, Ge, Ph> {}


// parameters ------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Parameters<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
>(Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>);
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
> From<Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>> for W_Parameters<
    Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te
> {
    fn from(value: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Parameters {}
impl TS_Parameters for () {}
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
>TS_Parameters for W_Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te> {}

////////////////////////////////////////////////////////////////////////////////
