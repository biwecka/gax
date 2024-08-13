// Imports /////////////////////////////////////////////////////////////////////
use super::{
    CrossoverStrategies, MutationStrategies, OffspringRejectionStrategies,
    Parameters, SelectionStrategies, TerminationStrategies,
};
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    stats::Stats,
};

// Traits //////////////////////////////////////////////////////////////////////
// pub trait AdaptionMetrics<Ov: ObjectiveValue, St: Stats<Ov>>: From<St> {}

pub trait Adaption {
    fn exec<
        // Ov: ObjectiveValue,
        Ct: Context,
        Ge: Genotype<Ct>,
        // Se: SelectionStrategies,
        // Cx: CrossoverStrategies,
        // Mu: MutationStrategies,
        // Of: OffspringRejectionStrategies,
        // Te: TerminationStrategies<Ov>,
        // Ad: Adaption,
        Ov: ObjectiveValue,
        St: Stats<Ov, Ct, Ge>,
        // Am: AdaptionMetrics<Ov, St>
    >(
        &self,
        stats: &mut St, //Parameters<Ov, Ct, Ge, Se, Cx, Mu, Of, Te, Ad>,
    );
}

////////////////////////////////////////////////////////////////////////////////
