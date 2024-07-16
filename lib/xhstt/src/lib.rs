// Modules /////////////////////////////////////////////////////////////////////
pub mod data;
pub mod model;

// Functions ///////////////////////////////////////////////////////////////////
pub fn parse(xml: &str) -> model::XhsttArchive {
    quick_xml::de::from_str(xml).unwrap()
}

pub mod tools {
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

                    Err(e) => panic!("errof: {e:?}"),
                }
            },

            Err(e) => panic!("error: {e:?}"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
