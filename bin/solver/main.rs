// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////
use rand::distributions::Distribution;
use xhstt::data::{Instances, X2014a};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    // Load XHSTT XML content.
    let xhstt_xml = Instances::X2014a(X2014a::Abramson15).xml();

    // Parse XHSTT XML
    let xhstt = xhstt::parse(&xhstt_xml);

    // Extract problem instance
    let instance = if let Some(i) =
        xhstt.instances.map(|i| i.list.first().cloned()).flatten()
    {
        i
    } else {
        panic!("No problem instance found.");
    };

    // Call algorithm
    alg_1::run(instance);
}

////////////////////////////////////////////////////////////////////////////////
