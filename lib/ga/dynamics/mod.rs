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

#[cfg(feature = "log_dynamics")]
use crate::tools::rerun_logger::RerunLogger;

// Crossover ///////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// self-parameterization methods.
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

        // Logger
        #[cfg(feature = "log_dynamics")] rerun_logger: &RerunLogger,
    );

    fn identifier(&self) -> String;
}


/// Implementation of the `Dynamic` trait for the unit type `()`. This allows
/// the user to specify no dynamic when building the algorithm, by using
/// the following line:
/// ```rust ,ignore
/// let alg = ga::Builder::new()
///     .set_encoding(encoding)
///     .set_parameters(parameters)
///     .set_dynamics::<()>(None)       // <-- this line
///     .set_custom_logger::<()>(None)
///     .build();
/// ```
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

        // Logger
        #[cfg(feature = "log_dynamics")] _rerun_logger: &RerunLogger,
    ) {
    }

    fn identifier(&self) -> String {
        "".into()
    }
}

////////////////////////////////////////////////////////////////////////////////
