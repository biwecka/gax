// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::chromosome::Chromosome;
use hashbrown::HashMap;
use rand::{distributions::Uniform, prelude::Distribution};
use xhstt::db::Database;

// Functions ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub fn dynamic_single_point(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = db.events().len();

    // Randomness
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, chr_len);

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        let split_index = dist.sample(&mut rng);
        let p0_parts = p0.0.split_at(split_index);
        let p1_parts = p1.0.split_at(split_index);

        let c0 = [p0_parts.0, p1_parts.1].concat();
        let c1 = [p1_parts.0, p0_parts.1].concat();

        assert_eq!(c0.len(), chr_len);
        assert_eq!(c1.len(), chr_len);

        children.push(Chromosome(c0));
        children.push(Chromosome(c1));
    }

    // Return
    children
}

#[allow(unused)]
pub fn static_single_point(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database,
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    // Get chromosome length
    let chr_len = db.events().len();

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        let p0_parts = p0.0.split_at(325);
        let p1_parts = p1.0.split_at(325);

        let c0 = [p0_parts.0, p1_parts.1].concat();
        let c1 = [p1_parts.0, p0_parts.1].concat();

        assert_eq!(c0.len(), chr_len);
        assert_eq!(c1.len(), chr_len);

        children.push(Chromosome(c0));
        children.push(Chromosome(c1));
    }

    // Return
    children
}

#[allow(unused)]
pub fn pmx(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database
) -> Vec<Chromosome> {
    let mut children = Vec::with_capacity(parent_pairs.len() * 2);

    let chromosome_len = db.events().len();

    // Randomness
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(1, chromosome_len);

    // Calculate lower and upper "cut" indices
    let (lower, upper) = {
        let mut l = dist.sample(&mut rng);
        let mut r = dist.sample(&mut rng);

        while l.abs_diff(r) < 2 {
            l = dist.sample(&mut rng);
            r = dist.sample(&mut rng);
        }

        if l < r {
            (l, r)
        } else {
            (r, l)
        }
    };

    // Iterate parent pairs and perform crossover
    for ((p0, _), (p1, _)) in parent_pairs {
        // Split p0 into parts
        let p0_l = &p0.0[0..lower];
        let p0_m = &p0.0[lower..upper];
        let p0_r = &p0.0[upper..];

        // Split p1 into parts
        let p1_l = &p1.0[0..lower];
        let p1_m = &p1.0[lower..upper];
        let p1_r = &p1.0[upper..];

        // Create matcher
        let matcher = PmxMatcher::new(p0_m, p1_m);

        // Create child 0
        let c0_l = p0_l.iter().map(|x| matcher.calc_x_to_y(*x)).collect::<Vec<_>>();
        let c0_m = p1_m;
        let c0_r = p0_r.iter().map(|x| matcher.calc_x_to_y(*x)).collect::<Vec<_>>();

        let mut c0 = c0_l.to_vec();
        c0.extend(c0_m);
        c0.extend(c0_r);

        // Create child 1
        let c1_l = p1_l.iter().map(|x| matcher.calc_y_to_x(*x)).collect::<Vec<_>>();
        let c1_m = p0_m;
        let c1_r = p1_r.iter().map(|x| matcher.calc_y_to_x(*x)).collect::<Vec<_>>();

        let mut c1 = c1_l.to_vec();
        c1.extend(c1_m);
        c1.extend(c1_r);

        children.push(c0.into());
        children.push(c1.into());
    }

    children
}

#[allow(unused)]
pub fn shift(
    parent_pairs: Vec<((Chromosome, usize), (Chromosome, usize))>,
    db: &Database
) -> Vec<Chromosome> {
    parent_pairs
        .into_iter()
        .map(|((mut p0, _), (mut p1, _))| {
            let mut x = vec![p0.0.pop().unwrap()];
            x.extend(p0.0);
            p0.0 = x;

            let mut y = vec![p1.0.pop().unwrap()];
            y.extend(p1.0);
            p1.0 = y;

            (p0, p1)
        })
        .flat_map(|(a, b)| vec![a, b])
        .collect()
}

// Helper Structs //////////////////////////////////////////////////////////////
struct PmxMatcher {
    // x: Vec<i32>,
    // y: Vec<i32>,

    x_to_y: HashMap<u16, u16>,
    y_to_x: HashMap<u16, u16>,
}

impl PmxMatcher {
    pub fn new(x: &[u16], y: &[u16]) -> Self {
        assert_eq!(x.len(), y.len());

        let mut x_to_y = HashMap::new();
        let mut y_to_x = HashMap::new();

        for i in 0..x.len() {
            let a = x[i];
            let b = y[i];

            x_to_y.insert(b, a);
            y_to_x.insert(a, b);
        }


        Self { x_to_y, y_to_x }
    }

    pub fn calc_x_to_y(&self, input: u16) -> u16 {
        let mut result = input;

        while let Some(x) = self.x_to_y.get(&result) {
            result = *x;
        }

        result
    }

    pub fn calc_y_to_x(&self, input: u16) -> u16 {
        let mut result = input;

        while let Some(x) = self.y_to_x.get(&result) {
            result = *x;
        }

        result
    }
}

////////////////////////////////////////////////////////////////////////////////
