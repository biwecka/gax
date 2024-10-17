use ndarray::{arr1, arr2, s, Array2, ArrayBase};
use std::ops::BitOrAssign;

fn main() {
    let m = 3;
    let n = 8;

    let mut a = Array2::<bool>::default((m, n));
    print_array(&a);

    a.slice_mut(s![1, ..])
        .assign(&arr1(&[false, true, true, false, true, false, false, true]));

    a.slice_mut(s![0, ..])
        .assign(&arr1(&[false, false, false, false, false, true, true, false]));

    print_array(&a);

    // let x = arr1(&[true, true, false, false, true, false, true, false]);

    let clone = a.clone();

    let r0 = clone.row(0).to_owned();

    a.slice_mut(s![1, ..]).bitor_assign(&r0);

    print_array(&a);

    let indices: Vec<_> = a
        .row(0)
        .indexed_iter()
        .filter_map(|(i, &val)| if val { Some(i) } else { None })
        .collect();
    dbg!(&indices);

    // a.slice_mut(s![.., ..]).assign(
    //     &Array2::from_shape_vec((1, n),
    //     vec![
    //         false, true, true, false, true, false, false, false, true, true,
    //         // false, true, true, false, true, false, false, false, true, true,
    //         // false, true, true, false, true, false, false, false, true, true,
    //         // false, true, true, false, true, false, false, false, true, true,
    //     ]
    // ).unwrap());

    let m1 = arr1(&[true, false, true]);
    print_matrix(&m1);

    let m2 = arr2(&[[1, 2], [3, 4], [6, 56]]);
    print_matrix(&m2);

    // let m4 = Array4::<u8>::default((1,2,3,4));
    // print_matrix(&m4);
}

fn print_array(arr: &Array2<bool>) {
    let m = arr.shape()[0];
    let n = arr.shape()[1];

    print!("Array = ");

    for row in 0..m {
        for col in 0..n {
            let mut s = String::from("");
            if col > 0 && col % 4 == 0 {
                s += " ";
            }
            if col == 0 && row != 0 {
                s += "        ";
            }
            if arr[[row, col]] {
                s += "1";
            } else {
                s += "0";
            }

            print!("{s}");
        }

        println!();
    }

    println!();
}

fn print_matrix<A, S, D>(matrix: &ArrayBase<S, D>)
where
    A: std::fmt::Display,
    S: ndarray::Data<Elem = A>,
    D: ndarray::Dimension + ndarray::RemoveAxis,
{
    match matrix.ndim() {
        1 => {
            let length = matrix.shape()[0];
            let mut s = format!("Matrix 1 x {}: [", length);
            for item in matrix.iter() {
                s += &format!(" {:>3}", item);
            }

            s += " ]";
            println!("{}", s);
        }

        2 => {
            let rows = matrix.shape()[0];
            let columns = matrix.shape()[1];

            let mut s = format!("Matrix {} x {}:\n", rows, columns);

            for row in matrix.outer_iter() {
                s += "[";
                for item in row.iter() {
                    s += &format!(" {:>3}", item)
                }
                s += " ]\n";
            }

            println!("{}", s);
        }

        _ => println!("{}", matrix),
    }
}
