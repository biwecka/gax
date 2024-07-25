// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Report {
    #[serde(rename = "InfeasibilityValue")]
    pub infeasibility_value: u32,

    #[serde(rename = "ObjectiveValue")]
    pub objective_value: u32,

    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,

    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Events>,

    #[serde(rename = "EventGroups", skip_serializing_if = "Option::is_none")]
    pub event_groups: Option<EventGroups>,
}

// Sub-Structs /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct Resources {
        #[serde(rename = "Resource", default)]
        pub list: Vec<pub struct Resource {
            // Attributes
            #[serde(rename = "@Reference", skip_serializing_if = "Option::is_none")]
            pub reference: Option<String>,

            // Children
            #[serde(rename = "Constraint", default)]
            pub constraints: Vec<pub struct Constraint {
                // Attributes
                #[serde(rename = "@Reference")]
                pub reference: String,

                // Children
                #[serde(rename = "Cost")]
                pub cost: u32,

                #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
                pub description: Option<String>,
            }>,

        }>,
    }
);

structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct Events {
        #[serde(rename = "Event", default)]
        pub list: Vec<pub struct Event {
            // Attributes
            #[serde(rename = "@Reference", skip_serializing_if = "Option::is_none")]
            pub reference: Option<String>,

            // Children
            #[serde(rename = "Constraint", default)]
            pub constraints: Vec<Constraint>,

        }>,
    }
);

structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct EventGroups {
        #[serde(rename = "EventGroup", default)]
        pub list: Vec<pub struct EventGroup {
            // Attributes
            #[serde(rename = "@Reference", skip_serializing_if = "Option::is_none")]
            pub reference: Option<String>,

            // Children
            #[serde(rename = "Constraint", default)]
            pub constraints: Vec<Constraint>,

        }>,
    }
);

////////////////////////////////////////////////////////////////////////////////
