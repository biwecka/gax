// Imports /////////////////////////////////////////////////////////////////////
use std::path::PathBuf;

// Error ///////////////////////////////////////////////////////////////////////
/// Error type unifying all errors that can occur in the auto runner.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),

    #[error("Environment variable '{0}' not found.")]
    EnvVar(&'static str),

    #[error("Dotenvy error: {0}")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Ctrl+C handler: {0}")]
    CtrlC(#[from] ctrlc::Error),

    #[error("Pushover: {0}")]
    Pushover(#[from] pushover::Error),

    #[error("IO Error: {0}")]
    IO(String),

    #[error("Existing and new param.csv don't match: {0}")]
    ParamCsvNotMatching(PathBuf),

    #[error("Error @ params.csv: {0}")]
    ParamsCsv(String),

    #[error("Error @ general.csv: {0}")]
    GeneralCsv(String),

    #[error("Error @ generations.csv: {0}")]
    GenerationsCsv(String),

    #[error("Error @ solution.xml: {0}")]
    SolutionXml(String),
}

////////////////////////////////////////////////////////////////////////////////
