// Imports /////////////////////////////////////////////////////////////////////
use super::configs::Config;
use crate::error::Error;
use alg_11::encoding::{Chromosome, Context, Cost};
use ga::encoding::ObjectiveValue;
use ga::{
    dynamics::Dynamic,
    operators::{Crossover, Mutation},
    process::{
        replacement::{Replace, Replacement},
        selection::{Select, Selection},
    },
    report::Report,
};
use std::fs;
use std::path::PathBuf;
use xhstt::parser::instances::Instance;
use xhstt::parser::solution_groups::solution::events::Event;
use xhstt::parser::XhsttArchive;

// Files Helper ////////////////////////////////////////////////////////////////
pub struct FilesHelper {
    /// /<instance>/<alg>/<cfg>
    cfg_path: PathBuf,

    /// /<instance>/<alg>/<cfg>/<run>
    run_path: PathBuf,
}

impl FilesHelper {
    pub fn new(
        data_dir: &str,
        instance: &str,
        alg: &str,
        params_id: &str,
        dynamics_id: &Option<String>,
        time: &str,
    ) -> Self {
        // Create config path
        let mut cfg_path = PathBuf::from(data_dir);
        cfg_path.push(instance);
        cfg_path.push(alg);
        match dynamics_id {
            Some(d) => cfg_path.push(format!("cfg_{}___dyn_{}", params_id, d)),
            None => cfg_path.push(format!("cfg_{}", params_id)),
        }

        // Create run_path
        let mut run_path = cfg_path.clone();
        run_path.push(format!("run_{}", time));

        Self { cfg_path, run_path }
    }

    pub fn ensure_dirs(&self) -> Result<(), Error> {
        // Create directories
        fs::create_dir_all(self.run_path.clone()).map_err(|e| {
            Error::IO(format!("Failed to create directories: {e}"))
        })
    }

    pub fn write_params_csv(&self, c: Config) -> Result<(), Error> {
        // Construct path to params.csv
        let mut params_csv_path = self.cfg_path.clone();
        params_csv_path.push("params.csv");

        // Create file content
        let content = Into::<ParamsCsv>::into(c).to_csv()?;

        // Check if file already exists; if so load it
        if params_csv_path.exists() {
            let existing = fs::read_to_string(params_csv_path.clone())
                .map_err(|e| {
                    Error::ParamsCsv(format!("Read existing file: {e}"))
                })?;

            if content.trim() != existing.trim() {
                println!("existing:\n{}", existing.trim());
                println!("new:\n{}", content.trim());

                println!("path: {:?}", params_csv_path);

                return Err(Error::ParamCsvNotMatching(
                    params_csv_path.clone(),
                ));
            } else {
                return Ok(());
            }
        }

        // Write content to file
        fs::write(params_csv_path, content)
            .map_err(|e| Error::ParamsCsv(format!("Writing file: {e}")))?;

        Ok(())
    }

    pub fn write_general_csv(
        &self,
        report: Report<Cost, Context, Chromosome>,
    ) -> Result<(), Error> {
        // Construct path to params.csv
        let mut general_csv_path = self.run_path.clone();
        general_csv_path.push("general.csv");

        // Create file content
        let content = Into::<GeneralCsv>::into(report).to_csv()?;

        // Write content to file
        fs::write(general_csv_path, content)
            .map_err(|e| Error::GeneralCsv(format!("Writing file: {e}")))?;

        Ok(())
    }

    pub fn write_generations_csv(
        &self,
        report: Report<Cost, Context, Chromosome>,
    ) -> Result<(), Error> {
        // Construct path to params.csv
        let mut generations_csv_path = self.run_path.clone();
        generations_csv_path.push("generations.csv");

        // Create file content
        let content = Into::<GenerationsCsv>::into(report).to_csv()?;

        // Write content to file
        fs::write(generations_csv_path, content)
            .map_err(|e| Error::GenerationsCsv(format!("Writing file: {e}")))?;

        Ok(())
    }

    pub fn write_solution_xml(
        &self,
        mut xhstt: XhsttArchive,
        instance: Instance,
        time: String,
        solution_events: Vec<Event>,
        report: Report<Cost, Context, Chromosome>,
    ) -> Result<(), Error> {
        // Create solution
        let solution = xhstt::tools::create_solution(
            &instance.id,
            &format!("run_{}", time),
            "biwecka",
            "GAX (GA for XHSTT)",
            Some(report.runtime),
            solution_events,
        );

        // Add solution to xhstt
        xhstt.solution_groups = Some(solution);

        // Construct solution.xml path
        let mut path = self.run_path.clone();
        path.push("solution.xml");

        // Write result
        xhstt::tools::write_xhstt(&xhstt, path).map_err(|_| {
            Error::SolutionXml("Failed to write solution.xml".into())
        })?;

        Ok(())
    }
}

