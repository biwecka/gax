// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype};

// Crossover ///////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// crossover methods.
/// As the crossover operation depends on the genotype, no default
/// implementation can be provided. Although the [`crate::utils::crossover`]
/// provides some helpful implementations for commonly used datatypes.
///
/// You might notice, that the parameters to the exec function only contain
/// the "parent" chromosomes, without any parameter for the crossover rate.
/// This is because the crossover rate should be defined with the crossover
/// method and should therefore be part of the crossover method (e.g. a
/// parameter for an enum variant which represents one crossover method).
pub trait Crossover<Ctx: Context, Ge: Genotype<Ctx>> {
    fn exec(&self, parent_0: &Ge, parent_1: &Ge, context: &Ctx) -> (Ge, Ge);
}

// Mutation ////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// mutation methods. Just like the crossover operathor, the mutation operator
/// also depends on the genotype, therefore no default implementation can be
/// provided. But the [`crate::utils::crossover`] provides some helpful
/// implementations for commonly used datatypes.
///
/// You might notice, that the parameters to the exec function only the
/// chromosome, without any parameter for the crossover rate. This is because
/// the crossover rate should be defined with the crossover method and should
/// therefore be part of the crossover method (e.g. a parameter for an enum
/// variant which represents one crossover method).
pub trait Mutation<Ctx: Context, Ge: Genotype<Ctx>> {
    fn exec(&self, chromosome: &mut Ge, context: &Ctx);
}

////////////////////////////////////////////////////////////////////////////////
