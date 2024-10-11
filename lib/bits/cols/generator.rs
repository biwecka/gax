#[macro_export]
macro_rules! generate_cols {
    (
        $vis:vis struct $name:ident {
            type: $ty:ty,
            base: $bty:ty,
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $name {
            data: Vec<$ty>,

            rows: $bty,
            cols: usize,
        }

        impl $name {
            pub fn new(rows: $bty, cols: usize) -> Self {
                let data = vec![<$ty>::new(rows, 0); cols];

                Self { data, rows, cols }
            }

            pub fn rows(&self) -> $bty { self.rows }
            pub fn cols(&self) -> usize { self.cols }

            pub fn get(&self, col: usize, row: $bty) -> bool {
                assert!(col < self.cols);
                self.data[col].get(row)
            }

            pub fn set(&mut self, col: usize, row: $bty) {
                assert!(col < self.cols);
                self.data[col].set(row);
            }

            pub fn unset(&mut self, col: usize, row: $bty) {
                assert!(col < self.cols);
                self.data[col].unset(row);
            }

            pub fn col(&self, col: usize) -> &$ty {
                assert!(col < self.cols);
                &self.data[col]
            }

            pub fn col_mut(&mut self, col: usize) -> &mut $ty {
                assert!(col < self.cols);
                &mut self.data[col]
            }

            pub fn set_col(&mut self, col: usize, data: $ty) {
                assert!(col < self.cols);
                assert_eq!(data.len(), self.rows);
                self.data[col] = data;
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut rows: Vec<String> = vec![];

                for row in (0..self.rows).rev() {
                    let mut curr_row: Vec<String> = vec![];

                    for col in 0..self.cols {
                        if col > 0 && col % 4 == 0 {
                            curr_row.push(" ".into());
                        }

                        let val = self.data[col].get(row);

                        if val {
                            curr_row.push("1".into());

                        } else {
                            curr_row.push("0".into());
                        }
                    }

                    rows.push(curr_row.join(""));
                }

                write!(f, "{}", rows.join("\n"))
            }
        }
    }
}
