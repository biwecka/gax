use ndarray::Array2;

fn main() {
    let mut matrix = Array2::<u8>::default((6, 10));
    matrix[[1, 0]] = 1;
    matrix[[2, 0]] = 1;

    matrix[[3, 1]] = 1;
    matrix[[5, 1]] = 1;

    matrix[[2, 2]] = 1;
    matrix[[3, 2]] = 1;

    print_matrix(&matrix);

    for (i, col) in matrix.columns().into_iter().enumerate() {
        println!("column = {}", i);

        let sum = col.sum();
        println!("sum    = {sum}");

        let continuous = 'x: {
            let mut prev = col.first().unwrap();
            let mut seq_end = false;

            for val in col.iter().skip(1) {
                // If we already observed the end of a 1s-sequence, and
                // are currently observing the start of another 1s-sequence,
                // we return false.
                if seq_end && prev == &0 && val == &1 {
                    break 'x false;
                }

                // As soon as we detect the end of a 1s-sequence, we set the
                // corresponding flag to true.
                if prev == &1 && val == &0 {
                    seq_end = true;
                }

                // Update "previous".
                prev = val;
            }

            true
        };

        println!("cont.  = {}", continuous);

        println!("");
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
