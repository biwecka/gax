use std::marker::PhantomData;

// Imports /////////////////////////////////////////////////////////////////////
use crate::{encoding::{Context, Genotype, ObjectiveValue}, operators::{Crossover, Mutation}, parameters::Parameters, process::{rejection::Rejection, replacement::Replacement, selection::Selection, termination::Termination}};

use super::Dynamic;

// Dynamics ////////////////////////////////////////////////////////////////////
pub struct Dynamics<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> {
    pub list: Vec<Dy>,

    // PhantomData
    objective_value: std::marker::PhantomData<Ov>,
    context: std::marker::PhantomData<Ctx>,
    genotype: std::marker::PhantomData<Ge>,
    // phenotype: std::marker::PhantomData<Ph>,
    t: std::marker::PhantomData<T>,
    selection: std::marker::PhantomData<Se>,
    crossover: std::marker::PhantomData<Cr>,
    mutation: std::marker::PhantomData<Mu>,
    rejection: std::marker::PhantomData<Re>,
    replacement: std::marker::PhantomData<Rp>,
    termination: std::marker::PhantomData<Te>,
}

// Typestates //////////////////////////////////////////////////////////////////

// dynamics --------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Dynamics<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
>(
    Vec<Dy>, PhantomData<Ov>, PhantomData<Ctx>, PhantomData<Ge>, PhantomData<T>,
    PhantomData<Se>, PhantomData<Cr>, PhantomData<Mu>, PhantomData<Re>,
    PhantomData<Rp>, PhantomData<Te>
);
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> From<Vec<Dy>> for W_Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy> {
    fn from(value: Vec<Dy>) -> Self {
        Self(
            value,
            PhantomData, PhantomData, PhantomData, PhantomData, PhantomData,
            PhantomData, PhantomData, PhantomData, PhantomData, PhantomData
        )
    }
}

#[allow(non_camel_case_types)] pub trait TS_Dynamics {}
impl TS_Dynamics for () {}
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> TS_Dynamics for W_Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy> {}


// Builder /////////////////////////////////////////////////////////////////////
/// TODO: docs
pub struct Builder<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,

    TsDy: TS_Dynamics,
> {
    list: TsDy,

    // PhantomData
    objective_value: std::marker::PhantomData<Ov>,
    context: std::marker::PhantomData<Ctx>,
    genotype: std::marker::PhantomData<Ge>,
    // phenotype: std::marker::PhantomData<Ph>,
    t: std::marker::PhantomData<T>,
    selection: std::marker::PhantomData<Se>,
    crossover: std::marker::PhantomData<Cr>,
    mutation: std::marker::PhantomData<Mu>,
    rejection: std::marker::PhantomData<Re>,
    replacement: std::marker::PhantomData<Rp>,
    termination: std::marker::PhantomData<Te>,
}

impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> Builder<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, ()> {
    pub fn for_parameters(_parameters: &Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>) -> Self {
        Self {
            list: (),

            // PhantomData
            objective_value: std::marker::PhantomData,
            context: std::marker::PhantomData,
            genotype: std::marker::PhantomData,
            // phenotype: std::marker::PhantomData,
            t: std::marker::PhantomData,
            selection: std::marker::PhantomData,
            crossover: std::marker::PhantomData,
            mutation: std::marker::PhantomData,
            rejection: std::marker::PhantomData,
            replacement: std::marker::PhantomData,
            termination: std::marker::PhantomData,
        }
    }
}

// set_dynamics ----------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
> Builder<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, ()> {
    pub fn set<Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>>(self, dynamics: Vec<Dy>) -> Builder<
    Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, W_Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>
    > {
        Builder {
            list: dynamics.into(),

            // PhantomData
            objective_value: std::marker::PhantomData,
            context: std::marker::PhantomData,
            genotype: std::marker::PhantomData,
            // phenotype: std::marker::PhantomData,
            t: std::marker::PhantomData,
            selection: std::marker::PhantomData,
            crossover: std::marker::PhantomData,
            mutation: std::marker::PhantomData,
            rejection: std::marker::PhantomData,
            replacement: std::marker::PhantomData,
            termination: std::marker::PhantomData,
        }
    }
}


// build -----------------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    // Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> Builder<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, W_Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>> {

    pub fn build(self) -> Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy> {
        Dynamics {
            list: self.list.0,

            // PhantomData
            objective_value: std::marker::PhantomData,
            context: std::marker::PhantomData,
            genotype: std::marker::PhantomData,
            // phenotype: std::marker::PhantomData,
            t: std::marker::PhantomData,
            selection: std::marker::PhantomData,
            crossover: std::marker::PhantomData,
            mutation: std::marker::PhantomData,
            rejection: std::marker::PhantomData,
            replacement: std::marker::PhantomData,
            termination: std::marker::PhantomData,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
