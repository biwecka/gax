use hashbrown::HashSet;
use xhstt::db::constraints::Constraint;

// Imports /////////////////////////////////////////////////////////////////////
use super::{context::Context, genotype::Chromosome, objective_value::Cost};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    event_to_time: Vec<usize>,
    resource_to_events: ndarray::Array2<bool>,
}

impl Phenotype {
    pub fn blueprint(db: &xhstt::db::Database, ctx: &Context) -> Self {
        let event_to_time: Vec<usize> = vec![0; db.events().len()];

        let mut resource_to_events = ndarray::Array2::<bool>::default((
            ctx.num_resources,
            ctx.num_events,
        ));
        // Matrix2D::init(db.resources().len(), db.events().len());

        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);

                resource_to_events[[resource_idx, event_idx]] = true;
            }
        }

        Self { event_to_time, resource_to_events }
    }

    /// Get the events (indices) that are assigned to the given resource.
    pub fn events_by_resource(&self, resource_idx: usize) -> Vec<usize> {
        self.resource_to_events
            // .get_row(resource_idx)
            .row(resource_idx)
            .iter()
            .enumerate()
            .filter_map(|(i, value)| match value {
                true => Some(i),
                false => None,
            })
            .collect()
    }

    /// Get all times that are allocated to the given events. Duplicates are
    /// removed.
    pub fn times_by_events(&self, event_idxs: &[usize]) -> Vec<usize> {
        let mut times: HashSet<usize> = HashSet::new();

        for event_idx in event_idxs {
            times.insert(self.event_to_time[*event_idx]);
        }

        times.into_iter().collect()
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromsome: &Chromosome) -> Self {
        let mut clone = self.clone();
        clone.event_to_time = chromsome.as_slice().to_vec();

        clone
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;
        for (constraint, indices) in &ctx.constraints {
            match constraint {
                Constraint::AssignTimeConstraint(_) => {}
                Constraint::AvoidClashesConstraint(x) => {
                    total_cost += super::constraints::avoid_clashes_constraint(
                        self, x, indices,
                    );
                }
            }
        }

        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
