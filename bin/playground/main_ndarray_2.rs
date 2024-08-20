// Imports /////////////////////////////////////////////////////////////////////
use ndarray::{Array2, Axis};

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    let mut times = Array2::<u8>::default((6, 10));
    times[[1, 0]] = 1;
    times[[2, 0]] = 1;

    times[[0, 1]] = 1;

    times[[2, 2]] = 1;
    times[[3, 2]] = 1;

    let mut resources = Array2::<u8>::default((4, 10));
    resources[[2, 0]] = 1;
    resources[[2, 1]] = 1;
    resources[[2, 2]] = 1;
    resources[[2, 4]] = 1;

    println!("Times");
    print_matrix(&times);

    println!("Resources");
    print_matrix(&resources);

    // "events_by_resource"
    let resource_idx = 2;
    let res_row = resources.row(resource_idx);
    println!("Resource row");
    print_matrix(&res_row);

    // get columns of "times" by "res_row"
    let num_events = res_row.sum();
    println!("Number of events of this resource: {}\n", num_events);

    let mut matrix = Array2::<u8>::default((6, num_events as usize));

    let mut matrix_col_index = 0;
    for (i, val) in res_row.iter().enumerate() {
        if *val == 1 {
            let mut col = matrix.column_mut(matrix_col_index);
            col.assign(&times.column(i));
            matrix_col_index += 1;
        }
    }

    println!("Extracted columns from 'times' matrix:");
    print_matrix(&matrix);

    // Summarize
    let res = matrix.fold_axis(Axis(1), 0, |acc, x| acc + *x).map(|x| {
        if *x > 0 {
            x - 1
        } else {
            *x
        }
    });
    println!("Row sum:");
    print_matrix(&res);

    println!("-------- TEST ----------");

    let test = times.slice(ndarray::s![.., 0..2]);
    print_matrix(&test);
}

////////////////////////////////////////////////////////////////////////////////

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
