// Modules /////////////////////////////////////////////////////////////////////
pub mod metadata;
pub mod instances;
pub mod solution_groups;

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct XhsttArchive {
    // Attributes
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Children
    #[serde(rename = "MetaData", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<metadata::MetaData>,

    #[serde(rename = "Instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<instances::Instances>,

    #[serde(rename = "SolutionGroups", skip_serializing_if = "Option::is_none")]
    pub solution_groups: Option<solution_groups::SolutionGroups>,
}

////////////////////////////////////////////////////////////////////////////////
