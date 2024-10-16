// Imports /////////////////////////////////////////////////////////////////////
use super::chromosome::Chromosome;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Allocation {
    times: Matrix2D<i8>,
    resources: Matrix2D<bool>,

    duration: Vec<u32>,
}

impl Allocation {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let num_events = db.events().len();
        let num_times = db.times().len();
        let num_resources = db.resources().len();

        let times = Matrix2D::init(num_times, num_events);
        let mut resources = Matrix2D::init(num_resources, num_events);
        let duration = db.events().iter().map(|e| e.duration).collect();

        // Fill resources
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resources.set(resource_idx, event_idx, true);
            }
        }

        // Return
        Self { times, resources, duration }
    }

    /// Create a new allocation by applying a chromosome to the current one.
    /// This does not mutate the current allocation. The current allocation
    /// is cloned, the chromosome is applied and then this new allocation is
    /// returned.
    pub fn derive(&self, chromosome: &Chromosome) -> Self {
        let mut a = self.clone();

        for event_idx in &chromosome.0 {
            // Get allocated resources
            let resource_idxs = a
                .resources
                .get_col(*event_idx as usize)
                .into_iter()
                .enumerate()
                .filter_map(
                    |(i, value)| {
                        if value {
                            Some(i as u8)
                        } else {
                            None
                        }
                    },
                )
                .collect::<Vec<u8>>();

            // Create collision vector. This vector combines events the
            // event (-> event_idx) is related to through resource allocations.
            let mut matrix =
                Matrix2D::<bool>::init(resource_idxs.len(), chromosome.0.len());
            for (i, resource_idx) in resource_idxs.iter().enumerate() {
                matrix.set_row(i, a.resources.get_row(*resource_idx as usize));
            }

            let collision_vector = matrix.or_rows();

            // Get duration of event
            let duration = a.duration[*event_idx as usize] as usize;

            // Get times available for allocation.
            let time_groups = a
                .times
                .get_col(*event_idx as usize)
                .iter()
                .enumerate()
                .map(|(i, value)| (i as u8, *value))
                .collect::<Vec<(u8, i8)>>()
                .windows(duration)
                //                   time_idx, value (-1, 0, 1)
                .filter_map(|window: &[(u8, i8)]| {
                    // All values in the window must be 0, for the window to
                    // be allocatable.
                    let values: Vec<i8> =
                        window.iter().map(|(_, val)| *val).collect();
                    if values.contains(&-1) || values.contains(&1) {
                        None
                    } else {
                        let indices: Vec<u8> =
                            window.iter().map(|(i, _val)| *i).collect();
                        Some(indices)
                    }
                })
                .collect::<Vec<Vec<u8>>>();

            if time_groups.is_empty() {
                continue;
            }

            // TIMES: ...
            let mut time_groups_e: Vec<(Vec<u8>, usize)> = vec![];

            'tg_loop: for time_group in time_groups {
                let mut matrix = Matrix2D::<bool>::init(2, chromosome.0.len());

                let mut efficiency = 0;

                for time_idx in &time_group {
                    // Get row of the current time_idx from the time matrix
                    let time_row = a.times.get_row(*time_idx as usize);
                    let time_alloc =
                        time_row.iter().map(|x| *x > 0).collect::<Vec<bool>>();

                    matrix.set_row(0, &time_alloc);
                    matrix.set_row(1, &collision_vector);

                    let result = matrix.and_rows();

                    // Check result to be 000000 (all values = false)
                    if result.contains(&true) {
                        continue 'tg_loop; // this time does cannot be
                                           // assigned to the event
                    }

                    // Calculate the efficiency of the allocation in this
                    // timeslot.
                    let time_blocked_slots =
                        time_row.iter().map(|x| *x < 0).collect::<Vec<bool>>();

                    // Re-use the matrix from above
                    matrix.set_row(0, &time_blocked_slots);
                    matrix.set_row(1, &collision_vector);

                    let t_efficiency = matrix
                        .and_rows()
                        .iter()
                        .map(|x| if *x { 1 } else { 0 })
                        .sum::<usize>();

                    // times.push((time_idx, efficiency));
                    efficiency += t_efficiency;
                }

                // Add to time_groups_e
                time_groups_e.push((time_group, efficiency));
            }

            // Sort the times by their efficiency (descendingly)
            time_groups_e
                .sort_by_key(|(_, efficiency)| std::cmp::Reverse(*efficiency));

            // Allocate the highest ranked time to the allocation.
            if let Some((time_idxs, _)) = time_groups_e.first() {
                for time_idx in time_idxs {
                    for (i, collision) in collision_vector.iter().enumerate() {
                        if !collision {
                            continue;
                        }

                        if i == (*event_idx as usize) {
                            a.times.set(*time_idx as usize, i, 1);
                        } else {
                            a.times.set(*time_idx as usize, i, -1);
                        }
                    }
                }
            }
        }

        // Return
        a
    }

    pub fn times_by_event(&self, event_idx: usize) -> Vec<i8> {
        self.times.get_col(event_idx)
    }

    pub fn event_duration(&self, event_idx: usize) -> u32 {
        assert!(event_idx < self.duration.len());
        self.duration[event_idx]
    }
}

