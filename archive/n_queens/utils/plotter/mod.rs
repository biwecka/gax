#![allow(unused)]

// Imports /////////////////////////////////////////////////////////////////////
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::draw_piston_window;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::Stats;

// Constants ///////////////////////////////////////////////////////////////////
const FPS: usize = 60;

// Plotter /////////////////////////////////////////////////////////////////////
pub struct Plotter {
    stats_receiver: Receiver<Stats>,
    window: PistonWindow,
    stats: Stats,
}

impl Plotter {
    pub fn init() -> (Self, Sender<Stats>) {
        // Create communication channel
        let (sender, receiver) = mpsc::channel::<Stats>();

        // Create window
        let mut window: PistonWindow =
            WindowSettings::new("Genetic Algorithm Stats", [450, 300])
                // .exit_on_esc(true)
                .samples(4)
                .build()
                .unwrap();

        window.set_max_fps(FPS as u64);

        (
            Self { stats_receiver: receiver, window, stats: Stats::default() },
            sender,
        )
    }

    pub fn start(mut self) {
        // Draw-Loop
        while draw_piston_window(&mut self.window, |b| {
            // Create drawing area
            let root = b.into_drawing_area();
            root.fill(&WHITE)?;

            // Update stats
            while let Ok(update) = self.stats_receiver.try_recv() {
                self.stats = update;
            }

            // Render stats
            let areas = root.split_evenly((3, 2));
            let area_objective_value = &areas[0]; // best + worst
            let area_population_size = &areas[1];
            let area_population_heatmap = &areas[2];
            let area_selection_differential = &areas[3];
            let area_ov_dist = &areas[4];
            // let area_distinct_selections = &areas[4];

            plot_utils::objective_value_chart(
                area_objective_value,
                &self.stats.best,
                &self.stats.worst,
            );

            plot_utils::population_size_chart(
                area_population_size,
                &self.stats.population_size,
            );

            plot_utils::population_heatmap(
                area_population_heatmap,
                &self.stats.chromosome_heatmap,
            );

            plot_utils::selection_differential_chart(
                area_selection_differential,
                &self.stats.selection_differential,
            );

            plot_utils::population_ov_histogram(
                area_ov_dist,
                self.stats.ov_distribution.clone(),
            );

            // Refresh UI
            root.present()?;
            Ok(())
        })
        .is_some()
        {}
    }
}
