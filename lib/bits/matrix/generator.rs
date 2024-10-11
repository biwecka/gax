#[macro_export]
macro_rules! generate_matrix {
    (
        $vis:vis struct $name:ident {
            col_type: $col_ty:ty,
            col_base: $col_base:ty,

            row_type: $row_ty:ty,
            row_base: $row_base:ty,
        }
    ) => {
        #[derive(Debug)]
        $vis struct $name {
            row_data: Vec<$row_ty>,
            col_data: Vec<$col_ty>,

            rows: $col_base,
            cols: $row_base,
        }

        impl $name {
            pub fn new(rows: $col_base, cols: $row_base) -> Self {
                let row_data = vec![<$row_ty>::new(cols, 0); rows as usize];
                let col_data = vec![<$col_ty>::new(rows, 0); cols as usize];

                Self { row_data, col_data, rows, cols }
            }

            pub fn num_rows(&self) -> $col_base { self.rows }
            pub fn num_cols(&self) -> $row_base { self.cols }

            pub fn rows(&self) -> &[$row_ty] { &self.row_data }
            pub fn cols(&self) -> &[$col_ty] { &self.col_data }

            #[inline]
            fn conv_row_index(&self, row: $col_base) -> usize {
                self.row_data.len() - (row as usize) - 1
            }

            #[inline]
            fn conv_col_index(&self, col: $row_base) -> usize {
                self.col_data.len() - (col as usize) - 1
            }

            /// Get a single bit value in the matrix.
            pub fn get(&self, row: $col_base, col: $row_base) -> bool {
                assert!(row < self.rows);
                assert!(col < self.cols);

                let c_row = self.conv_row_index(row);

                self.row_data[c_row].get(col)
            }

            /// Set a single bit value in the matrix.
            pub fn set(&mut self, row: $col_base, col: $row_base) {
                assert!(row < self.rows);
                assert!(col < self.cols);

                let c_row = self.conv_row_index(row);
                let c_col = self.conv_col_index(col);

                self.row_data[c_row].set(col);
                self.col_data[c_col].set(row);
            }

            /// Get a whole row from the matrix.
            pub fn row(&self, row: $col_base) -> & $row_ty {
                assert!(row < self.rows);
                let c_row = self.conv_row_index(row);
                &self.row_data[c_row]
            }

            /// Get a whole column from the matrix.
            pub fn col(&self, col: $row_base) -> & $col_ty {
                assert!(col < self.cols);
                let c_col = self.conv_col_index(col);
                &self.col_data[c_col]
            }

            /// Updates a whole row in the matrix.
            pub fn set_row(&mut self, row: $col_base, data: $row_ty) {
                assert!(row < self.rows);
                assert_eq!(data.len(), self.cols);

                // Update row
                let c_row = self.conv_row_index(row);
                self.row_data[c_row] = data;

                // Update columns
                for c in 0..self.num_cols() {
                    let c_col = self.conv_col_index(c);

                    if data.get(c) {
                        self.col_data[c_col].set(row);

                    } else {
                        self.col_data[c_col].unset(row);
                    }
                }
            }

            /// Update a whole column in the matrix.
            pub fn set_col(&mut self, col: $row_base, data: $col_ty) {
                assert!(col < self.cols);
                assert_eq!(data.len(), self.rows);

                // Update col
                let c_col = self.conv_col_index(col);
                self.col_data[c_col] = data;

                // Update rows
                for r in 0..self.num_rows() {
                    let c_row = self.conv_row_index(r);

                    if data.get(r) {
                        self.row_data[c_row].set(col);

                    } else {
                        self.row_data[c_row].unset(col);
                    }
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let rows = self
                    .row_data
                    .iter()
                    .map(|r| format!("{r}")).collect::<Vec<_>>();

                write!(f, "{}", rows.join("\n"))
            }
        }
    }
}
