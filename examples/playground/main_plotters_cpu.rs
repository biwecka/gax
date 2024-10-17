// fn main() {
//     n_queens::run();
// }

use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
// use plotters_piston::{draw_piston_window};
use systemstat::platform::common::Platform;
use systemstat::System;

use std::collections::vec_deque::VecDeque;

const FPS: u32 = 10;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Real Time CPU Usage", [450, 300])
        .samples(4)
        .build()
        .unwrap();

    let sys = System::new();

    window.set_max_fps(FPS as u64);

    let mut load_measurement: Vec<_> = (0..FPS).map(|_| sys.cpu_load().unwrap()).collect();
    let mut epoch = 0;
    let mut data = vec![];


    while let Some(_) = draw_piston_window(&mut window, |b| {
        let cpu_loads = load_measurement[epoch % FPS as usize].done()?;

        let root = b.into_drawing_area();
        root.fill(&WHITE)?;

        if data.len() < cpu_loads.len() {
            for _ in data.len()..cpu_loads.len() {
                data.push(VecDeque::from(vec![0f32; N_DATA_POINTS + 1]));
            }
        }

        for (core_load, target) in cpu_loads.into_iter().zip(data.iter_mut()) {
            if target.len() == N_DATA_POINTS + 1 {
                target.pop_front();
            }
            target.push_back(1.0 - core_load.idle);
        }

        let mut cc = ChartBuilder::on(&root)
            .margin(10)
            .caption("Real Time CPU Usage", ("sans-serif", 30))
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32)?;

        cc.configure_mesh()
            .x_label_formatter(&|x| format!("{}", -(LENGTH as f32) + (*x as f32 / FPS as f32)))
            .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
            .x_labels(15)
            .y_labels(5)
            .x_desc("Seconds")
            .y_desc("% Busy")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        for (idx, data) in (0..).zip(data.iter()) {
            cc.draw_series(LineSeries::new(
                (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
                &Palette99::pick(idx),
            ))?
            .label(format!("CPU {}", idx))
            .legend(move |(x, y)| {
                Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
            });
        }

        cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        load_measurement[epoch % FPS as usize] = sys.cpu_load()?;
        epoch += 1;
        Ok(())
    }) {}
}


////////////////////////////////////////////////////////////////////////////////
use piston_window::context::Context;
use piston_window::ellipse::circle;
use piston_window::{circle_arc, ellipse, line, rectangle, Event, Loop};
use piston_window::{G2d, /*PistonWindow*/};

use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind,
};

#[derive(Debug)]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for DummyBackendError {}

pub struct PistonBackend<'a, 'b> {
    size: (u32, u32),
    scale: f64,
    context: Context,
    graphics: &'b mut G2d<'a>,
}

fn make_piston_rgba(color: &BackendColor) -> [f32; 4] {
    let (r, g, b) = color.rgb;
    let a = color.alpha;

    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32,
    ]
}
fn make_point_pair(a: BackendCoord, b: BackendCoord, scale: f64) -> [f64; 4] {
    [
        a.0 as f64 * scale,
        a.1 as f64 * scale,
        b.0 as f64 * scale,
        b.1 as f64 * scale,
    ]
}

impl<'a, 'b> PistonBackend<'a, 'b> {
    pub fn new(size: (u32, u32), scale: f64, context: Context, graphics: &'b mut G2d<'a>) -> Self {
        Self {
            size,
            context,
            graphics,
            scale,
        }
    }
}

impl<'a, 'b> DrawingBackend for PistonBackend<'a, 'b> {
    type ErrorType = DummyBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        piston_window::rectangle(
            make_piston_rgba(&color),
            make_point_pair(point, (1, 1), self.scale),
            self.context.transform,
            self.graphics,
        );
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        line(
            make_piston_rgba(&style.color()),
            self.scale,
            make_point_pair(from, to, self.scale),
            self.context.transform,
            self.graphics,
        );
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if fill {
            rectangle(
                make_piston_rgba(&style.color()),
                make_point_pair(
                    upper_left,
                    (bottom_right.0 - upper_left.0, bottom_right.1 - upper_left.1),
                    self.scale,
                ),
                self.context.transform,
                self.graphics,
            );
        } else {
            let color = make_piston_rgba(&style.color());
            let [x0, y0, x1, y1] = make_point_pair(upper_left, bottom_right, self.scale);
            line(
                color,
                self.scale,
                [x0, y0, x0, y1],
                self.context.transform,
                self.graphics,
            );
            line(
                color,
                self.scale,
                [x0, y1, x1, y1],
                self.context.transform,
                self.graphics,
            );
            line(
                color,
                self.scale,
                [x1, y1, x1, y0],
                self.context.transform,
                self.graphics,
            );
            line(
                color,
                self.scale,
                [x1, y0, x0, y0],
                self.context.transform,
                self.graphics,
            );
        }
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let rect = circle(center.0 as f64, center.1 as f64, radius as f64);
        if fill {
            ellipse(
                make_piston_rgba(&style.color()),
                rect,
                self.context.transform,
                self.graphics,
            );
        } else {
            circle_arc(
                make_piston_rgba(&style.color()),
                self.scale,
                std::f64::consts::PI,
                0.0,
                rect,
                self.context.transform,
                self.graphics,
            );
            circle_arc(
                make_piston_rgba(&style.color()),
                self.scale,
                0.0,
                std::f64::consts::PI,
                rect,
                self.context.transform,
                self.graphics,
            );
        }
        Ok(())
    }
}

#[allow(clippy::single_match)]
pub fn draw_piston_window<F: FnOnce(PistonBackend) -> Result<(), Box<dyn std::error::Error>>>(
    window: &mut PistonWindow,
    draw: F,
) -> Option<Event> {
    if let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| match event {
            Event::Loop(Loop::Render(arg)) => {
                draw(PistonBackend::new(
                    (arg.draw_size[0], arg.draw_size[1]),
                    arg.window_size[0] / arg.draw_size[0] as f64,
                    c,
                    g,
                ))
                .ok();
            }
            _ => {}
        });
        return Some(event);
    }
    None
}