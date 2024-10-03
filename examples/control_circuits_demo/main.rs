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
        if t < 50 {
            0.
        } else if t < 200 {
            1.
        } else if t < 500 {
            0.5
        } else {
            1.5
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
