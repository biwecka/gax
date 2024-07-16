// Modules /////////////////////////////////////////////////////////////////////

// Imports /////////////////////////////////////////////////////////////////////

// Main ////////////////////////////////////////////////////////////////////////

use quick_xml::events::Event;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Constraints {
    #[serde(rename = "$value", default)]
    pub list: Vec<Constraint>
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Constraint {
    #[serde(rename = "a")]
    A(ConstA),

    #[serde(rename = "b")]
    B(ConstB),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConstA {
    #[serde(rename = "@id")]
    pub id: String,

    pub x: i32,
    pub y: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConstB {
    #[serde(rename = "@id")]
    pub id: String,

    pub z: Z
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Z {
    #[serde(rename = "@id")]
    pub id: i32
}

fn main() {
    let xml_raw = xhstt::data::Instances::Test.xml();

    let parsed = xhstt::parse(&xml_raw);
    // println!("{parsed:#?}");

    let xml_new = quick_xml::se::to_string_with_root("HighSchoolTimetableArchive", &parsed).unwrap();
    let formatted = xhstt::tools::format_xml(&xml_new);
    std::fs::write("./new.xml", formatted).unwrap();


    // let xml = r#"
    // <constraints>
    //     <a id="const_a">
    //         <x>123</x>
    //         <y>Hallo</y>
    //     </a>

    //     <b id="const_b">
    //         <z id="1" />
    //     </b>

    //     <c>
    //         <asdf>asdf</asdf>
    //     </c>
    // </constraints>
    // "#;

    // let parsed = quick_xml::de::from_str::<Constraints>(&xml);
    // println!("{parsed:#?}");

}

////////////////////////////////////////////////////////////////////////////////
