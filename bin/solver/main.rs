// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////
use xhstt::xml::{Archives, X2014a};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Load XHSTT XML content.
    let xml = Archives::X2014a(X2014a::Abramson15).xml();

    // Parse XHSTT XML
    let xhstt = xhstt::parse(&xml);

    // Extract problem instance
    let instance = xhstt
        .instances
        .map(|i| i.list.first().cloned())
        .flatten()
        .expect("No problem instance found.");

    // Call algorithm
    alg_1::run(instance.clone());
}

////////////////////////////////////////////////////////////////////////////////
