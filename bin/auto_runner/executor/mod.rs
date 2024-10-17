// Modules /////////////////////////////////////////////////////////////////////
mod configs;
mod instances;
mod files;

// Imports /////////////////////////////////////////////////////////////////////
use configs::Config;
use xhstt::parser::instances::Instance;
use crate::{env::Env, error::Error, git::Git, log::Logger};

// Executor ////////////////////////////////////////////////////////////////////
pub struct ExecutorAlg12<'a> {
    env: Env,
    log: &'a Logger,
    git: Git,

    instances: Vec<Instance>,
    next_instance: usize,

    configs: Vec<Config>,
    next_config: usize,
}

impl<'a> ExecutorAlg12<'a> {
    pub fn new(env: Env, log: &'a Logger, git: Git) -> Self {
        let instances = instances::instances();
        let next_instance = 0;
        assert!(!instances.is_empty());

        let configs = configs::configs();
        let next_config = 0;
        assert!(!configs.is_empty());

        Self {
            env, log, git,
            instances, next_instance,
            configs, next_config,
        }
    }

    pub fn run_next(&mut self) -> Result<(), Error> {
        // Get current ISO timestamp
        let time = chrono::Utc::now().to_rfc3339();

        // Get next instance and config
        let i = self.instances[self.next_instance].clone();
        let cfg = self.configs[self.next_config].clone();

        println!(
            "[{}] alg_12 (inst={}, cfg={})",
            time,
            i.metadata.name,
            self.next_config
        );

        // Call algorithm
        let (events, report) = alg_12::auto_run(
            i.clone(),
            cfg.params,
            cfg.dynamics
        );

        // 1) Create directories
        files::create_dirs(
            &self.env.data_dir,
            &i.metadata.name,
            "alg_12",
            &report.parameter_identifier,
            &report.dynamics_identifier,
            &time
        )?;

        // 2) Write params.csv
        // 3) Write run_x/general.csv
        // 4) Write run_x/generations.csv
        // 5) Write solution.xml

        self.inc();
        Ok(())
    }

    /// Advance `next_config` and `next_instance`. Always iterate all configs
    /// first, before advancing to the next instance.
    fn inc(&mut self) {
        self.next_config += 1;

        if self.next_config >= self.configs.len() {
            self.next_config = 0;

            self.next_instance += 1;
        }

        if self.next_instance >= self.instances.len() {
            self.next_instance = 0;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
