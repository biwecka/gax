// Imports /////////////////////////////////////////////////////////////////////

use xhstt::{
    parser::{instances::Instance, XhsttArchive},
    xml::{Archives, X2014a},
};

// Functions ///////////////////////////////////////////////////////////////////
pub fn instances() -> Vec<(XhsttArchive, Instance)> {
    let hdtt4 = xhstt::parse(&Archives::X2014a(X2014a::Hdtt4).xml());
    let hdtt5 = xhstt::parse(&Archives::X2014a(X2014a::Hdtt5).xml());

    vec![
        (hdtt4.clone(), hdtt4.instance().unwrap()),
        (hdtt5.clone(), hdtt5.instance().unwrap()),
    ]
}

////////////////////////////////////////////////////////////////////////////////
