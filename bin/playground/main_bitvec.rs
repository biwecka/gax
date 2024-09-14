use bitvec::prelude::*;

fn main() {
    let bits: BitVec<u8, Msb0> = bitvec![
        u8, Msb0; // Lsb -> 64 | Msb -> 2
        0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // bits.set(6, true);

    // Load the bits as a little-endian number
    let le_number = bits.load_le::<u16>(); // Little-endian interpretation

    // Load the bits as a big-endian number
    let be_number = bits.load_be::<u16>(); // Big-endian interpretation

    println!("Bits (Lsb0): {:?}", bits);            // Output: [1, 1, 0, 1]
    println!("Little-endian interpretation: {}", le_number); // Output: 13
    println!("Big-endian interpretation: {}", be_number);    // Output: 11
}