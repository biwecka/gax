#[macro_export]
macro_rules! generate_rows {
    (
        $vis:vis struct $name:ident {
            type: $ty:ty,
            base: $bty:ty,
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $name {
            data: Vec<$ty>,

            rows: usize,
            cols: $bty,
        }

        impl $name {
            pub fn new(rows: usize, cols: $bty) -> Self {
                let data = vec![<$ty>::new(cols, 0); rows];
                Self { data, rows, cols }
            }

            pub fn rows(&self) -> usize { self.rows }
            pub fn cols(&self) -> $bty { self.cols }

            pub fn get(&self, row: usize, col: $bty) -> bool {
                assert!(row < self.rows);
                self.data[row].get(col)
            }

            pub fn set(&mut self, row: usize, col: $bty) {
                assert!(row < self.rows);
                self.data[row].set(col);
            }

            pub fn unset(&mut self, row: usize, col: $bty) {
                assert!(row < self.rows);
                self.data[row].unset(col);
            }

            pub fn row(&self, row: usize) -> & $ty {
                assert!(row < self.rows);
                &self.data[row]
            }

            pub fn row_mut(&mut self, row: usize) -> &mut $ty {
                assert!(row < self.rows);
                &mut self.data[row]
            }

            pub fn set_row(&mut self, row: usize, data: $ty) {
                assert!(row < self.rows);
                assert_eq!(data.len(), self.cols);
                self.data[row] = data;
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let rows = self
                    .data
                    .iter()
                    .map(|r| format!("{r}")).collect::<Vec<_>>();

                write!(f, "{}", rows.join("\n"))
            }
        }

        // impl std::fmt::Debug for $name {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         f.debug_struct(stringify!($name))
        //             .field("rows", &self.rows)
        //             .field("cols", &self.cols)
        //             .field("data", &self.data)
        //             .finish()
        //     }
        // }
    }
}
