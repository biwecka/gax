use bitvec::prelude::*;

fn reduce_bit<T: BitStore>(bv: BitVec<T, Lsb0>) -> BitVec<T, Lsb0> {
    // Interpret bitvec as number and subtract 1.
    let mut num = bv.load_le::<usize>();
    if num == 0 { return bv }
    num -= 1;

    // Now store this number into a new bitvector
    let mut sub = bitvec![T, Lsb0; 0; bv.len()];
    sub.store(num);

    // Perform `and` operation and return
    bv & sub
}

fn main() {
    let b_va = bitvec![u8, Lsb0; 0, 0, 1, 0, 1, 1];
    println!("b_va     = {:?}", b_va);

    let mut x = reduce_bit(b_va);
    println!("reduce   = {:?}", x);

    x = reduce_bit(x);
    println!("reduce   = {:?}", x);

    x = reduce_bit(x);
    println!("reduce   = {:?}", x);

    println!("x.ones() = {}", x.count_ones());

    // let bits: BitVec<u8, Lsb0> = bitvec![
    //     u8, Lsb0; // Lsb -> 64 | Msb -> 2
    //     0, 0, 0, 0, 0, 0, 1, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    // ];
    // // bits.set(6, true);

    // // Load the bits as a little-endian number
    // let le_number = bits.load_le::<u16>(); // Little-endian interpretation

    // // Load the bits as a big-endian number
    // let be_number = bits.load_be::<u16>(); // Big-endian interpretation

    // println!("Bits (Lsb0): {:?}", bits);
    // println!("Little-endian interpretation: {}", le_number); // Output: 64
    // println!("Big-endian interpretation: {}", be_number); // Output: 16384

    // println!("\n---------------------------------------------------------\n\n");

    // let bits = bitvec_simd::BitVec::from_bool_iterator(
    //     vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    //         .into_iter()
    //         .map(|x| x > 0),
    // );

    // println!("{:?}", bits);

    // println!("\n---------------------------------------------------------\n\n");

    // let mut bit = fixedbitset::FixedBitSet::with_capacity(8);
    // bit.set(0, true);
    // bit.set(2, true);
    // println!("{:b}", bit);

    // let num = bit.ones().fold(0u64, |acc, indices| acc | 1 << indices);
    // dbg!(num);
}
