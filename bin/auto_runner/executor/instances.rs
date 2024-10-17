// Imports /////////////////////////////////////////////////////////////////////

use xhstt::{parser::instances::Instance, xml::{Archives, X2014a}};

// Functions ///////////////////////////////////////////////////////////////////
pub fn instances() -> Vec<Instance> {
    let hdtt4 = xhstt::parse(&Archives::X2014a(X2014a::Hdtt4).xml()).instance()
        .expect("hdtt4 not found");

    let hdtt5 = xhstt::parse(&Archives::X2014a(X2014a::Hdtt5).xml()).instance()
        .expect("hdtt5 not found");

    vec![
        hdtt4,
        hdtt5,
    ]
}

////////////////////////////////////////////////////////////////////////////////
