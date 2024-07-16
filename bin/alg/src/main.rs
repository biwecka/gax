// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    let xml_raw = xhstt::data::Instances::X2014a(
        xhstt::data::X2014a::Abramson15
    ).xml();

    let xml_formatted = xhstt::tools::format_xml(&xml_raw);

    let parsed = xhstt::parse(&xml_raw);
    // println!("{parsed:#?}");

    let parsed_str = xhstt::tools::format_xml(
        &quick_xml::se::to_string_with_root(
            "HighSchoolTimetableArchive",
            &parsed
        ).unwrap()
    );

    dbg!(xml_formatted.eq(&parsed_str));


    // let xml_new = quick_xml::se::to_string_with_root("HighSchoolTimetableArchive", &parsed).unwrap();
    // let formatted = xhstt::tools::format_xml(&xml_new);
    // std::fs::write("./new.xml", formatted).unwrap();
}

////////////////////////////////////////////////////////////////////////////////
