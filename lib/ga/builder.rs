// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;
use crate::{
    dynamics::{Dynamic, Dynamics}, encoding::{Context, Encoding, Genotype, ObjectiveValue, Phenotype}, operators::{Crossover, Mutation}, parameters::Parameters, process::{
        rejection::Rejection, replacement::Replacement, selection::Selection, termination::Termination
    }, Algorithm
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
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    TsEn: TS_Encoding,
    TsPa: TS_Parameters,
    TsDy: TS_Dynamics,
> {
    encoding: TsEn,
    parameters: TsPa,
    dynamics: TsDy,

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
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    // TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> Builder<
    Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), (), (),
> {
    pub fn new() -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), (), (),
    > {
        Builder {
            encoding: (),
            parameters: (),
            dynamics: (),

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
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    // TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> std::default::Default for Builder<
    Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), (), (),
> {
    fn default() -> Self {
        Self::new()
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
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    // TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
> Builder<
    Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, W_Encoding<Ov, Ctx, Ge, Ph>,
    W_Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    W_Dynamics<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te, Dy>
> {
    pub fn build(
        self
    ) -> Algorithm<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, Dy> {
        Algorithm {
            encoding: self.encoding.0,
            params: self.parameters.0,
            dynamics: self.dynamics.0,

            #[cfg(feature = "cache")]
            cache: HashMap::<Ge, Ov>::new(),

            #[cfg(feature = "rerun_logger")]
            rerun_logger: crate::tools::rerun_logger::RerunLogger::connect("ga"),

            #[cfg(feature = "log_runtimes")]
            runtime_reference: std::time::Instant::now(),

            #[cfg(feature = "log_runtimes")]
            runtimes: vec![],
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
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    // TsEn: TS_Encoding,
    TsPa: TS_Parameters,
    TsDy: TS_Dynamics,
> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, (), TsPa, TsDy> {
    pub fn set_encoding(
        self,
        encoding: Encoding<Ov, Ctx, Ge, Ph>
    ) -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, W_Encoding<Ov, Ctx, Ge, Ph>,
        TsPa, TsDy
    > {
        Builder {
            encoding: encoding.into(),
            parameters: self.parameters,
            dynamics: self.dynamics,

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
    // Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    //
    TsEn: TS_Encoding,
    // TsPa: TS_Parameters,
    TsDy: TS_Dynamics,
> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn, (), TsDy> {
    pub fn set_parameters(
        self,
        parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
    ) -> Builder<
        Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn,
        W_Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>, TsDy
    > {
        Builder {
            encoding: self.encoding,
            parameters: parameters.into(),
            dynamics: self.dynamics,

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

// set_dynamics ----------------------------------------------------------------
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
    TsPa: TS_Parameters,
    // TsDy: TS_Dynamics,
> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn, TsPa, ()> {
    pub fn set_dynamics<Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,>(
        self,
        dynamics: Option<Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>>,
    ) -> Builder<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, TsEn, TsPa, W_Dynamics<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te, Dy>> {
        Builder {
            encoding: self.encoding,
            parameters: self.parameters,
            dynamics: dynamics.into(),

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

// encoding --------------------------------------------------------------------
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


// dynamics --------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Dynamics<
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
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
>(
    Option<Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>>,
    PhantomData<Ov>,
    PhantomData<Ctx>,
    PhantomData<Ge>,
    PhantomData<Cr>,
    PhantomData<Mu>,
    PhantomData<T>,
    PhantomData<Se>,
    PhantomData<Re>,
    PhantomData<Rp>,
    PhantomData<Te>,
);

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
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
> From<Option<Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>>> for W_Dynamics<
    Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te, Dy
> {
    fn from(value: Option<Dynamics<Ov, Ctx, Ge, T, Se, Cr, Mu, Re, Rp, Te, Dy>>) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData, )
    }
}

#[allow(non_camel_case_types)] pub trait TS_Dynamics {}
impl TS_Dynamics for () {}
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
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
>TS_Dynamics for W_Dynamics<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te, Dy> {}

////////////////////////////////////////////////////////////////////////////////
