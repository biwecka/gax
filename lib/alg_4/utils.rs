/// A two-dimensional array (matrix).
/// Addressing a cell is row-first (e.g. matrix[1,4] = 1st row, 4th column)
/// and zero based!
#[derive(Clone, Debug)]
pub struct Matrix2D<T: Copy + Default> {
    rows: usize,    // m
    columns: usize, // n
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix2D<T> {
    pub fn init(rows: usize, columns: usize) -> Self {
        Self { rows, columns, data: vec![T::default(); rows * columns] }
    }

    #[allow(unused)]
    pub fn get_cell(&self, row: usize, column: usize) -> T {
        assert!(row < self.rows);
        assert!(column < self.columns);

        let offset = row * self.columns;
        let index = offset + column;

        self.data[index]
    }

    #[allow(unused)]
    pub fn set_cell(&mut self, row: usize, column: usize, value: T) {
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
    #[allow(unused)]
    pub fn and_rows(&self) -> Vec<bool> {
        let mut result = self.get_row(0).to_vec();

        for row in 1..self.rows {
            for col in 0..self.columns {
                result[col] = result[col] && self.get_cell(row, col);
            }
        }

        result
    }

    #[allow(unused)]
    pub fn or_rows(&self) -> Vec<bool> {
        let mut result = self.get_row(0).to_vec();

        for row in 1..self.rows {
            for col in 0..self.columns {
                result[col] = result[col] || self.get_cell(row, col);
            }
        }

        result
    }
}

use xhstt::parser::solution_groups::solution::events::{
    Event as SolutionEvent, TimeRef,
};

pub fn create_from(
    chromosome: &super::Chromosome,
    db: &xhstt::db::Database,
) -> Vec<SolutionEvent> {
    let mut events = vec![];

    for (event_idx, time_idx) in chromosome.iter().enumerate() {
        // Get event and time
        let event = db.event_by_idx(event_idx);
        let time = db.time_by_idx(*time_idx as usize);

        // Create event
        events.push(SolutionEvent {
            reference: event.id.0.clone(),
            duration: None,
            resources: None,
            time: Some(TimeRef { reference: time.id.0.clone() }),
        });
    }

    // Return
    events
}
