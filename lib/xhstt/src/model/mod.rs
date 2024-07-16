// Modules /////////////////////////////////////////////////////////////////////
pub mod metadata;
pub mod instances;

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
}

////////////////////////////////////////////////////////////////////////////////


structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    // #[strikethrough[serde(rename = "lowercase")]]
    pub struct XhsttArchiveOld {
        #[serde(rename = "Instances")]
        pub instances: pub struct Instances {

            #[serde(rename = "Instance")]
            pub list: Vec<pub struct Instance {
                // Attributes
                #[serde(rename = "@Id")]
                pub id: String,

                // Metadata
                #[serde(rename = "MetaData", skip_serializing_if = "Option::is_none")]
                pub metadata: Option<pub struct Metadata {
                    #[serde(rename = "Name")]
                    name: String,

                    #[serde(rename = "Contributor")]
                    contributor: String,

                    #[serde(rename = "Date")]
                    date: String,

                    #[serde(rename = "Country")]
                    country: String,

                    #[serde(rename = "Description")]
                    description: String,

                    #[serde(rename = "Remarks", skip_serializing_if = "Option::is_none")]
                    remarks: Option<String>,
                }>,

                // Times
                #[serde(rename = "Times")]
                pub times: pub struct Times {
                    // Time groups
                    #[serde(rename = "TimeGroups")]
                    pub time_groups: pub struct TimeGroups {

                        #[serde(rename = "Week", skip_serializing_if = "Option::is_none")]
                        pub weeks: Option<Vec<pub struct Week {
                            #[serde(rename = "@Id")]
                            pub id: String,

                            #[serde(rename = "Name")]
                            pub name: String,

                            // TODO: add 'TimeGroups'?
                        }>>,

                        #[serde(rename = "Day", skip_serializing_if = "Option::is_none")]
                        pub days: Option<Vec<pub struct Day {
                            #[serde(rename = "@Id")]
                            pub id: String,

                            #[serde(rename = "Name")]
                            pub name: String,

                            // TODO: add 'TimeGroups'?
                        }>>,

                        #[serde(rename = "TimeGroup", skip_serializing_if = "Option::is_none")]
                        pub time_groups: Option<Vec<pub struct TimeGroup {
                            #[serde(rename = "@Id")]
                            pub id: String,

                            #[serde(rename = "Name")]
                            pub name: String,
                        }>>,
                    },

                    #[serde(rename = "Time")]
                    pub times: Vec<pub struct Time {
                        #[serde(rename = "@Id")]
                        pub id: String,

                        #[serde(rename = "Name")]
                        pub name: String,

                        #[serde(rename = "Day")]
                        pub day: pub struct DayRef {
                            #[serde(rename = "@Reference")]
                            pub reference: String,
                        },
                    }>,
                }

            }>,
        },
    }
);

