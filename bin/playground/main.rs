// Imports /////////////////////////////////////////////////////////////////////

// Main ////////////////////////////////////////////////////////////////////////
fn main() {
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = vec![5, 2, 3, 1, 6, 4];

    let x = ordered(a, b);
    println!("{x:?}");
}

pub fn ordered<T: std::fmt::Debug + Clone + PartialEq + Eq>(
    a: Vec<T>,
    b: Vec<T>,
    // rate: f32,
) -> Vec<T> {
    // let mut rng = rand::thread_rng();

    // if rng.gen_range(0. ..=1.) > rate {
    //     return (a.to_owned(), b.to_owned());
    // }

    // Get chromosome len
    let clen = a.len();

    // Create two random points/indices for the middle part of the ordered
    // crossover.
    // let i0 = rng.gen_range(0..(clen-1));
    // let i1 = rng.gen_range((i0 + 1)..clen); // This ensures that i1 > i0

    let i0 = 5;
    let i1 = 5;

    // Create first child
    let c0: Vec<T> = {
        // Middle part comes from parent a
        let middle = &a[i0..i1];
        println!("middle    = {middle:?}");

        // Get a view of parent b without the values from `middle`.
        let remainder = b
            .iter()
            .filter_map(|x| {
                if !middle.contains(x) {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();

        // The part after the middle (called tail) of this child is now filled
        // from the calculated remainder.
        println!("remainder = {remainder:?}");
        let tail = &remainder[0..(clen - i1)];

        // The part befor the middle (called head) of this child is now filled
        // with the remaining values from the remainder.
        let head = &remainder[(clen - i1)..];

        // Concatenate the three parts to get the full chromosome
        [head, middle, tail].concat()
    };

    c0
}
////////////////////////////////////////////////////////////////////////////////
