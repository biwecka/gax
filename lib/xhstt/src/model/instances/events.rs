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
        /// In the same way as `Week` and `Day` are alternative forms for
        /// `TimeGroup`, `Course`s are an alternative form of `EventGroup`s.
        ///
        /// Events in the same `Course` constitute one course of study in one
        /// subject for one group of students (e.g.: math in class 9b).
        ///
        /// **Courses are optional.**
        #[serde(rename = "Course", default)]
        pub courses: Vec<pub struct Course {
            // Attributes
            #[serde(rename = "@Id")]
            pub id: String,


            // Children
            #[serde(rename = "Name")]
            pub name: String,
        }>,

        /// Generic event group.
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
    /// An event can have two meanings:
    /// 1. Event = lesson (but it can have a duration >1)
    /// 2. Event = course (which needs to be split into multiple lessons by the
    ///            solver/scheduler -> "SplitEventsConstraint")
    ///
    /// This means, that if there are events with a duration >1 **and**
    /// "SplitEventsConstraint"s are given, then the scheduler can split up
    /// these events.
    /// Otherwise, one event = one lesson (even if ducation >2) and they should
    /// not be split up.
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

        /// The `course` property specifies, that this event lies/is part of
        /// the respective course (which is a special kind of event group).
        #[serde(rename = "Course", skip_serializing_if = "Option::is_none")]
        pub course: Option<pub struct CourseRef {

            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

        /// The time (time slot) that has to be assigned to an event by the
        /// solver.
        /// If this is set, the a time was pre-assigned to this event.
        #[serde(rename = "Time", skip_serializing_if = "Option::is_none")]
        pub time: Option<pub struct TimeRef {

            #[serde(rename = "@Reference")]
            pub reference: String,
        }>,

        /// The `resources` and `resource_groups` categories specify, which
        /// resources need to attend the event.
        #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
        pub resources: Option<pub struct Resources {

            #[serde(rename = "Resource", default)]
            pub list: Vec<pub struct Resource {
                // Attributes

                /// When the `reference` attribute is absent, the solver is
                /// expected to assign a ressource to the event.
                /// In this case the `role` and `resource_type` must be set.
                ///
                /// If the `reference` attribute is set, it references a
                /// predefined resource which attends the event.
                /// The `role` and `resource_type` are optional in this case.
                #[serde(rename = "@Reference", skip_serializing_if = "Option::is_none")]
                pub reference: Option<String>,

                // Children

                /// Must be set, if `reference` is not set.
                #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
                pub role: Option<pub struct Role {
                    #[serde(rename = "$text")]
                    pub value: String,
                }>,

                /// Must be set, if `reference` is not set.
                #[serde(rename = "ResourceType", skip_serializing_if = "Option::is_none")]
                pub resource_type: Option<pub struct ResourceTypeRef {
                    #[serde(rename = "@Reference")]
                    pub reference: String,
                }>,

                #[serde(rename = "Workload", skip_serializing_if = "Option::is_none")]
                pub workload: Option<u32>,

            }>,
        }>,

        /// The `resources` and `resource_groups` categories specify, which
        /// resources need to attend the event.
        ///
        /// At present, this is only used for pre-assignments and not for
        /// signaling the solver to schedule/assign ressources.
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

// Implementations /////////////////////////////////////////////////////////////
impl Resource {
    pub fn is_preassigned(&self) -> bool {
        self.reference.is_some()
    }
}

////////////////////////////////////////////////////////////////////////////////
