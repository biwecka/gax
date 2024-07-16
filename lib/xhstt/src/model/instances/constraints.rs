// Imports /////////////////////////////////////////////////////////////////////

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Constraints {
    #[serde(rename = "$value", default)]
    pub list: Vec<Constraint>,
}

// Sub /////////////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub enum Constraint {
        // AssignResourceConstraint,

        AssignTimeConstraint(pub struct {
            #[serde(rename = "@Id")]
            pub id: String,

            #[serde(rename = "Name")]
            pub name: String,

            #[serde(rename = "Required")]
            pub required: bool,

            #[serde(rename = "Weight")]
            pub weight: u32, // [0, 1000]

            #[serde(rename = "CostFunction")]
            pub cost_function: CostFunction,

            #[serde(rename = "AppliesTo")]
            pub applies_to: AppliesToEventsAndGroups,
        }),

        // SplitEventsConstraint,
        // DistributeSplitEventsConstraint,
        // PreferResourcesConstraint,
        // PreferTimesConstraint,
        // AvoidSplitAssignmentsConstraint,
        // SpreadEventsConstraint,
        // LinkEventsConstraint,
        // OrderEventsConstraint,

        AvoidClashesConstraint(pub struct {
            #[serde(rename = "@Id")]
            pub id: String,

            #[serde(rename = "Name")]
            pub name: String,

            #[serde(rename = "Required")]
            pub required: bool,

            #[serde(rename = "Weight")]
            pub weight: u32, // [0, 1000]

            #[serde(rename = "CostFunction")]
            pub cost_function: CostFunction,

            #[serde(rename = "AppliesTo")]
            pub applies_to: AppliesToResourcesAndGroups,
        }),

        // AvoidUnavailableTimesConstraint,
        // LimitIdleTimesConstraint,
        // ClusterBusyTimesConstraint,
        // LimitBusyTimesConstraint,
        // LimitWorkloadConstraint,
    }
);


// Sub Sub /////////////////////////////////////////////////////////////////////

// Const Function
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CostFunction {
    Linear,
    Quadratic,
    Step,
}

// AppliesToEventsAndGroups (event groups and events)
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct AppliesToEventsAndGroups {
        #[serde(rename = "EventGroups", skip_serializing_if = "Option::is_none")]
        pub event_groups: Option<pub struct EventGroupRefs {

            #[serde(rename = "EventGroup", default)]
            pub list: Vec<pub struct EventGroupRef {
                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,

        #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
        pub events: Option<pub struct EventRefs {

            #[serde(rename = "Event", default)]
            pub list: Vec<pub struct EventRef {
                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,
    }
);


// AppliesToEventGroups (event groups only)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppliesToEventGroups {
    #[serde(rename = "EventGroups")]
    pub event_groups: EventGroupRefs,
}

// AppliesToEventPairs (event pairs)
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct AppliesToEventPairs {
        #[serde(rename = "EventPairs")]
        pub event_pairs: pub struct EventPairs {

            #[serde(rename = "EventPair", default)]
            pub list: Vec<pub struct EventPair {

                #[serde(rename = "FirstEvent")]
                pub first_event: pub struct {
                    #[serde(rename = "@Reference")]
                    pub reference: String,
                },

                #[serde(rename = "SecondEvent")]
                pub second_event: pub struct {
                    #[serde(rename = "@Reference")]
                    pub reference: String,
                },

                #[serde(rename = "MinSeparation", skip_serializing_if = "Option::is_none")]
                pub min_separation: Option<u32>,

                #[serde(rename = "MaxSeparation", skip_serializing_if = "Option::is_none")]
                pub max_separation: Option<u32>,

            }>,
        },
    }
);


// AppliesToResourcesAndGroups (resource groups and events)
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]]
    pub struct AppliesToResourcesAndGroups {
        #[serde(rename = "ResourceGroups", skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<pub struct ResourceGroupRefs {

            #[serde(rename = "ResourceGroup", default)]
            pub list: Vec<pub struct ResourceGroupRef {
                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,

        #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
        pub resources: Option<pub struct ResourceRefs {

            #[serde(rename = "Resource", default)]
            pub list: Vec<pub struct ResourceRef {
                #[serde(rename = "@Reference")]
                pub reference: String,
            }>,
        }>,
    }
);




////////////////////////////////////////////////////////////////////////////////
