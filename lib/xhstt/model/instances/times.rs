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
        /// Weeks allow display software to determine how the times of an
        /// instance are grouped into weeks.
        /// **Specifying weeks is optional.**
        ///
        /// Weeks are usually only specified, if schedules change for example
        /// on a two-weekly basis.
        #[serde(rename = "Week", default)]
        pub weeks: Vec<pub struct Week {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,

        /// Days allow display software to determine how the times of an
        /// instance are grouped into days.
        /// **Specifying days is optional.**
        ///
        /// Days are usually specified, to define the days of the week.
        #[serde(rename = "Day", default)]
        pub days: Vec<pub struct Day {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,


        /// Time groups define generic groups of times. Even though days and
        /// weeks are also time groups, they are usually defined in the
        /// respective, separate property (above).
        ///
        /// An example for the usage of the generic time groups is splitting
        /// a day in "before lunch" and "after lunch".
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
