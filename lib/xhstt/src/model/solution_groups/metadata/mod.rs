// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetaData {
    #[serde(rename = "Contributor")]
    pub contributor: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Publication", skip_serializing_if = "Option::is_none")]
    pub publication: Option<String>,

    #[serde(rename = "Remarks", skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}
////////////////////////////////////////////////////////////////////////////////
