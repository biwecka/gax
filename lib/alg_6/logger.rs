// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context, Cost};
use ga::colors_transform::Color;
use hashbrown::HashMap;
use std::ops::AddAssign;

// Logger //////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct Logger {}

impl ga::tools::rerun_logger::CustomLogger<Cost, Context, Chromosome>
    for Logger
{
    fn log(
        &self,
        rec: &ga::rerun::RecordingStream,
        generation: usize,
        ctx: &Context,
        population: &[(Chromosome, Cost)],
    ) {
        // Log chromosome
        let x_values: Vec<usize> = (0..ctx.num_events).collect();
        let y_values: Vec<Vec<usize>> = population
            .iter()
            .map(|(c, _)| c.iter().cloned().collect())
            .collect();

        rec.set_time_sequence("generation", generation as i32);

        let mut points: Vec<(usize, usize)> = vec![];

        for y in y_values {
            let mut p: Vec<(usize, usize)> =
                x_values.iter().zip(y).map(|(a, b)| (*a, b)).collect();
            points.append(&mut p);
        }

        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        for p in points {
            map.entry(p).or_default().add_assign(1);
        }

        let min = *map.values().min().unwrap() as f32;
        let max = *map.values().max().unwrap() as f32;

        rec.log(
            "chromosomes",
            &ga::rerun::Points2D::new(
                map.iter().map(|((x, y), _)| (*x as f32, *y as f32)),
            )
            .with_colors(map.iter().map(|((_, _), n)| {
                // Calculate hue value (interval [0; 240])
                let hue = ((*n as f32 - min) * (240. - 0.)) / (max - min) + 0.;

                let hsl = ga::colors_transform::Hsl::from(hue, 100., 50.);

                let r = hsl.get_red().round() as u8;
                let g = hsl.get_green().round() as u8;
                let b = hsl.get_blue().round() as u8;

                ga::rerun::Color::from_rgb(r, g, b)
            })),
        )
        .unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////
