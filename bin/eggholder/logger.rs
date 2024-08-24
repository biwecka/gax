use colors_transform::Color;
use rerun::{RecordingStream, RecordingStreamBuilder, Scalar};

use crate::{chromosome::Chromosome, eggholder};

const GENERATION_TIME_SEQ: &'static str = "generation";

pub struct Logger {
    rec: RecordingStream,
}

impl std::default::Default for Logger {
    fn default() -> Self {
        let rec = RecordingStreamBuilder::new("eggholder").spawn().unwrap();
        Self { rec }
    }
}

impl Logger {
    pub fn draw_eggholder_function(&self) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, 0);

        let x0_values: Vec<f64> = (-512..512).step_by(2).map(|x| x as f64 ).collect();
        let x1_values: Vec<f64> = (-512..512).step_by(2).map(|x| x as f64 ).collect();

        let mut points: Vec<(f32, f32, f32)> = vec![];

        for x0 in x0_values {
            for x1 in &x1_values {
                let x2 = eggholder(x0, *x1);
                points.push((x0 as f32, *x1 as f32, x2 as f32));
            }
        }

        let min = -1000.;
        let max = 1000.;

        self.rec.log(
            "eggholder/fn",
            &rerun::Points2D::new(points.iter().map(|(a, b, _)| (*a, *b)))
                .with_colors(points.iter().map(|(_, _, c)| {
                    // Calculate hue value (interval [0; 240])
                    let hue = (((-c) - min) * (240. - 0.)) / (max - min) + 0.;

                    let hsl = colors_transform::Hsl::from(hue, 100., 50.);

                    let r = hsl.get_red().round() as u8;
                    let g = hsl.get_green().round() as u8;
                    let b = hsl.get_blue().round() as u8;

                    rerun::Color::from_rgb(r, g, b)
                }))
        ).unwrap();
    }

    pub fn draw_population(&self, generation: usize, population: &[(Chromosome, f64)]) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

        let mut points: Vec<(f32, f32)> = vec![];
        for individual in population {
            points.push((individual.0.x0() as f32, individual.0.x1() as f32));
        }

        let hsl = colors_transform::Hsl::from(310., 100., 50.);

        let r = hsl.get_red().round() as u8;
        let g = hsl.get_green().round() as u8;
        let b = hsl.get_blue().round() as u8;

        let color = rerun::Color::from_rgb(r, g, b);

        self.rec.log(
            "eggholder/pop",
            &rerun::Points2D::new(points.iter())
                .with_colors(points.iter().map(|_| {
                    color
                }))
        ).unwrap();
    }

    pub fn log_success_rate(&self, generation: usize, actual: f32, target: f32) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);
        let _ = self.rec.log("success_rate/pt1", &Scalar::new(actual as f64));
        let _ = self.rec.log("success_rate/target", &Scalar::new(target as f64));
    }

    pub fn log_mutation_std_dev(&self, generation: usize, std_dev: f32) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);
        let _ = self.rec.log("mutation/std_dev", &Scalar::new(std_dev as f64));
    }
}

