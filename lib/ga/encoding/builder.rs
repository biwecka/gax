// Imports /////////////////////////////////////////////////////////////////////
use std::marker::PhantomData;
use super::{Context, Genotype, ObjectiveValue, Phenotype};

// Encoding ////////////////////////////////////////////////////////////////////
pub struct Encoding<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> {
    pub context: Ctx,
    pub phenotype: Ph,

    // PhantomData
    objective_value: PhantomData<Ov>,
    genotype: PhantomData<Ge>,
}

// Typestates //////////////////////////////////////////////////////////////////

// context ---------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Context<Ctx: Context>(Ctx);
impl<Ctx: Context> From<Ctx> for W_Context<Ctx> {
    fn from(value: Ctx) -> Self {
        Self(value)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Context {}
impl TS_Context for () {}
impl<Ctx: Context> TS_Context for W_Context<Ctx> {}


// phenotype -------------------------------------------------------------------
#[allow(non_camel_case_types)] pub struct W_Phenotype<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
>(Ph, PhantomData<Ov>, PhantomData<Ctx>, PhantomData<Ge>);

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> From<Ph> for W_Phenotype<Ov, Ctx, Ge, Ph> {
    fn from(value: Ph) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData)
    }
}

#[allow(non_camel_case_types)] pub trait TS_Phenotype {}
impl TS_Phenotype for () {}
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
> TS_Phenotype for W_Phenotype<Ov, Ctx, Ge, Ph> {}


// Builder /////////////////////////////////////////////////////////////////////
pub struct Builder<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    //
    TsCtx: TS_Context,
    TsPh: TS_Phenotype,
> {
    pub context: TsCtx,
    pub phenotype: TsPh,

    // PhantomData
    _objective_value: PhantomData<Ov>,
    _context: PhantomData<Ctx>,
    _genotype: PhantomData<Ge>,
    _phenotype: PhantomData<Ph>
}

impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    //
    // TsCtx: TS_Context,
    // TsPh: TS_Phenotype,
> Builder<Ov, Ctx, Ge, Ph, (), ()> {
    pub fn new() -> Self {
        Self {
            context: (),
            phenotype: (),

            // PhantomData
            _objective_value: PhantomData,
            _context: PhantomData,
            _genotype: PhantomData,
            _phenotype: PhantomData,
        }
    }
}

// set_context -----------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    //
    // TsCtx: TS_Context,
    TsPh: TS_Phenotype,
> Builder<Ov, Ctx, Ge, Ph, (), TsPh> {
    pub fn set_context(self, context: Ctx) -> Builder<Ov, Ctx, Ge, Ph, W_Context<Ctx>, TsPh> {
        Builder {
            context: context.into(),
            phenotype: self.phenotype,

            // PhantomData
            _objective_value: PhantomData,
            _context: PhantomData,
            _genotype: PhantomData,
            _phenotype: PhantomData,
        }
    }
}

// set_phenotype ---------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    //
    TsCtx: TS_Context,
    // TsPh: TS_Phenotype,
> Builder<Ov, Ctx, Ge, Ph, TsCtx, ()> {
    pub fn set_phenotype/*<Ph: Phenotype<Ov, Ctx, Ge>>*/(self, phenotype: Ph) -> Builder<Ov, Ctx, Ge, Ph, TsCtx, W_Phenotype<Ov, Ctx, Ge, Ph>> {
        Builder {
            context: self.context,
            phenotype: phenotype.into(),

            // PhantomData
            _objective_value: PhantomData,
            _context: PhantomData,
            _genotype: PhantomData,
            _phenotype: PhantomData,
        }
    }
}

// build -----------------------------------------------------------------------
impl<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    //
    // TsCtx: TS_Context,
    // TsPh: TS_Phenotype,
> Builder<Ov, Ctx, Ge, Ph, W_Context<Ctx>, W_Phenotype<Ov, Ctx, Ge, Ph>> {
    pub fn build(self) -> Encoding<Ov, Ctx, Ge, Ph> {
        Encoding {
            context: self.context.0,
            phenotype: self.phenotype.0,

            // PhantomData
            objective_value: self._objective_value,
            genotype: self._genotype,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
