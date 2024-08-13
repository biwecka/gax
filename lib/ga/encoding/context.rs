/// This trait should be implemented by a struct which holds additional and
/// auxiliary variables, which are needed to e.g. generate the initial
/// population.
/// Any data can be added to structs which implement this trait, as the struct's
/// value will be directly passed to some functions to be used in their
/// implementation.
pub trait Context {}
