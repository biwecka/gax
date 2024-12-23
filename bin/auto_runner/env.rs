// Imports /////////////////////////////////////////////////////////////////////
use crate::error::Error;

// Constants ///////////////////////////////////////////////////////////////////
static PLOTS_REPO: &str = "PLOTS_REPO";
static DATA_DIR: &str = "DATA_DIR";
static GIT_USERNAME: &str = "GIT_USERNAME";
static GIT_PASSWORD: &str = "GIT_PASSWORD";
static PUSHOVER_API_KEY: &str = "PUSHOVER_API_KEY";
static PUSHOVER_USER_KEY: &str = "PUSHOVER_USER_KEY";

// Environment /////////////////////////////////////////////////////////////////
/// Struct which loads, stores and provides all external configurations needed
/// for the auto runner from the environment variables.
pub struct Env {
    pub plots_repo: String,
    pub data_dir: String,
    pub git_username: String,
    pub git_password: String,
    pub pushover_api: String,
    pub pushover_user: String,
}

impl Env {
    /// Load environment variables and create an [`Env`] instance.
    pub fn load() -> Result<Self, Error> {
        dotenvy::dotenv()?;

        let plots_repo =
            std::env::var(PLOTS_REPO).map_err(|_| Error::EnvVar(PLOTS_REPO))?;

        let data_dir =
            std::env::var(DATA_DIR).map_err(|_| Error::EnvVar(DATA_DIR))?;

        let git_username = std::env::var(GIT_USERNAME)
            .map_err(|_| Error::EnvVar(GIT_USERNAME))?;

        let git_password = std::env::var(GIT_PASSWORD)
            .map_err(|_| Error::EnvVar(GIT_PASSWORD))?;

        let pushover_api = std::env::var(PUSHOVER_API_KEY)
            .map_err(|_| Error::EnvVar(PUSHOVER_API_KEY))?;

        let pushover_user = std::env::var(PUSHOVER_USER_KEY)
            .map_err(|_| Error::EnvVar(PUSHOVER_USER_KEY))?;

        Ok(Self {
            plots_repo,
            data_dir,
            git_username,
            git_password,
            pushover_api,
            pushover_user,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
