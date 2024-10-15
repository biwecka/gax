// Imports /////////////////////////////////////////////////////////////////////
use super::chromosome::Chromosome;
use std::collections::HashSet;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Allocation {
    event_to_time: Vec<u8>,
    resource_to_events: Matrix2D,
}

impl Allocation {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let event_to_time: Vec<u8> = vec![0; db.events().len()];

        let mut resource_to_events =
            Matrix2D::init(db.resources().len(), db.events().len());

        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resource_to_events.set(resource_idx, event_idx, true);
            }
        }

        Self { event_to_time, resource_to_events }
    }

    /// Create a new allocation, by applying a chromosome to the current one.
    /// This does not mutate the current allocation. The current allocation
    /// is cloned, the chromosome is applied and then this new allocation is
    /// returned.
    pub fn derive(&self, chromosome: Chromosome) -> Self {
        let mut clone = self.clone();
        clone.event_to_time = chromosome.0;

        clone
    }

    /// Get the events (indices) that are assigned to the given resource.
    pub fn events_by_resource(&self, resource_idx: usize) -> Vec<usize> {
        self.resource_to_events
            .get_row(resource_idx)
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
    pub fn times_by_events(&self, event_idxs: &[usize]) -> Vec<u8> {
        let mut times: HashSet<u8> = HashSet::new();

        for event_idx in event_idxs {
            times.insert(self.event_to_time[*event_idx]);
        }

        times.into_iter().collect()
    }
}

// Helper Structs //////////////////////////////////////////////////////////////

/// A two-dimensional array (matrix).
/// Addressing a cell is row-first (e.g. matrix[1,4] = 1st row, 4th column)
/// and zero based!
#[derive(Clone, Debug)]
struct Matrix2D {
    rows: usize,    // m
    columns: usize, // n
    data: Vec<bool>,
}

impl Matrix2D {
    pub fn init(rows: usize, columns: usize) -> Self {
        Self { rows, columns, data: vec![false; rows * columns] }
    }

    #[allow(unused)]
    pub fn get(&self, row: usize, column: usize) -> bool {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index]
    }

    #[allow(unused)]
    pub fn set(&mut self, row: usize, column: usize, value: bool) {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index] = value;
    }

    #[allow(unused)]
    pub fn get_row(&self, row: usize) -> &[bool] {
        assert!(row < self.rows);

        let offset = row * self.columns;
        let end = offset + self.columns - 1;

        &self.data[offset..=end]
    }

    #[allow(unused)]
    pub fn get_col(&self, column: usize) -> Vec<bool> {
        assert!(column < self.columns);

        let mut indices = vec![];
        for i in (column..).step_by(self.columns) {
            if i >= self.data.len() {
                break;
            }
            indices.push(i);
        }

        let mut result = vec![];
        for i in indices {
            result.push(self.data[i]);
        }

        result
    }
}

////////////////////////////////////////////////////////////////////////////////
