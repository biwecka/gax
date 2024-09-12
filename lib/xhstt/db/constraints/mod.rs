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
    AssignResourceConstraint,
    AssignTimeConstraint(AssignTimeConstraint),
    SplitEventsConstraint,
    DistributeSplitEventsConstraint,
    PreferResourcesConstraint,
    PreferTimesConstraint,
    AvoidSplitAssignmentsConstraint,
    SpreadEventsConstraint,
    LinkEventsConstraint,
    OrderEventsConstraint,
    AvoidClashesConstraint(AvoidClashesConstraint),
    AvoidUnavailableTimesConstraint,
    LimitIdleTimesConstraint,
    ClusterBusyTimesConstraint,
    LimitBusyTimesConstraint,
    LimitWorkloadConstraint,
}

impl Constraint {
    pub fn is_required(&self) -> bool {
        match self {
            Self::AssignTimeConstraint(x) => x.required,
            Self::AvoidClashesConstraint(x) => x.required,

            // TODO
            _ => false,
        }
    }
}

impl From<IConstraint> for Constraint {
    fn from(value: crate::parser::instances::constraints::Constraint) -> Self {
        match value {
            IConstraint::AssignResourceConstraint(_) => {
                Self::AssignResourceConstraint
            }

            IConstraint::AssignTimeConstraint(data) => {
                Self::AssignTimeConstraint(data.into())
            }

            IConstraint::SplitEventsConstraint(_) => {
                Self::SplitEventsConstraint
            }
            IConstraint::DistributeSplitEventsConstraint(_) => {
                Self::DistributeSplitEventsConstraint
            }
            IConstraint::PreferResourcesConstraint(_) => {
                Self::PreferResourcesConstraint
            }
            IConstraint::PreferTimesConstraint(_) => {
                Self::PreferTimesConstraint
            }
            IConstraint::AvoidSplitAssignmentsConstraint(_) => {
                Self::AvoidSplitAssignmentsConstraint
            }
            IConstraint::SpreadEventsConstraint(_) => {
                Self::SpreadEventsConstraint
            }
            IConstraint::LinkEventsConstraint(_) => Self::LinkEventsConstraint,
            IConstraint::OrderEventsConstraint(_) => {
                Self::OrderEventsConstraint
            }

            IConstraint::AvoidClashesConstraint(data) => {
                Self::AvoidClashesConstraint(data.into())
            }

            IConstraint::AvoidUnavailableTimesConstraint(_) => {
                Self::AvoidUnavailableTimesConstraint
            }
            IConstraint::LimitIdleTimesConstraint(_) => {
                Self::LimitIdleTimesConstraint
            }
            IConstraint::ClusterBusyTimesConstraint(_) => {
                Self::ClusterBusyTimesConstraint
            }
            IConstraint::LimitBusyTimesConstraint(_) => {
                Self::LimitBusyTimesConstraint
            }
            IConstraint::LimitWorkloadConstraint(_) => {
                Self::LimitWorkloadConstraint
            }
        }
    }
}

// Helper Structs //////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
