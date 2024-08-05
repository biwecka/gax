// use std::i8;

// fn main() {
//     let x: u8 = 0b10110100;
//     println!("x    = {x:08b}");

//     let y: u8 = 2;
//     // println!("y = {y:#010b}");

//     let _z = x | y;
//     // println!("z = {z:#010b}");

//     let n0 = x >> 6;
//     println!("x[0] = {:02b}", n0);

//     let n1 = (x & 0b00110000) >> 4;
//     println!("x[1] =   {:02b}", n1);

//     let n2 = (x & 0b00001100) >> 2;
//     println!("x[2] =     {:02b}", n2);

//     let n3 = x & 0b00000011;
//     println!("x[3] =       {:02b}", n3);

//     println!();

//     let e: i8 = i8::MIN;
//     println!("e = {e:08b} ({e})");

//     let d: i8 = -1;
//     println!("d = {d:08b} ({d})");

//     let c: i8 = 0;
//     println!("c = {c:08b} ({c})");

//     let a: i8 = 1;
//     println!("a = {a:08b} ({a})");

//     let b: i8 = i8::MAX;
//     println!("b = {b:08b} ({b})");
// }

use std::collections::HashMap;

fn main() {
    let a = vec![1, 3, 0, 4, 7, 2, 9, 6, 5, 8];
    let b = vec![9, 4, 6, 2, 1, 0, 7, 5, 3, 8];

    let i0 = 3; // [
    let i1 = 6; // )

    let a_l = &a[0..i0];
    let a_m = &a[i0..i1];
    let a_r = &a[i1..];

    let b_l = &b[0..i0];
    let b_m = &b[i0..i1];
    let b_r = &b[i1..];

    let matcher = Matcher::new(a_m.to_vec(), b_m.to_vec());

    // Child one
    let a2_l = a_l.iter().map(|x| matcher.calc_x_to_y(*x)).collect::<Vec<_>>();
    let a2_m = b_m.to_vec();
    let a2_r = a_r.iter().map(|x| matcher.calc_x_to_y(*x)).collect::<Vec<_>>();

    let mut a2 = vec![];
    a2.extend(a2_l);
    a2.extend(a2_m);
    a2.extend(a2_r);

    // Child two
    let b2_l = b_l.iter().map(|x| matcher.calc_y_to_x(*x)).collect::<Vec<_>>();
    let b2_m = a_m.to_vec();
    let b2_r = b_r.iter().map(|x| matcher.calc_y_to_x(*x)).collect::<Vec<_>>();

    let mut b2 = vec![];
    b2.extend(b2_l);
    b2.extend(b2_m);
    b2.extend(b2_r);

    println!("a = {a2:?}");
    println!("b = {b2:?}");
}

struct Matcher {
    // x: Vec<i32>,
    // y: Vec<i32>,
    x_to_y: HashMap<i32, i32>,
    y_to_x: HashMap<i32, i32>,
}

impl Matcher {
    pub fn new(x: Vec<i32>, y: Vec<i32>) -> Self {
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

    pub fn calc_x_to_y(&self, input: i32) -> i32 {
        let mut result = input;

        while let Some(x) = self.x_to_y.get(&result) {
            result = *x;
        }

        result
    }

    pub fn calc_y_to_x(&self, input: i32) -> i32 {
        let mut result = input;

        while let Some(x) = self.y_to_x.get(&result) {
            result = *x;
        }

        result
    }
}
