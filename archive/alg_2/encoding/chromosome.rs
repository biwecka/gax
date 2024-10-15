/// The chromosome stores a vector of time indices, where the vector index
/// corresponds to the event index and the value of the vector corresponds
/// to the time index.
///
/// Example:
/// -   there are 4 events with no time: [ e0, e1, e2, e3 ]
/// -   there are 6 times: [ t0, t1, t2, t3, t4, t5 ]
///
/// A chromosome then looks like this: [ 2, 5, 1, 5 ]
/// This means
/// -   index 0 -> e0; value 2 -> t2
/// -   index 1 -> e1; value 5 -> t5
/// -   ...
///
#[derive(Clone, Debug)]
pub struct Chromosome(pub Vec<u8>);
impl Chromosome {
    #[allow(unused)]
    pub fn init(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }
}

impl From<Vec<u8>> for Chromosome {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
