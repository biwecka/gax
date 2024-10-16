use std::cmp::Ordering;

use plotters::{coord::Shift, prelude::*};
use plotters_piston::PistonBackend;

pub fn objective_value_chart(
    area: &DrawingArea<PistonBackend, Shift>,
    best: &[usize],
    worst: &[usize],
) {
    //
    let max_x_value = (100 * (best.len() / 100) + 100).max(100);
    let max_y_value = *worst.iter().max().unwrap_or(&0);

    let mut chart = ChartBuilder::on(area)
        .caption("Objective Value", ("sans", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_x_value, 0..max_y_value)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Generations")
        .axis_desc_style(("sans", 14))
        .draw()
        .unwrap();

    // Draw best
    chart
        .draw_series(LineSeries::new(
            // Data
            best.iter()
                .enumerate()
                .map(|(x, y)| (x, *y))
                .collect::<Vec<(usize, usize)>>(),
            // Color
            &Palette99::pick(0),
        ))
        .unwrap()
        .label("best")
        .legend(|(x, y)| {
            PathElement::new(
                // Dimensions
                vec![(x, y), (x + 10, y)],
                // Color
                Palette99::pick(0),
            )
        });

    // Print out current best as number
    if let Some(x) = best.last().cloned() {
        chart
            .draw_series(PointSeries::of_element(
                vec![(best.len() - 1, x)],
                2,
                &Palette99::pick(0),
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(
                            format!("{}", x),
                            (0, -20),
                            ("sans", 14).into_font(),
                        )
                },
            ))
            .unwrap();
    }

    // Draw worst
    chart
        .draw_series(LineSeries::new(
            // Data
            worst
                .iter()
                .enumerate()
                .map(|(x, y)| (x, *y))
                .collect::<Vec<(usize, usize)>>(),
            // Color
            &Palette99::pick(0),
        ))
        .unwrap()
        .label("worst")
        .legend(|(x, y)| {
            PathElement::new(
                // Dimensions
                vec![(x, y), (x + 10, y)],
                // Color
                Palette99::pick(0),
            )
        });

    // Print out current worst as number
    if let Some(x) = worst.last().cloned() {
        chart
            .draw_series(PointSeries::of_element(
                vec![(worst.len() - 1, x)],
                2,
                &Palette99::pick(0),
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(
                            format!("{}", x),
                            (0, -20),
                            ("sans", 14).into_font(),
                        )
                },
            ))
            .unwrap();
    }
}

pub fn population_size_chart(
    area: &DrawingArea<PistonBackend, Shift>,
    population_size: &[usize],
) {
    //
    let max_x_value = (100 * (population_size.len() / 100) + 100).max(100);
    let max_y_value = *population_size.iter().max().unwrap_or(&0);

    let mut chart = ChartBuilder::on(area)
        .caption("Population Size", ("sans", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_x_value, 0..max_y_value)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Generations")
        .axis_desc_style(("sans", 14))
        .draw()
        .unwrap();

    // Draw best
    chart
        .draw_series(LineSeries::new(
            // Data
            population_size
                .iter()
                .enumerate()
                .map(|(x, y)| (x, *y))
                .collect::<Vec<(usize, usize)>>(),
            // Color
            &Palette99::pick(0),
        ))
        .unwrap()
        .label("best");
    // .legend(|(x, y)| PathElement::new(
    //     // Dimensions
    //     vec![(x, y), (x+10, y)],
    //     // Color
    //     &Palette99::pick(0)
    // ));

    // Print out current best as number
    if let Some(x) = population_size.last().cloned() {
        chart
            .draw_series(PointSeries::of_element(
                vec![(population_size.len() - 1, x)],
                2,
                &Palette99::pick(0),
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(
                            format!("{}", x),
                            (0, -20),
                            ("sans", 14).into_font(),
                        )
                },
            ))
            .unwrap();
    }
}

pub fn population_heatmap(
    area: &DrawingArea<PistonBackend, Shift>,
    heatmap: &ndarray::Array2<usize>,
) {
    let dimensions = heatmap.shape();
    let y = dimensions[0];
    let x = dimensions[1];

    let mut chart = ChartBuilder::on(area)
        .caption("Chromosome Heatmap", ("sans", 20))
        .margin(10)
        .top_x_label_area_size(30)
        // .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..(x), (y)..0)
        .unwrap();

    chart
        .configure_mesh()
        // .x_labels(15)
        // .y_labels(15)
        .max_light_lines(4)
        // .x_label_offset(35)
        // .y_label_offset(25)
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans", 20))
        .draw()
        .unwrap();

    let max_value = *heatmap.iter().max().unwrap_or(&usize::MAX) as f64;

    chart
        .draw_series(heatmap.indexed_iter().map(|((row, col), val)| {
            // dbg!(&row, &col);
            let hue = if max_value == 0. {
                0.666_667
            } else {
                0.666_667 * (max_value - *val as f64) / max_value
            };

            let x = col;
            let y = row;

            Rectangle::new(
                [(x, y), (x + 1, y + 1)],
                HSLColor(hue, 1., 0.5).filled(),
            )
        }))
        .unwrap();
}

pub fn selection_differential_chart(
    area: &DrawingArea<PistonBackend, Shift>,
    differentials: &[f32],
) {
    let max_x_value = (100 * (differentials.len() / 100) + 100).max(100) as f32;
    let max_y_value = *differentials
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .unwrap_or(&5.);

    let mut chart = ChartBuilder::on(area)
        .caption(
            "Selection differential (avg(selection) - avg(population))",
            ("sans", 20),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0. ..max_x_value, -max_y_value..max_y_value)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Generations")
        .axis_desc_style(("sans", 14))
        .draw()
        .unwrap();

    // Draw best
    chart
        .draw_series(LineSeries::new(
            // Data
            differentials
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f32, *y))
                .collect::<Vec<(f32, f32)>>(),
            // Color
            &Palette99::pick(0),
        ))
        .unwrap()
        .label("selection differential")
        .legend(|(x, y)| {
            PathElement::new(
                // Dimensions
                vec![(x, y), (x + 10, y)],
                // Color
                Palette99::pick(0),
            )
        });

    // Print out current best as number
    // if let Some(x) = best.last().cloned() {
    //     chart
    //         .draw_series(PointSeries::of_element(
    //             vec![(best.len() - 1, x)],
    //             2,
    //             &Palette99::pick(0),
    //             &|c, s, st| {
    //                 return EmptyElement::at(c)
    //                     + Circle::new((0, 0), s, st.filled())
    //                     + Text::new(
    //                         format!("{}", x),
    //                         (0, -20),
    //                         ("sans", 14).into_font()
    //                     )
    //             }
    //         )).unwrap();
    // }
}

pub fn population_ov_histogram(
    area: &DrawingArea<PistonBackend, Shift>,
    histogram_data: Vec<(usize, usize)>,
) {
    let max_value =
        histogram_data.iter().map(|(val, _)| *val).max().unwrap_or(0);

    let mut chart = ChartBuilder::on(area)
        .caption(
            "Objective Value distribution (in current population)",
            ("sans", 20),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_value, 0..histogram_data.len())
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.filled())
                .margin(10)
                .data(histogram_data),
        )
        .unwrap();
}
