// Modules /////////////////////////////////////////////////////////////////////
pub mod metadata;
pub mod solution;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SolutionGroups {
    #[serde(rename = "SolutionGroup")]
    pub list: Vec<SolutionGroup>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SolutionGroup {
    // Attributes
    #[serde(rename = "@Id")]
    pub id: String,

    // Children
    #[serde(rename = "MetaData")]
    pub metadata: metadata::MetaData,

    #[serde(rename = "Solution", default)]
    pub solutions: Vec<solution::Solution>,
}


////////////////////////////////////////////////////////////////////////////////