// Params CSV //////////////////////////////////////////////////////////////////
#[derive(Clone, serde::Serialize)]
struct ParamsCsv {
    population: usize,
    mu_rate: f32,
    dynamic: bool,

    selection: String,
    crossover: String,
    mutation: String,
    replacement: String,

    dynamics: String,
}

impl ParamsCsv {
    pub fn to_csv(&self) -> Result<String, Error> {
        let mut writer =
            csv::WriterBuilder::new().delimiter(b';').from_writer(vec![]);

        writer
            .serialize(self.clone())
            .map_err(|e| Error::ParamsCsv(format!("Serializing data: {e}")))?;

        writer
            .flush()
            .map_err(|e| Error::ParamsCsv(format!("Flushing writer: {e}")))?;

        // Result
        let inner = writer
            .into_inner()
            .map_err(|e| Error::ParamsCsv(format!("Extract writer: {e}")))?;

        String::from_utf8(inner)
            .map_err(|e| Error::ParamsCsv(format!("CSV to string: {e}")))
    }
}

impl From<Config> for ParamsCsv {
    fn from(c: Config) -> Self {
        let population = c.params.population_size;
        let mu_rate = c.params.mutation_rate;
        let dynamic = c.dynamics.is_some();

        let selection = <Select as Selection<
            Cost,
            Context,
            Chromosome,
            usize,
        >>::identifier(&c.params.selection);

        let crossover = c.params.crossover.identifier();
        let mutation = c.params.mutation.identifier();
        let replacement =
            <Replace as Replacement<usize>>::identifier(&c.params.replacement);

        let dynamics = c
            .dynamics
            .unwrap_or_default()
            .into_iter()
            .map(|d| d.identifier())
            .collect::<Vec<_>>()
            .join(",");

        Self {
            population,
            mu_rate,
            dynamic,
            selection,
            crossover,
            mutation,
            replacement,
            dynamics,
        }
    }
}

// General CSV /////////////////////////////////////////////////////////////////
#[derive(Clone, serde::Serialize)]
struct GeneralCsv {
    success: bool,
    ov: usize,
    gen: usize,
    runtime: usize,
}

impl GeneralCsv {
    pub fn to_csv(&self) -> Result<String, Error> {
        let mut writer =
            csv::WriterBuilder::new().delimiter(b';').from_writer(vec![]);

        writer
            .serialize(self.clone())
            .map_err(|e| Error::GeneralCsv(format!("Serializing data: {e}")))?;

        writer
            .flush()
            .map_err(|e| Error::GeneralCsv(format!("Flushing writer: {e}")))?;

        // Result
        let inner = writer
            .into_inner()
            .map_err(|e| Error::GeneralCsv(format!("Extract writer: {e}")))?;

        String::from_utf8(inner)
            .map_err(|e| Error::GeneralCsv(format!("CSV to string: {e}")))
    }
}

impl From<Report<Cost, Context, Chromosome>> for GeneralCsv {
    fn from(r: Report<Cost, Context, Chromosome>) -> Self {
        let ov = r.population.first().unwrap().1.to_usize();
        let success = ov == 0;
        let gen = r.generation;
        let runtime = r.runtime;

        Self { success, ov, gen, runtime }
    }
}

// Generations CSV /////////////////////////////////////////////////////////////
#[derive(Clone)]
struct GenerationsCsv {
    list: Vec<GenCsv>,
}

#[derive(Clone, serde::Serialize)]
struct GenCsv {
    pub gen: usize,
    pub best: usize,
    pub worst: usize,

    pub mean: f64,
    pub median: f64,
    pub variance: f64,
    pub std_dev: f64,

    pub diversity: f64,
}

impl GenerationsCsv {
    pub fn to_csv(&self) -> Result<String, Error> {
        let mut writer =
            csv::WriterBuilder::new().delimiter(b';').from_writer(vec![]);

        for gen in &self.list {
            writer.serialize(gen.clone()).map_err(|e| {
                Error::GenerationsCsv(format!("Serializing data: {e}"))
            })?;
        }

        writer.flush().map_err(|e| {
            Error::GenerationsCsv(format!("Flushing writer: {e}"))
        })?;

        // Result
        let inner = writer.into_inner().map_err(|e| {
            Error::GenerationsCsv(format!("Extract writer: {e}"))
        })?;

        String::from_utf8(inner)
            .map_err(|e| Error::GenerationsCsv(format!("CSV to string: {e}")))
    }
}

impl From<Report<Cost, Context, Chromosome>> for GenerationsCsv {
    fn from(r: Report<Cost, Context, Chromosome>) -> Self {
        let list = r
            .log
            .into_iter()
            .map(|log| GenCsv {
                gen: log.generation,
                best: log.best,
                worst: log.worst,
                mean: log.mean,
                median: log.median,
                variance: log.variance,
                std_dev: log.std_dev,
                diversity: log.diversity,
            })
            .collect::<Vec<_>>();

        Self { list }
    }
}

////////////////////////////////////////////////////////////////////////////////
