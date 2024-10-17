use std::{fs, path::PathBuf};

// Imports /////////////////////////////////////////////////////////////////////
use crate::error::Error;

// Functions ///////////////////////////////////////////////////////////////////
pub fn create_dirs(
    data_dir: &str,
    instance: &str,
    alg: &str,
    params_id: &str,
    dynamics_id: &Option<String>,
    time: &str,
) -> Result<(), Error> {
    // Create base path from data_dir.
    let mut path = PathBuf::from(data_dir);

    // Instance path (e.g. "hdtt4")
    path.push(instance);

    // Algorithm
    path.push(alg);

    // Add configuration path (with or without dynamic)
    match dynamics_id {
        Some(d) => path.push(format!("cfg_{}___dyn_{}", params_id, d)),
        None => path.push(format!("cfg_{}", params_id)),
    }

    // Add run
    path.push(format!("run_{}", time));

    // Create directories
    fs::create_dir_all(path)
        .map_err(|e| Error::IO(format!("Failed to create directories: {e}")))
}

////////////////////////////////////////////////////////////////////////////////
