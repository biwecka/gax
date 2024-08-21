// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;
use crate::{
    encoding::{Encoding, Context, Genotype, ObjectiveValue, Phenotype},
    operators::{Crossover, Mutation},
    process::{
        rejection::Rejection, replacement::Replacement, selection::Selection,
        termination::Termination
    },
};

// Parameter ///////////////////////////////////////////////////////////////////
pub struct Parameters<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
> {
    // General
    pub population_size: usize,
    pub crossover_rate: Option<f32>,
    pub mutation_rate: f32,

    // Operators + Process
    pub selection: Se,
    pub crossover: Cr,
    pub mutation: Mu,
    pub rejection: Re,
    pub replacement: Rp,
    pub termination: Te,

    // PhantomData
    objective_value: PhantomData<Ov>,
    context: PhantomData<Ctx>,
    genotype: PhantomData<Ge>,
    t: PhantomData<T>,
}


// Typestates //////////////////////////////////////////////////////////////////

// population_size -------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_PopulationSize(usize);
impl From<usize> for W_PopulationSize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_PopulationSize {}
impl TS_PopulationSize for () {}
impl TS_PopulationSize for W_PopulationSize {}

// crossover_rate --------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_CrossoverRate(Option<f32>);
impl From<Option<f32>> for W_CrossoverRate {
    fn from(value: Option<f32>) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_CrossoverRate {}
impl TS_CrossoverRate for () {}
impl TS_CrossoverRate for W_CrossoverRate {}

// mutation_rate ---------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_MutationRate(f32);
impl From<f32> for W_MutationRate {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_MutationRate {}
impl TS_MutationRate for () {}
impl TS_MutationRate for W_MutationRate {}


// selection -------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Selection<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    T,

    Se: Selection<Ov, Ctx, Ge, T>
>(Se, PhantomData<Ov>, PhantomData<Ctx>, PhantomData<Ge>, PhantomData<T>);

impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    T,

