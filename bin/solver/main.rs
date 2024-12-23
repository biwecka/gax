// Modules /////////////////////////////////////////////////////////////////////

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
