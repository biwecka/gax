// Imports /////////////////////////////////////////////////////////////////////

// Enums ///////////////////////////////////////////////////////////////////////
/// In contrast to the other parameters, [`Replacement`] is a simple enum which
/// represents different replacement strategies. This enum does not provide
/// any methods to be called. That's because the [`super::Parameters`] struct
/// directly processes the replacement strategy and provides two functions
/// ([`super::Parameters::selection_size`] and
/// [`super::Parameters::elite_size`]) to access the processed replacement
/// strategy information.
pub enum Replacement {
    Full,
    Elite(f32),
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
