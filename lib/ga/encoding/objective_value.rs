/// Implementions of this trait must ensure, that their ordering is always
/// DESCENDING in the sense of the measure they represent.
///
/// Examples:
/// -   Fitness from 0.0 to 1.0, where 1.0 is the goal (maximization):
///     A sorted vector must then look like this [0.9, 0.7, 0.6, 0.6, ...]
/// -   Fitness from 0 to 1000, where 0 is the goal (minimization):
///     A sorted vector must then look like this [20, 50, 110, 120, 300, ...]
///
pub trait ObjectiveValue: PartialEq + Eq + PartialOrd + Ord + Clone {}

// Implementations /////////////////////////////////////////////////////////////
impl ObjectiveValue for usize {}

////////////////////////////////////////////////////////////////////////////////
