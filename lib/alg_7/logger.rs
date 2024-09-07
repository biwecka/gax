
pub struct Logger {}
impl std::default::Default for Logger {
    fn default() -> Self {
        Self {}
    }
}

impl ga::tools::rerun_logger::CustomLogger for Logger {
    fn log<
        Ov: ga::encoding::ObjectiveValue,
        Ctx: ga::encoding::Context,
        Ge: ga::encoding::Genotype<Ctx>,
    >(&self, rec: &rerun::RecordingStream, population: &[(Ge, Ov)]) {
        
    }
}
