// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Events {
    #[serde(rename = "Event", default)]
    pub list: Vec<Event>,
}

// Sub-Structs /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct Event {
        // Attributes
        #[serde(rename = "@Reference")]
        pub reference: String,

        // Children
        #[serde(rename = "Duration", skip_serializing_if = "Option::is_none")]
        pub duration: Option<u32>,

        #[serde(rename = "Time", skip_serializing_if = "Option::is_none")]
        pub time: Option<pub struct TimeRef {

            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

        #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
        pub resources: Option<pub struct Resources {

            #[serde(rename = "Resource", default)]
            pub list: Vec<pub struct Resource {
                // Attributes
                #[serde(rename = "@Reference")]
                pub reference: String,

                // Children
                #[serde(rename = "Role")]
                pub role: pub struct Role {
                    #[serde(rename = "$text")]
                    pub value: String,
                },

            }>,
        }>,
    }
);

////////////////////////////////////////////////////////////////////////////////
