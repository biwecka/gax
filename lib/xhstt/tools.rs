// Imports /////////////////////////////////////////////////////////////////////
use std::path::Path;

use crate::parser::{
    solution_groups::{
        metadata::MetaData,
        solution::{
            events::{Event, Events},
            Solution,
        },
        SolutionGroup, SolutionGroups,
    },
    XhsttArchive,
};

// Functions ///////////////////////////////////////////////////////////////////

/// Serialize the [`XhsttArchive`] struct into an XML string.
pub fn xhstt_to_xml_string(xhstt: &XhsttArchive) -> String {
    quick_xml::se::to_string_with_root("HighSchoolTimetableArchive", xhstt)
        .unwrap()
}

/// Format the given XML string (pretty-printing).
pub fn format_xml(xml: &str) -> String {
    match xmltree::Element::parse(xml.as_bytes()) {
        Ok(element) => {
            let mut buffer = Vec::new();

            let config = xmltree::EmitterConfig {
                perform_indent: true,
                indent_string: "\t".into(),
                pad_self_closing: false,
                write_document_declaration: false,
                ..xmltree::EmitterConfig::default()
            };

            match element.write_with_config(&mut buffer, config) {
                Ok(_) => match String::from_utf8(buffer) {
                    Ok(formatted_xml) => formatted_xml,
                    Err(e) => panic!("error: {e:?}"),
                },

                Err(e) => panic!("error: {e:?}"),
            }
        }

        Err(e) => panic!("error: {e:?}"),
    }
}

/// Assemble the [`SolutionGroups`] data structure from a list of [`Event`]s
/// and the additionally provided information (e.g. instance_id, etc.).
pub fn create_solution(
    instance_id: &str,
    solution_id: &str,
    contributor: &str,
    description: &str,
    runtime: Option<usize>, // in seconds
    solution_events: Vec<Event>,
) -> SolutionGroups {
    let date = chrono::Local::now().to_rfc3339();

    SolutionGroups {
        list: vec![SolutionGroup {
            id: solution_id.to_owned(),
            metadata: MetaData {
                contributor: contributor.to_owned(),
                date,
                description: description.to_owned(),
                publication: None,
                remarks: None,
            },

            solutions: vec![Solution {
                reference: instance_id.to_owned(),
                description: None,
                running_time: runtime.map(|s| format!("{}",s)),
                events: Some(Events { list: solution_events }),
            }],
        }],
    }
}

/// This function writes the XHSTT archive given as parameter to disk by
/// performing the following steps:
/// 1.  Convert [`XhsttArchive`] to an XML string
/// 2.  Format the XML string (pretty-print).
/// 3.  Write formatted XML to disk.
pub fn write_xhstt(
    xhstt: &XhsttArchive,
    path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let xml = xhstt_to_xml_string(xhstt);
    let formatted_xml = format_xml(&xml);

    std::fs::write(path, formatted_xml)
}

////////////////////////////////////////////////////////////////////////////////
