// Modules /////////////////////////////////////////////////////////////////////
mod configs;
mod files;
mod instances;

// Imports /////////////////////////////////////////////////////////////////////
use crate::{env::Env, error::Error};
use configs::Config;
use files::FilesHelper;
use xhstt::parser::{instances::Instance, XhsttArchive};

// Executor ////////////////////////////////////////////////////////////////////
pub struct ExecutorAlg12 {
    env: Env,
    // log: &'a Logger,
    // git: Git,
    instances: Vec<(XhsttArchive, Instance)>,
    next_instance: usize,

    configs: Vec<Config>,
    next_config: usize,
}

impl ExecutorAlg12 {
    pub fn new(env: Env) -> Self {
        let instances = instances::instances();
        let next_instance = 0;
        assert!(!instances.is_empty());

        let configs = configs::configs();
        let next_config = 3;
        assert!(!configs.is_empty());

        Self { env, instances, next_instance, configs, next_config }
    }

    pub fn run_next(&mut self) -> Result<(), Error> {
        // Get current ISO timestamp
        let time = chrono::Utc::now().to_rfc3339();

        // Get next instance and config
        let i = self.instances[self.next_instance].clone();
        let cfg = self.configs[self.next_config].clone();

        println!(
            "[{}] alg_12 (inst={}, cfg={})",
            time, i.1.metadata.name, self.next_config
        );

        // Execute algorithm
        let (events, report) = alg_12::auto_run(
            i.1.clone(),
            cfg.clone().params,
            cfg.clone().dynamics,
        );

        // Create file helper for the following steps
        let fh = FilesHelper::new(
            &self.env.data_dir,
            &i.1.metadata.name,
            "alg_12",
            &report.parameter_identifier,
            &report.dynamics_identifier,
            &time,
        );

        // 1) Create directories
        fh.ensure_dirs()?;

        // 2) Write params.csv
        fh.write_params_csv(cfg.clone())?;

        // 3) Write run_x/general.csv
        fh.write_general_csv(report.clone())?;

        // 4) Write run_x/generations.csv
        fh.write_generations_csv(report.clone())?;

        // 5) Write solution.xml
        fh.write_solution_xml(i.0, i.1, time, events, report)?;

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
