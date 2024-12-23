//! The `solver` binary combines functionality provided by the library crates
//! in this repository, to "pull everything together" and actually attempt
//! solving a XHSTT problem instance.
//!
//! First of all, the `xhstt` library is used, as it provides a set of XHSTT
//! instances as raw XML and functions to parse this XML.
//!
//! Afterwards, the solver uses an imported algorithm (like `alg_12`) to
//! execute an algorithm on the selected problem instance.
//!
//! For writing solutions to disk, the `xhstt` library is again used to
//! correctly format the solution returned by the algorithm and convert it to
//! valid XHSTT XML.

// Imports /////////////////////////////////////////////////////////////////////
use xhstt::xml::{Archives, X2014a};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Select XHSTT problem instance and parse its XML file.
    let selection = Archives::X2014a(X2014a::Hdtt4);
    let xhstt = xhstt::parse(&selection.xml());

    // Extract problem instance
    let instance = xhstt.instance().expect("No problem instance found.");

    // Get current time (UTC) and start measuring time
    let time = chrono::Utc::now().to_rfc3339();
    let start = std::time::Instant::now();

    // Execute algorithm
    let solution_events = alg_12::run(instance.clone());

    // Stop time
    let runtime = start.elapsed().as_secs();

    // Clone the original XHSTT instance
    let mut xhstt_solution = xhstt.clone();

    // Convert the solution events returned by the algorithm into a solution
    // group.
    let solution = xhstt::tools::create_solution(
        &instance.id,
        &format!("run_{}", time),
        "biwecka",
        "GAX (GA for XHSTT)",
        Some(runtime as usize),
        solution_events,
    );

    // Overwrite the existing solution groups (from the original XML file) with
    // our solution group which contains only our solution.
    xhstt_solution.solution_groups = Some(solution);

    // Create directory for writing solution file
    let dir = match selection {
        Archives::X2014a(_) => instance.metadata.name,
        Archives::X2014(_) => instance.id,
    };
    std::fs::create_dir_all(format!("./assets/solutions/{dir}"))
        .expect("create dir");

    // Write result
    let _ = xhstt::tools::write_xhstt(
        &xhstt_solution,
        format!("./assets/solutions/{dir}/solution.xml"),
    );
}

////////////////////////////////////////////////////////////////////////////////
