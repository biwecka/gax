// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////
use xhstt::{
    parser::solution_groups::{
        metadata::MetaData,
        solution::{events::Events, Solution},
        SolutionGroup, SolutionGroups,
    },
    xml::{Archives, X2014a},
};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Load XHSTT XML content.
    let xml = Archives::X2014a(X2014a::Abramson15).xml();

    // Parse XHSTT XML
    let mut xhstt = xhstt::parse(&xml);

    println!("stats = {:#?}", xhstt.instance_stats().unwrap());

    // Extract problem instance
    let instance = xhstt
        .clone()
        .instances
        .map(|i| i.list.first().cloned())
        .flatten()
        .expect("No problem instance found.");

    // Call algorithm
    let solution_events = alg_2::run(instance);

    // // Call algorithm
    // let solution_events = alg_1::run(instance.clone());

    // // Assemble solution
    // let solution_groups = SolutionGroups {
    //     list: vec![
    //         SolutionGroup {
    //             id: "biwecka_test".into(),
    //             metadata: MetaData {
    //                 contributor: "biwecka".into(),
    //                 date: "29.07.2024".into(),
    //                 description: "just a test".into(),
    //                 publication: None,
    //                 remarks: None,
    //             },

    //             solutions: vec![
    //                 Solution {
    //                     reference: instance.id,
    //                     description: None,
    //                     running_time: None,
    //                     events: Some(Events {
    //                         list: solution_events,
    //                     }),
    //                 }
    //             ]
    //         }
    //     ]
    // };

    // xhstt.solution_groups = Some(solution_groups);
    // let xml = xhstt::tools::xhstt_to_xml_string(&xhstt);
    // let xmlf = xhstt::tools::format_xml(&xml);
    // let _ = std::fs::write("./assets/solution.xml", xmlf);
}

////////////////////////////////////////////////////////////////////////////////
