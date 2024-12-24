// Modules /////////////////////////////////////////////////////////////////////
pub mod db;
pub mod model;
pub mod parser;
pub mod tools;
pub mod utils;
pub mod xml;

// Functions ///////////////////////////////////////////////////////////////////

/// Parse the given string as XHSTT XML archive.  
/// **Attention:** Function *panics* on error.
pub fn parse(xml: &str) -> parser::XhsttArchive {
    quick_xml::de::from_str(xml).unwrap()
}

// Tests ///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::{
        tools,
        xml::{Archives, X2014a},
    };

    #[test]
    fn parse_abramson_15() {
        // XML string
        let xml = Archives::X2014a(X2014a::Abramson15).xml();

        // Format original xml
        let orig_formatted = tools::format_xml(&xml);

        // Parse xml to datastructure
        let data = crate::parse(&xml);

        // Data to string
        let data_str = quick_xml::se::to_string_with_root(
            "HighSchoolTimetableArchive",
            &data,
        )
        .unwrap();

        // Format
        let data_str_formatted = tools::format_xml(&data_str);

        // Assert
        assert_eq!(orig_formatted, data_str_formatted)
    }

    #[test]
    fn parse_all_11() {
        // XML string
        let xml = Archives::X2014a(X2014a::All11).xml();

        // Format original xml
        let orig_formatted = tools::format_xml(&xml);

        // Parse xml to datastructure
        let data = crate::parse(&xml);

        // Data to string
        let data_str = quick_xml::se::to_string_with_root(
            "HighSchoolTimetableArchive",
            &data,
        )
        .unwrap();

        // Format
        let data_str_formatted = tools::format_xml(&data_str);

        // Assert
        assert_eq!(orig_formatted, data_str_formatted)
    }

    // #[test]
    // fn parse_australia_bghs_98() {
    //     // XML string
    //     let xml = Archives::X2014(X2014::AustraliaBGHS98).xml();

    //     // Format original xml
    //     let orig_formatted = tools::format_xml(&xml);

    //     // Parse xml to datastructure
    //     let data = crate::parse(&xml);

    //     // Data to string
    //     let data_str = quick_xml::se::to_string_with_root(
    //         "HighSchoolTimetableArchive",
    //         &data
    //     ).unwrap();

    //     // Format
    //     let data_str_formatted = tools::format_xml(&data_str);

    //     // Assert
    //     assert_eq!(orig_formatted, data_str_formatted)
    // }
}

////////////////////////////////////////////////////////////////////////////////