    Se: Selection<Ov, Ctx, Ge, T>
> From<Se> for W_Selection<Ov, Ctx, Ge, T, Se> {
    fn from(value: Se) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Selection {}
impl TS_Selection for () {}
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    T,

    Se: Selection<Ov, Ctx, Ge, T>
> TS_Selection for W_Selection<Ov, Ctx, Ge, T, Se> {}


// crossover -------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Crossover<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Cr: Crossover<Ctx, Ge>
>(Cr, PhantomData<Ctx>, PhantomData<Ge>);

impl<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Cr: Crossover<Ctx, Ge>
> From<Cr> for W_Crossover<Ctx, Ge, Cr> {
    fn from(value: Cr) -> Self {
        Self(value, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Crossover {}
impl TS_Crossover for () {}
impl<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Cr: Crossover<Ctx, Ge>,
> TS_Crossover for W_Crossover<Ctx, Ge, Cr> {}


// mutation --------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Mutation<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Mu: Mutation<Ctx, Ge>
>(Mu, PhantomData<Ctx>, PhantomData<Ge>);

impl<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Mu: Mutation<Ctx, Ge>
> From<Mu> for W_Mutation<Ctx, Ge, Mu> {
    fn from(value: Mu) -> Self {
        Self(value, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Mutation {}
impl TS_Mutation for () {}
impl<
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Mu: Mutation<Ctx, Ge>,
> TS_Mutation for W_Mutation<Ctx, Ge, Mu> {}



// rejection -------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Rejection<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Re: Rejection<Ov, Ctx, Ge>
>(Re, PhantomData<Ov>, PhantomData<Ctx>, PhantomData<Ge>);

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Re: Rejection<Ov, Ctx, Ge>
> From<Re> for W_Rejection<Ov, Ctx, Ge, Re> {
    fn from(value: Re) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Rejection {}
impl TS_Rejection for () {}
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Re: Rejection<Ov, Ctx, Ge>
> TS_Rejection for W_Rejection<Ov, Ctx, Ge, Re> {}


// replacement -----------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Replacement<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Rp: Replacement<(Ge, Ov)>
>(Rp, PhantomData<Ov>, PhantomData<Ctx>, PhantomData<Ge>);

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Rp: Replacement<(Ge, Ov)>
> From<Rp> for W_Replacement<Ov, Ctx, Ge, Rp> {
    fn from(value: Rp) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Replacement {}
impl TS_Replacement for () {}
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,

    Rp: Replacement<(Ge, Ov)>
> TS_Replacement for W_Replacement<Ov, Ctx, Ge, Rp> {}

// termination -----------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Termination<
    Ov: ObjectiveValue,

    Te: Termination<Ov>,
>(Te, PhantomData<Ov>);

impl<
Ov: ObjectiveValue,

Te: Termination<Ov>,
> From<Te> for W_Termination<Ov, Te> {
    fn from(value: Te) -> Self {
        Self(value, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Termination {}
impl TS_Termination for () {}
impl<
    Ov: ObjectiveValue,

    Te: Termination<Ov>,
> TS_Termination for W_Termination<Ov, Te> {}


// Builder /////////////////////////////////////////////////////////////////////
pub struct Builder<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> {
    // Data
    population_size: TsPs,
    crossover_rate: TsCrr,
    mutation_rate: TsMur,
    selection: TsSe,
    crossover: TsCr,
    mutation: TsMu,
    rejection: TsRe,
    replacement: TsRp,
    termination: TsTe,


    // PhantomData (from encoding)
    objective_value: PhantomData<Ov>,
    context: PhantomData<Ctx>,
    genotype: PhantomData<Ge>,
    phenotype: PhantomData<Ph>,
    t: PhantomData<T>,
}

impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    // TsPs: TS_PopulationSize,
    // TsSe: TS_Selection,
    // TsCr: TS_Crossover,
    // TsMu: TS_Mutation,
    // TsRe: TS_Rejection,
    // TsRp: TS_Replacement,
    // TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, (), (), (), (), (), (), (), (), ()> {
    pub fn for_encoding(_encoding: &Encoding<Ov, Ctx, Ge, Ph>) -> Self {
        Self {
            population_size: (),
            crossover_rate: (),
            mutation_rate: (),
            selection: (),
            crossover: (),
            mutation: (),
            rejection: (),
            replacement: (),
            termination: (),

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_population_size ---------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    // TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, (), TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe, TsRp, TsTe> {
    pub fn set_population_size(self, population_size: usize) -> Builder<
        Ov, Ctx, Ge, Ph, T, W_PopulationSize, TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe, TsRp, TsTe
    > {
        Builder {
            population_size: population_size.into(),
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_crossover_rate ----------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    // TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, (), TsMur, TsSe, TsCr, TsMu, TsRe, TsRp, TsTe> {
    pub fn set_crossover_rate(self, crossover_rate: Option<f32>) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, W_CrossoverRate, TsMur, TsSe, TsCr, TsMu, TsRe, TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: crossover_rate.into(),
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_mutation_rate -----------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    // TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, (), TsSe, TsCr, TsMu, TsRe, TsRp, TsTe> {
    pub fn set_mutation_rate(self, mutation_rate: f32) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, W_MutationRate, TsSe, TsCr, TsMu, TsRe, TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: mutation_rate.into(),
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_selection ---------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    // TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, (), TsCr, TsMu, TsRe, TsRp, TsTe> {
    pub fn set_selection<Se: Selection<Ov, Ctx, Ge, T>>(
        self,
        selection: Se
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, W_Selection<Ov, Ctx, Ge, T, Se>, TsCr, TsMu,
        TsRe, TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: selection.into(),
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_crossover ---------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    // TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, (), TsMu, TsRe, TsRp, TsTe> {
    pub fn set_crossover<Cr: Crossover<Ctx, Ge>>(
        self,
        crossover: Cr
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, W_Crossover<Ctx, Ge, Cr>, TsMu, TsRe,
        TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: crossover.into(),
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_mutation ----------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    // TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, (), TsRe, TsRp, TsTe> {
    pub fn set_mutation<Mu: Mutation<Ctx, Ge>>(
        self,
        mutation: Mu
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, W_Mutation<Ctx, Ge, Mu>, TsRe,
        TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: mutation.into(),
            rejection: self.rejection,
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_rejection ---------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    // TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu, (), TsRp, TsTe> {
    pub fn set_rejection<Re: Rejection<Ov, Ctx, Ge>>(
        self,
        rejection: Re
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu,
        W_Rejection<Ov, Ctx, Ge, Re>, TsRp, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: rejection.into(),
            replacement: self.replacement,
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_replacement -------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    // TsRp: TS_Replacement,
    TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe, (), TsTe> {
    pub fn set_replacement<Rp: Replacement<(Ge, Ov)>>(
        self,
        replacement: Rp
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe,
        W_Replacement<Ov, Ctx, Ge, Rp>, TsTe
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: replacement.into(),
            termination: self.termination,

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// set_termination -------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    TsPs: TS_PopulationSize,
    TsCrr: TS_CrossoverRate,
    TsMur: TS_MutationRate,
    TsSe: TS_Selection,
    TsCr: TS_Crossover,
    TsMu: TS_Mutation,
    TsRe: TS_Rejection,
    TsRp: TS_Replacement,
    // TsTe: TS_Termination,
> Builder<Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe, TsRp, ()> {
    pub fn set_termination<Te: Termination<Ov>>(
        self,
        termination: Te
    ) -> Builder<
        Ov, Ctx, Ge, Ph, T, TsPs, TsCrr, TsMur, TsSe, TsCr, TsMu, TsRe, TsRp,
        W_Termination<Ov, Te>
    > {
        Builder {
            population_size: self.population_size,
            crossover_rate: self.crossover_rate,
            mutation_rate: self.mutation_rate,
            selection: self.selection,
            crossover: self.crossover,
            mutation: self.mutation,
            rejection: self.rejection,
            replacement: self.replacement,
            termination: termination.into(),

            // PhantomData
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
            phenotype: PhantomData,
            t: PhantomData,
        }
    }
}

// build -----------------------------------------------------------------------
impl<
    Ov: ObjectiveValue + Into<T>,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    T,
    //
    // TsPs: TS_PopulationSize,
    // TsSe: TS_Selection,
    // TsCr: TS_Crossover,
    // TsMu: TS_Mutation,
    // TsRe: TS_Rejection,
    // TsRp: TS_Replacement,
    // TsTe: TS_Termination,

    Se: Selection<Ov, Ctx, Ge, T>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
> Builder<
    Ov, Ctx, Ge, Ph, T, W_PopulationSize, W_CrossoverRate, W_MutationRate,
    W_Selection<Ov, Ctx, Ge, T, Se>, W_Crossover<Ctx, Ge, Cr>,
    W_Mutation<Ctx, Ge, Mu>, W_Rejection<Ov, Ctx, Ge, Re>,
    W_Replacement<Ov, Ctx, Ge, Rp>, W_Termination<Ov, Te>
> {
    pub fn build(self) -> Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te> {
        Parameters {
            population_size: self.population_size.0,
            crossover_rate: self.crossover_rate.0,
            mutation_rate: self.mutation_rate.0,
            selection: self.selection.0,
            crossover: self.crossover.0,
            mutation: self.mutation.0,
            rejection: self.rejection.0,
            replacement: self.replacement.0,
            termination: self.termination.0,

            // PhantomData
            context: PhantomData,
            objective_value: PhantomData,
            genotype: PhantomData,
            t: PhantomData
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
