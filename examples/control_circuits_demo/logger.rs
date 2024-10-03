// Logger //////////////////////////////////////////////////////////////////////
pub struct RerunLogger {
    rec: rerun::RecordingStream,
}

impl RerunLogger {
    pub fn connect(name: &str) -> Self {
        let rec = rerun::RecordingStreamBuilder::new(name).spawn().unwrap();
        Self { rec }
    }

    #[allow(unused)]
    pub fn get_stream(&self) -> &rerun::RecordingStream {
        &self.rec
    }

    pub fn input(&self, t: usize, input: f64) {
        self.rec.set_time_sequence("time", t as u32);
        let _ = self.rec.log("val/input", &rerun::Scalar::new(input as f64));
    }

    pub fn output_pt1(&self, t: usize, output: f64) {
        self.rec.set_time_sequence("time", t as u32);
        let _ = self.rec.log("val/pt1", &rerun::Scalar::new(output as f64));
    }

    pub fn output_pt2(&self, t: usize, output: f64) {
        self.rec.set_time_sequence("time", t as u32);
        let _ = self.rec.log("val/pt2", &rerun::Scalar::new(output as f64));
    }
}

////////////////////////////////////////////////////////////////////////////////
