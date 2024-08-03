// Imports /////////////////////////////////////////////////////////////////////
use super::utils::{AppliesToResourcesAndGroups, CostFunction};
use crate::parser::instances::constraints::AvoidClashesConstraint as IAvoidClashesConstraint;

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct AvoidClashesConstraint {
    pub id: String,
    pub name: String,
    pub required: bool,
    pub weight: u32,
    pub cost_function: CostFunction,
    pub applies_to: AppliesToResourcesAndGroups,
}

impl From<IAvoidClashesConstraint> for AvoidClashesConstraint {
    fn from(value: IAvoidClashesConstraint) -> Self {
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