// Helper Structs //////////////////////////////////////////////////////////////

/// A two-dimensional array (matrix).
/// Addressing a cell is row-first (e.g. matrix[1,4] = 1st row, 4th column)
/// and zero based!
#[derive(Clone, Debug)]
struct Matrix2D<T: Copy + Default> {
    rows: usize,    // m
    columns: usize, // n
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix2D<T> {
    pub fn init(rows: usize, columns: usize) -> Self {
        Self { rows, columns, data: vec![T::default(); rows * columns] }
    }

    #[allow(unused)]
    pub fn get(&self, row: usize, column: usize) -> T {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index]
    }

    #[allow(unused)]
    pub fn set(&mut self, row: usize, column: usize, value: T) {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index] = value;
    }

    #[allow(unused)]
    pub fn set_row(&mut self, row: usize, values: &[T]) {
        assert!(row < self.rows);
        assert_eq!(values.len(), self.columns);

        let offset = row * self.columns;
        for (i, val) in values.iter().enumerate() {
            self.data[offset + i] = *val;
        }
    }

    #[allow(unused)]
    pub fn get_row(&self, row: usize) -> &[T] {
        assert!(row < self.rows);

        let offset = row * self.columns;
        let end = offset + self.columns - 1;

        &self.data[offset..=end]
    }

    #[allow(unused)]
    pub fn get_col(&self, column: usize) -> Vec<T> {
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

impl Matrix2D<bool> {
    /// Combines all rows by applying the "and" operation.
    pub fn and_rows(&self) -> Vec<bool> {
        let mut result = self.get_row(0).to_vec();

        for row in 1..self.rows {
            #[allow(clippy::needless_range_loop)]
            for col in 0..self.columns {
                result[col] = result[col] && self.get(row, col);
            }
        }

        result
    }

    pub fn or_rows(&self) -> Vec<bool> {
        let mut result = self.get_row(0).to_vec();

        for row in 1..self.rows {
            #[allow(clippy::needless_range_loop)]
            for col in 0..self.columns {
                result[col] = result[col] || self.get(row, col);
            }
        }

        result
    }
}

// Tests ///////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::Matrix2D;

    #[test]
    fn matrix_1() {
        // t t f f
        // f t f t
        let mut matrix = Matrix2D::<bool>::init(2, 4);
        matrix.set(0, 0, true);
        matrix.set(0, 1, true);
        matrix.set(0, 2, false);
        matrix.set(0, 3, false);

        matrix.set(1, 0, false);
        matrix.set(1, 1, true);
        matrix.set(1, 2, false);
        matrix.set(1, 3, true);

        assert_eq!(matrix.get_row(0), vec![true, true, false, false]);
        assert_eq!(matrix.get_row(1), vec![false, true, false, true]);

        assert_eq!(matrix.get_col(0), vec![true, false]);
        assert_eq!(matrix.get_col(1), vec![true, true]);
        assert_eq!(matrix.get_col(2), vec![false, false]);
        assert_eq!(matrix.get_col(3), vec![false, true]);
    }

    #[test]
    fn matrix_and_or_cols() {
        // t t f f
        // f t f t
        let mut matrix = Matrix2D::<bool>::init(2, 4);
        matrix.set(0, 0, true);
        matrix.set(0, 1, true);
        matrix.set(0, 2, false);
        matrix.set(0, 3, false);

        matrix.set(1, 0, false);
        matrix.set(1, 1, true);
        matrix.set(1, 2, false);
        matrix.set(1, 3, true);

        assert_eq!(matrix.and_rows(), vec![false, true, false, false]);
        assert_eq!(matrix.or_rows(), vec![true, true, false, true]);
    }

    #[test]
    fn matrix_set_row() {
        // t t f f
        // f t f t
        let mut matrix = Matrix2D::<bool>::init(2, 4);
        matrix.set(0, 0, true);
        matrix.set(0, 1, true);
        matrix.set(0, 2, false);
        matrix.set(0, 3, false);

        matrix.set(1, 0, false);
        matrix.set(1, 1, true);
        matrix.set(1, 2, false);
        matrix.set(1, 3, true);

        matrix.set_row(1, &[false, false, true, false]);

        assert_eq!(matrix.get_row(1), vec![false, false, true, false]);
    }
}

////////////////////////////////////////////////////////////////////////////////
