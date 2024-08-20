// Modules /////////////////////////////////////////////////////////////////////
pub mod events;
mod report;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Solution {
    // Attributes
    #[serde(rename = "@Reference")]
    pub reference: String,

    // Children
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "RunningTime", skip_serializing_if = "Option::is_none")]
    pub running_time: Option<String>,

    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<events::Events>,
    // #[serde(rename = "Report", skip_serializing_if = "Option::is_none")]
    // pub report: Option<report::Report>,
}

////////////////////////////////////////////////////////////////////////////////
