// Modules /////////////////////////////////////////////////////////////////////
pub mod data;
pub mod model;
pub mod tools;

// Functions ///////////////////////////////////////////////////////////////////
pub fn parse(xml: &str) -> model::XhsttArchive {
    quick_xml::de::from_str(xml).unwrap()
}

// Tests ///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::{
        data::{Instances, X2014a},
        tools,
    };

    #[test]
    fn parse_abramson_15() {
        // XML string
        let xml = Instances::X2014a(X2014a::Abramson15).xml();

        // Format original xml
        let orig_formatted = tools::format_xml(&xml);

        // Parse xml to datastructure
        let data = crate::parse(&xml);

        // Data to string
        let data_str =
            quick_xml::se::to_string_with_root("HighSchoolTimetableArchive", &data).unwrap();

        // Format
        let data_str_formatted = tools::format_xml(&data_str);

        // Assert
        assert_eq!(orig_formatted, data_str_formatted)
    }

    #[test]
    fn parse_all_11() {
        // XML string
        let xml = Instances::X2014a(X2014a::All11).xml();

        // Format original xml
        let orig_formatted = tools::format_xml(&xml);

        // Parse xml to datastructure
        let data = crate::parse(&xml);

        // Data to string
        let data_str =
            quick_xml::se::to_string_with_root("HighSchoolTimetableArchive", &data).unwrap();

        // Format
        let data_str_formatted = tools::format_xml(&data_str);

        // Assert
        assert_eq!(orig_formatted, data_str_formatted)
    }

    // #[test]
    // fn parse_australia_bghs_98() {
    //     // XML string
    //     let xml = Instances::X2014(X2014::AustraliaBGHS98).xml();

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