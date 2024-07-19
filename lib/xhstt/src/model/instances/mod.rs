// Modules /////////////////////////////////////////////////////////////////////
pub mod constraints;
pub mod events;
pub mod metadata;
pub mod resources;
pub mod times;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Instances {
    #[serde(rename = "Instance")]
    pub list: Vec<Instance>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Instance {
    // Attributes
    #[serde(rename = "@Id")]
    pub id: String,

    // Children
    #[serde(rename = "MetaData")]
    pub metadata: metadata::MetaData,

    #[serde(rename = "Times")]
    pub times: times::Times,

    #[serde(rename = "Resources")]
    pub resources: resources::Resources,

    #[serde(rename = "Events")]
    pub events: events::Events,

    #[serde(rename = "Constraints")]
    pub constraints: constraints::Constraints,
}

////////////////////////////////////////////////////////////////////////////////
