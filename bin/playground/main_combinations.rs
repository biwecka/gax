use itertools::Itertools;
use xhstt::xml::{Archives, X2014a};

fn main() {
    // Load XHSTT XML content.
    let xml = Archives::X2014a(X2014a::Abramson15).xml();

    // Parse XHSTT XML
    let xhstt = xhstt::parse(&xml);

    // Extract problem instance
    let instance = xhstt.instance().unwrap();

    // Create database
    let db = xhstt::db::Database::init(&instance).unwrap();

    let num_times = db.times().len();
    let num_events = db.events().len();

    // average events per time
    let avg_ept = ((num_events as f32) / (num_times as f32)).round() as usize;
    dbg!(&avg_ept);

    // Create resources helper
    let res = Resources::init(&db);

    for n in 15..16 {
        //=(avg_ept * 2 + 1) {
        println!("n             = {}", n);

        let mut non_conflicting = vec![];
        let mut counter = 0;

        for combination in (0..num_events).map(|x| x as u8).combinations(n) {
            let conflict = res.has_resource_conflict(
                &combination.iter().map(|x| *x as usize).collect::<Vec<_>>(),
            );
            if !conflict {
                non_conflicting.push(combination);
            }
            counter += 1;
            if counter % 1_000_000 == 0 {
                println!("{counter} (len: {})", non_conflicting.len());
            }
        }

        println!("non-conflicting combinations = {}", non_conflicting.len());
    }
}

pub struct Resources {
    pub matrix: ndarray::Array2<u8>,
}

impl Resources {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let mut matrix =
            ndarray::Array2::default((db.resources().len(), db.events().len()));

        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                matrix[[resource_idx, event_idx]] = 1;
            }
        }

        Self { matrix }
    }

    /// Add up multiple columns. This means the 1st elements of all columns are
    /// added, then the 2nd elements of the columns are added and so on.
    /// In other words: The columns are summed up by row.
    ///
    /// Example:  
    /// Col 0   : 1 0 0 2 1  
    /// Col 1   : 0 1 0 0 2  
    /// Result  : 4 3
    fn add_cols(&self, col_indices: &[usize]) -> Vec<u8> {
        // Create a temporary matrix, which will contain only the selected
        // rows from `self.matrix`.
        let mut tmp = ndarray::Array2::<u8>::default((
            self.matrix.shape()[0], // num_resources
            col_indices.len(),
        ));

        // Copy the selected rows (by `row_indices`) from `self.matrix` to
        // the temporary matrix
        for (tmp_index, matrix_index) in col_indices.iter().enumerate() {
            let mut col = tmp.column_mut(tmp_index);
            col.assign(&self.matrix.column(*matrix_index));
        }

        // Combine the rows by "and"
        let result = tmp.fold_axis(ndarray::Axis(1), 0, |acc, x| *acc + *x);
        result.to_vec()
    }

    pub fn has_resource_conflict(&self, event_ids: &[usize]) -> bool {
        // Calculate collision vector
        let collision_vector = self.add_cols(event_ids);

        // If the collision vector contains a number larger than 1, there is
        // resource conflict between these events.
        collision_vector.iter().any(|x| *x > 1)
    }
}

fn print_matrix<S, D>(matrix: &ndarray::ArrayBase<S, D>)
where
    S: ndarray::Data<Elem = u8>,
    D: ndarray::Dimension + ndarray::RemoveAxis,
{
    match matrix.ndim() {
        1 => {
            // let length = matrix.shape()[0];
            let mut s = String::from("["); //format!("Matrix 1 x {}: [", length);
            for item in matrix.iter() {
                if *item > 0 {
                    s += &format!(" {}", item);
                } else {
                    s += " ·";
                }
            }

            s += " ]\n";
            println!("{}", s);
        }

        2 => {
            // let rows = matrix.shape()[0];
            // let columns = matrix.shape()[1];

            // let mut s = format!("Matrix {} x {}:\n", rows, columns);
            let mut s = String::from("");

            for row in matrix.outer_iter() {
                s += "[";
                for item in row.iter() {
                    if *item > 0 {
                        s += &format!(" {}", item);
                    } else {
                        s += " ·";
                    }
                }
                s += " ]\n";
            }

            println!("{}", s);
        }

        _ => println!("{}", matrix),
    }
}
