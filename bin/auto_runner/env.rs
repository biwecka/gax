use crate::error::Error;

static PLOTS_REPO: &str = "PLOTS_REPO";
static GIT_USERNAME: &str = "GIT_USERNAME";
static GIT_PASSWORD: &str = "GIT_PASSWORD";
static PUSHOVER_API_KEY: &str = "PUSHOVER_API_KEY";
static PUSHOVER_USER_KEY: &str = "PUSHOVER_USER_KEY";

pub struct Env {
    pub plots_repo: String,
    pub git_username: String,
    pub git_password: String,
    pub pushover_api: String,
    pub pushover_user: String,
}

impl Env {
    pub fn load() -> Result<Self, Error> {
        dotenvy::dotenv()?;

        let plots_repo =
            std::env::var(PLOTS_REPO).map_err(|_| Error::EnvVar(PLOTS_REPO))?;

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
            git_username,
            git_password,
            pushover_api,
            pushover_user,
        })
    }
}
