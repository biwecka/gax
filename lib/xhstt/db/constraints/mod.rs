// Modules /////////////////////////////////////////////////////////////////////
pub mod assign_time_constraint;
pub mod avoid_clashes_constraint;
pub mod utils;

// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::constraints::Constraint as IConstraint;
use assign_time_constraint::AssignTimeConstraint;
use avoid_clashes_constraint::AvoidClashesConstraint;

// Constraints /////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub enum Constraint {
    // AssignResourceConstraint,
    AssignTimeConstraint(AssignTimeConstraint),
    // SplitEventsConstraint,
    // DistributeSplitEventsConstraint,
    // PreferResourcesConstraint,
    // PreferTimesConstraint,
    // AvoidSplitAssignmentsConstraint,
    // SpreadEventsConstraint,
    // LinkEventsConstraint,
    // OrderEventsConstraint,
    AvoidClashesConstraint(AvoidClashesConstraint),
    // AvoidUnavailableTimesConstraint,
    // LimitIdleTimesConstraint,
    // ClusterBusyTimesConstraint,
    // LimitBusyTimesConstraint,
    // LimitWorkloadConstraint,
}

impl Constraint {
    pub fn is_required(&self) -> bool {
        match self {
            Self::AssignTimeConstraint(x) => x.required,
            Self::AvoidClashesConstraint(x) => x.required,
        }
    }
}

impl From<IConstraint> for Constraint {
    fn from(value: crate::parser::instances::constraints::Constraint) -> Self {
        match value {
            IConstraint::AssignTimeConstraint(data) => {
                Self::AssignTimeConstraint(data.into())
            }

            IConstraint::AvoidClashesConstraint(data) => {
                Self::AvoidClashesConstraint(data.into())
            }
        }
    }
}

// Helper Structs //////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
