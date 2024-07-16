// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Times {
    #[serde(rename = "TimeGroups", skip_serializing_if = "Option::is_none")]
    pub time_groups: Option<TimeGroups>,

    #[serde(rename = "Time", default)]
    pub times: Vec<Time>,
}

// Sub-Structs /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct TimeGroups {
        #[serde(rename = "Week", default)]
        pub weeks: Vec<pub struct Week {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,

        #[serde(rename = "Day", default)]
        pub days: Vec<pub struct Day {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,

        #[serde(rename = "TimeGroup", default)]
        pub time_groups: Vec<pub struct TimeGroup {
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
    pub struct Time {
        // Attributes
        #[serde(rename = "@Id")]
        pub id: String,


        // Children
        #[serde(rename = "Name")]
        pub name: String,

        #[serde(rename = "Week", skip_serializing_if = "Option::is_none")]
        pub week: Option<pub struct WeekRef {
            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

        #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
        pub day: Option<pub struct DayRef {
            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

        #[serde(rename = "TimeGroups", skip_serializing_if = "Option::is_none")]
        pub time_groups: Option<pub struct TimeGroupRefs {

            #[serde(rename = "TimeGroup", default)]
            pub list: Vec<pub struct TimeGroupRef {

                #[serde(rename = "@Reference")]
                pub reference: String,
            }>
        }>,
    }
);

////////////////////////////////////////////////////////////////////////////////
