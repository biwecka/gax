// Modules /////////////////////////////////////////////////////////////////////
mod logger;

// Imports /////////////////////////////////////////////////////////////////////
use control_circuits::{PT1, PT2};
use logger::RerunLogger;

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    let logger = RerunLogger::connect("control_circuits");

    let mut pt1 = PT1::new(16., 1., 1.);
    let mut pt2 = PT2::new(1., 10., 0.4, 1., 1.);

    let input = |t: usize| -> f64 {
        // if t < 50 {
        //     0.
        // } else if t < 200 {
        //     1.
        // } else if t < 500 {
        //     0.5
        // } else {
        //     1.5
        // }

        if t < 10 {
            0.1
        } else if t < 50 {
            0.15
        } else if t < 100 {
            0.2
        } else if t < 130 {
            0.1
        } else if t < 150 {
            0.12
        } else if t < 200 {
            0.4
        } else if t < 225 {
            0.6
        } else if t < 250 {
            0.8
        } else if t < 300 {
            1.0
        } else if t < 330 {
            2.0
        } else if t < 360 {
            2.6
        } else if t < 400 {
            3.0
        } else if t < 500 {
            4.0
        } else if t < 550 {
            4.5
        } else if t < 600 {
            6.5
        } else {
            10.
        }
    };

    for t in 0..800 {
        // Output (controlled)
        pt1.update(input(t));
        pt2.update(input(t));

        // Log
        logger.input(t, input(t));
        logger.output_pt1(t, pt1.get_output());
        logger.output_pt2(t, pt2.get_output());

        // Sleep
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

////////////////////////////////////////////////////////////////////////////////
