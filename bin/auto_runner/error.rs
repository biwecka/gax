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
}
