fn main() {
    n_queens::run();

    // let mut list: Vec<usize> = vec![5, 0, 1, 2, 8, 9, 2, 1, 7];
    // list.sort();

    // println!("list  = {:?}", list);

    // let max: usize = *list.iter().max().unwrap();

    // list = list.iter().map(|x| max - x).collect();


    // let sum: usize = list.iter().sum();

    // let weight: Vec<f32> = list.iter().map(|val| *val as f32 / sum as f32).collect();
    // let mut acc: f32 = 0.;
    // let mut accum: Vec<f32> = vec![];
    // for w in &weight {
    //     accum.push(w + acc);
    //     acc = w + acc;
    // }
    // let last = accum.last_mut().unwrap();
    // *last = 1.;

    // println!("list' = {:?}", list);
    // println!("weight= {:?}", weight);
    // println!("accum = {:?}", accum);


    // let mut map = std::collections::HashMap::<usize, usize>::new();

    // let mut rng = rand::thread_rng();
    // let dist = rand::distributions::Uniform::new_inclusive(0., 1.);

    // for _ in 0..1_000_000_000 {
    //     let value = dist.sample(&mut rng);

    //     for (i, section) in accum.iter().enumerate() {
    //         if &value < section {
    //             map.entry(i).or_default().add_assign(1);
    //             break;
    //         }
    //     }
    // }

    // let mut results: Vec<(usize, usize)> = vec![];
    // for (i, amount) in map {
    //     results.push((i, amount));
    // }

    // results.sort_by_key(|(x, _)| *x);
    // println!("results = {:?}", results);
}