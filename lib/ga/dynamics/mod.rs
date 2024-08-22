// Modules /////////////////////////////////////////////////////////////////////
#[rustfmt::skip] mod builder;
pub use builder::*;

// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    operators::{Crossover, Mutation},
    parameters::Parameters,
    runtime_data::RuntimeData,
};

use crate::process::{
    rejection::Rejection, replacement::Replacement, selection::Selection,
    termination::Termination,
};

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
pub trait Dynamic<
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
>: Send + Sync
{
    fn setup(
        &self,
        // Output
        rtd: &mut RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        parameters: &mut Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        context: &mut Ctx,
    );

    fn exec(
        &self,
        // Input
        rtd: &RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,

        // "Output"
        parameters: &mut Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        context: &mut Ctx,
    );
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
        Rp: Replacement<(Ge, Ov)>,
        Te: Termination<Ov>,
    > Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te> for ()
{
    #[inline(always)]
    fn setup(
        &self,
        // Output
        _rtd: &mut RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        _parameters: &mut Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        _context: &mut Ctx,
    ) {
    }

    #[inline(always)]
    fn exec(
        &self,
        // Input
        _rtd: &RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,

        // "Output"
        _parameters: &mut Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
        _context: &mut Ctx,
    ) {
    }
}

////////////////////////////////////////////////////////////////////////////////
