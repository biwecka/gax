use std::cmp::Ordering;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;

const FPS: u32 = 30;
fn main() {
    // Communication channel
    #[derive(Debug)]
    struct Data {
        pub index: usize,
        pub x: f32,
        pub y: f32,
    }
    let (sender, receiver) = std::sync::mpsc::channel::<Data>();

    // Spawn data producing thread.
    let _data = std::thread::spawn(move || {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum Functions {
            Square,
            Sin,
        }

        impl Functions {
            fn index(&self) -> usize {
                match self {
                    Self::Square => 0,
                    Self::Sin => 1,
                }
            }

            fn value(&self, x: f32) -> f32 {
                match self {
                    Self::Square => x * x,
                    Self::Sin => x.sin(),
                }
            }
        }

        let mut function = Functions::Square;
        let mut x = -2.;
        let mut limit = 1.99;

        loop {
            let _ = sender.send(Data {
                index: function.index(),
                x,
                y: function.value(x),
            });
            x += 0.1;

            if x > limit {
                let _ = sender.send(Data {
                    index: function.index(),
                    x,
                    y: function.value(x),
                });

                if function == Functions::Square {
                    function = Functions::Sin;
                    x = -5.;
                    limit = 4.99;

                } else {
                    break;
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });


    let mut window: PistonWindow = WindowSettings::new("Real Time CPU Usage", [450, 300])
        .samples(4)
        .build()
        .unwrap();

    window.set_max_fps(FPS as u64);

    let mut data_0: Vec<(f32, f32)> = vec![];
    let mut data_1: Vec<(f32, f32)> = vec![];

    // This function is executed exactlx FPS amount of times per second.
    while let Some(_) = draw_piston_window(&mut window, |b| {
        while let Ok(data) = receiver.try_recv() {
            if data.index == 0 {
                data_0.push((data.x, data.y));
            }
            else if data.index == 1 {
                data_1.push((data.x, data.y));
            }
        }

        let root = b.into_drawing_area();
        root.fill(&WHITE)?;


        let d0_x_min = data_0.clone().iter().min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Less)).unwrap_or(&(0., 0.)).0;
        let d0_x_max = data_0.clone().iter().max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Less)).unwrap_or(&(0., 0.)).0;

        let d1_x_min = data_1.clone().iter().min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Less)).unwrap_or(&(0., 0.)).0;
        let d1_x_max = data_1.clone().iter().max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Less)).unwrap_or(&(0., 0.)).0;

        let x_min = f32::min(d0_x_min, d1_x_min).floor();
        let x_max = f32::max(d0_x_max, d1_x_max).ceil();

        let mut chart = ChartBuilder::on(&root)
            .caption("y = x^2", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, -1.1f32..4.1f32)?;

        chart.configure_mesh().draw()?;


        // Draw data_0
        chart
            .draw_series(LineSeries::new(data_0.clone(), &Palette99::pick(0)))?
            .label("y = x * x")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &Palette99::pick(0)));

        // Draw data_1
        chart
            .draw_series(LineSeries::new(data_1.clone(), &Palette99::pick(1)))?
            .label("y = sin(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &Palette99::pick(1)));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

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