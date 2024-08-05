/// The chromosome stores a vector of event indices, where the order of these
/// indices in the chromosome defines the order in which the corresponding
/// events are scheduled. The values of the vector fields are event indices.
///
/// This makes the encoding a permutation encoding.
///
#[derive(Clone, Debug)]
pub struct Chromosome(pub Vec<u16>);
impl Chromosome {
    pub fn init(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }
}

impl From<Vec<u16>> for Chromosome {
    fn from(value: Vec<u16>) -> Self {
        Self(value)
    }
}
