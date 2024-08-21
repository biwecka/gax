// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////
use xhstt::{
    // parser::solution_groups::{
    //     metadata::MetaData,
    //     solution::{events::Events, Solution},
    //     SolutionGroup, SolutionGroups,
    // },
    xml::{Archives, X2014a},
};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Load XHSTT XML content.
    // let xml = Archives::X2014a(X2014a::Abramson15).xml();
    let xml = Archives::X2014a(X2014a::Hdtt4).xml();

    // Parse XHSTT XML
    let xhstt = xhstt::parse(&xml);

    // Extract problem instance
    let instance = xhstt
        .clone()
        .instances
        .and_then(|i| i.list.first().cloned())
        .expect("No problem instance found.");

    // Call algorithm
    // let solution_events = alg_2::run(instance.clone());
    // let solution_events = alg_3::run(instance.clone());
    // let solution_events = alg_4::run(instance.clone());
    // let solution_events = alg_5::run(instance.clone());
    let solution_events = alg_6::run(instance.clone());

    // Write result
    let solution = xhstt::tools::create_solution(
        &instance.id,
        "test_run",
        "biwecka",
        "GA for XHSTT",
        solution_events,
    );
    let mut xhstt_solution = xhstt.clone();
    xhstt_solution.solution_groups = Some(solution);

    let _ = xhstt::tools::write_xhstt(&xhstt_solution, "./assets/solution.xml");
}

////////////////////////////////////////////////////////////////////////////////
