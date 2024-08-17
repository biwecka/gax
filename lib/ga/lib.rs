// Modules /////////////////////////////////////////////////////////////////////
pub mod encoding;
pub mod operators;
pub mod process;
pub mod utils;
#[rustfmt::skip] pub mod parameters;
#[rustfmt::skip] mod builder;

// Re-Exports //////////////////////////////////////////////////////////////////
pub use builder::*;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Encoding, Genotype, ObjectiveValue, Phenotype};
use operators::{Crossover, Mutation};
use parameters::Parameters;
use process::{
    rejection::Rejection, replacement::Replacement, selection::Selection,
    termination::Termination,
};

// #[cfg(feature = "log")]
// use rerun::Archetype;

// Algorithm /////////////////////////////////////////////////////////////////////

pub struct Algorithm<
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
> {
    encoding: Encoding<Ov, Ctx, Ge, Ph>,
    params: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
}




// Builder /////////////////////////////////////////////////////////////////////

// pub struct Builder {}
// impl Builder {
//     pub fn new() -> Self {
//         Builder {}
//     }

//     pub fn set_encoding<
//         Ov: ObjectiveValue,
//         Ctx: Context,
//         Ge: Genotype<Ctx>,
//         Ph: Phenotype<Ov, Ctx, Ge>,
//     >(
//         self,
//         encoding: Encoding<Ov, Ctx, Ge, Ph>,
//     ) -> BuilderWithEncoding<Ov, Ctx, Ge, Ph> {
//         BuilderWithEncoding { encoding }
//     }
// }

// pub struct BuilderWithEncoding<
//     Ov: ObjectiveValue,
//     Ctx: Context,
//     Ge: Genotype<Ctx>,
//     Ph: Phenotype<Ov, Ctx, Ge>,
// > {
//     encoding: Encoding<Ov, Ctx, Ge, Ph>,
// }

// impl<
//         Ov: ObjectiveValue,
//         Ctx: Context,
//         Ge: Genotype<Ctx>,
//         Ph: Phenotype<Ov, Ctx, Ge>,
//     > BuilderWithEncoding<Ov, Ctx, Ge, Ph>
// {
//     pub fn set_parameters<
//         Cr: Crossover<Ctx, Ge>,
//         Mu: Mutation<Ctx, Ge>,
//         T: From<Ov>,
//         Se: Selection<Ov, Ctx, Ge, T>,
//         Re: Rejection<Ov, Ctx, Ge>,
//         Rp: Replacement<(Ge, Ov)>,
//         Te: Termination<Ov>,
//     >(
//         self,
//         parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
//     ) -> BuilderWithEncodingAndParameters<
//         Ov,
//         Ctx,
//         Ge,
//         Ph,
//         Cr,
//         Mu,
//         T,
//         Se,
//         Re,
//         Rp,
//         Te,
//     > {
//         BuilderWithEncodingAndParameters { encoding: self.encoding, parameters }
//     }
// }

// pub struct BuilderWithEncodingAndParameters<
//     Ov: ObjectiveValue,
//     Ctx: Context,
//     Ge: Genotype<Ctx>,
//     Ph: Phenotype<Ov, Ctx, Ge>,
//     Cr: Crossover<Ctx, Ge>,
//     Mu: Mutation<Ctx, Ge>,
//     T: From<Ov>,
//     Se: Selection<Ov, Ctx, Ge, T>,
//     Re: Rejection<Ov, Ctx, Ge>,
//     Rp: Replacement<(Ge, Ov)>,
//     Te: Termination<Ov>,
// > {
//     encoding: Encoding<Ov, Ctx, Ge, Ph>,
//     parameters: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
// }

// impl<
//         Ov: ObjectiveValue,
//         Ctx: Context,
//         Ge: Genotype<Ctx>,
//         Ph: Phenotype<Ov, Ctx, Ge>,
//         Cr: Crossover<Ctx, Ge>,
//         Mu: Mutation<Ctx, Ge>,
//         T: From<Ov>,
//         Se: Selection<Ov, Ctx, Ge, T>,
//         Re: Rejection<Ov, Ctx, Ge>,
//         Rp: Replacement<(Ge, Ov)>,
//         Te: Termination<Ov>,
//     >
//     BuilderWithEncodingAndParameters<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te>
// {
//     pub fn build(
//         self,
//     ) -> Algorithm<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te> {
//         Algorithm { encoding: self.encoding, params: self.parameters }
//     }
// }

////////////////////////////////////////////////////////////////////////////////
