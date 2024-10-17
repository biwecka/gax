// Imports /////////////////////////////////////////////////////////////////////

use simple_moving_average::SMA;

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    let mut list = [
        (0..300).collect::<Vec<usize>>(),
        (0..300).collect::<Vec<usize>>(),
        (0..300).collect::<Vec<usize>>(),
        (0..300).collect::<Vec<usize>>(),
        // (0..300).collect::<Vec<usize>>(),
        // (0..300).collect::<Vec<usize>>(),
        // vec![
        //     20, 20, 21, 15, 16, 11, 10, 3, 56, 13, 4, 20
        // ]
    ].concat();

    list.sort_by_key(|x| std::cmp::Reverse(*x));

    let mut best = list.first().cloned().unwrap();

    // let mut success: f32 = 0.25;
    // for x in list.iter() {
    //     if *x < best {
    //         success = pt1(success, 1., 100.);
    //         best = *x;
    //         // println!("");

    //     } else {
    //         success = pt1(success, 0., 100.);
    //     }

    //     println!("{success}");
    // }

    let mut successes = vec![];

    let mut ma = simple_moving_average::SumTreeSMA::<f32, f32, 100>::new();
    for x in list {
        if x < best {
            ma.add_sample(1.);
            best = x;

            successes.push(1);

        } else {
            ma.add_sample(0.);

            successes.push(0);
        }

        println!("{}", ma.get_average());
    }


    println!("success avg total = {}", successes.iter().sum::<usize>() as f32 / successes.len() as f32);

}

fn pt1(y: f32, u: f32, t: f32) -> f32 {
    if t + 1.0 != 1.0 {
        y + ((u - y) / t)
    } else {
        u
    }
}

////////////////////////////////////////////////////////////////////////////////
