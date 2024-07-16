// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Events {
    #[serde(rename = "EventGroups", skip_serializing_if = "Option::is_none")]
    pub event_groups: Option<EventGroups>,

    #[serde(rename = "Event", default)]
    pub events: Vec<Event>,
}

// Sub-Structs /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct EventGroups {
        #[serde(rename = "Course", default)]
        pub courses: Vec<pub struct Course {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,

        #[serde(rename = "EventGroup", default)]
        pub event_groups: Vec<pub struct EventGroup {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,
    }
);

structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct Event {
        // Attributes
        #[serde(rename = "@Id")]
        pub id: String,

        #[serde(rename = "@Color", skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,


        // Children
        #[serde(rename = "Name")]
        pub name: String,

        #[serde(rename = "Duration")]
        pub duration: u32,

        #[serde(rename = "Workload", skip_serializing_if = "Option::is_none")]
        pub workload: Option<u32>,

        #[serde(rename = "Course", skip_serializing_if = "Option::is_none")]
        pub course: Option<pub struct CourseRef {

            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

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
                #[serde(rename = "@Reference", skip_serializing_if = "Option::is_none")]
                pub reference: Option<String>,

                // Children
                #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
                pub role: Option<pub struct Role {
                    #[serde(rename = "$text")]
                    pub value: String,
                }>,

                #[serde(rename = "ResourceType", skip_serializing_if = "Option::is_none")]
                pub resource_type: Option<pub struct ResourceTypeRef {
                    #[serde(rename = "@Reference")]
                    pub reference: String,
                }>,

                #[serde(rename = "Workload", skip_serializing_if = "Option::is_none")]
                pub workload: Option<u32>,

            }>,
        }>,

        #[serde(rename = "ResourceGroups", skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<pub struct ResourceGroupRefs {

            #[serde(rename = "ResourceGroup", default)]
            pub list: Vec<pub struct ResourceGroupRef {

                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,

        #[serde(rename = "EventGroups", skip_serializing_if = "Option::is_none")]
        pub event_groups: Option<pub struct EventGroupRefs {

            #[serde(rename = "EventGroup", default)]
            pub list: Vec<pub struct EventGroupRef {

                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,
    }
);

////////////////////////////////////////////////////////////////////////////////
