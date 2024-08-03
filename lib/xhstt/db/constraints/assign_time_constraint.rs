// Imports /////////////////////////////////////////////////////////////////////
use super::utils::{AppliesToEventsAndGroups, CostFunction};
use crate::parser::instances::constraints::AssignTimeConstraint as IAssignTimeConstraint;

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct AssignTimeConstraint {
    pub id: String,
    pub name: String,
    pub required: bool,
    pub weight: u32,
    pub cost_function: CostFunction,
    pub applies_to: AppliesToEventsAndGroups,
}

impl From<IAssignTimeConstraint> for AssignTimeConstraint {
    fn from(value: IAssignTimeConstraint) -> Self {
        Self {
            id: value.id,
            name: value.name,
            required: value.required,
            weight: value.weight,
            cost_function: value.cost_function.into(),
            applies_to: value.applies_to.into(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
