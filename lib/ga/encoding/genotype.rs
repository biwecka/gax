// Imports /////////////////////////////////////////////////////////////////////
use super::Context;

// Traits //////////////////////////////////////////////////////////////////////
/// The genotype is the encoded representation of a potential solution to a
/// problem. It is typically represented as a string of *genes* which might
/// be binary digits, integers or of other data types.
pub trait Genotype<Ctx: Context>: PartialEq + Eq + Clone {
    fn gnerate(size: usize, ctx: &Ctx) -> Vec<Self>;
}

////////////////////////////////////////////////////////////////////////////////
