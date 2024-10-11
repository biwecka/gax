#![feature(unbounded_shifts)]

use bits::{matrix::BitsMatrix8x16, rows::BitRows8, Bits16, Bits8};

fn main() {
    let mut m = BitsMatrix8x16::new(4, 8);
    m.set_row(1, Bits16::new(8, 9));
    m.set_col(6, Bits8::new(4, 15));

    println!("{m}");

    // // Print rows
    // for r in m.rows() {
    //     println!("{r}");
    // }

    // // Print cols
    // let mut rows: Vec<String> = vec![];
    // for r in (0..m.num_rows()).rev() {
    //     let mut row: Vec<String> = vec![];
    //     for (ci, c) in m.cols().iter().enumerate() {
    //         if ci > 0 && ci % 4 == 0 {
    //             row.push(" ".into());
    //         }

    //         if c.get(r) {
    //             row.push("1".into());
    //         } else {
    //             row.push("0".into());
    //         }
    //     }

    //     rows.push(row.join(""));
    // }
    // println!("{}", rows.join("\n"));
}


fn main2() {
    let b0 = Bits8::new(8, 0b1011_1011);
    println!("{b0}");
    // println!("ones     = {:?}", b0.ones().collect::<Vec<_>>());
    // println!("zeros    = {:?}", b0.zeros().collect::<Vec<_>>());
    // println!("group(3) = {:?}", b0.groups(3).collect::<Vec<_>>());
    // println!("group(2) = {:?}", b0.groups(2).collect::<Vec<_>>());
    // println!("group(1) = {:?}", b0.groups(1).collect::<Vec<_>>());

    let blocks = b0.blocks();
    for (d, bits) in blocks.iter().enumerate() {
        if bits.is_zero() {
            continue;
        }

        println!("d={d} | {bits}");
    }
}

#[derive(Copy, Clone)]
pub struct Bits {
    len: u8,
    len_mask: u8,
    bits: u8,
}

fn uint_bits(int: u8, len: u8) -> String {
    format!("{:0pad$b}", int, pad = len as usize)
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().rev().collect::<String>())
        .rev()
        .collect::<Vec<String>>()
        .join(" ")
}

impl std::fmt::Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = uint_bits(self.bits, self.len);
        write!(f, "{str}")
    }
}

impl std::fmt::Debug for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bits")
            .field("len      ", &self.len.to_string())
            .field("len_mask ", &uint_bits(self.len_mask, self.len))
            .field("data     ", &uint_bits(self.bits, self.len))
            .finish()
    }
}

impl Bits {
    pub fn new(len: u8, value: u8) -> Self {
        assert!(len <= 8);

        // Calculate the mask for handling "free" space in the underlying data.
        // Using "unbounded left shift" because the "modulo trick" will make
        // no shift at all resulting in a mask of 11111111 (all 1).
        let len_mask = !(u8::MAX.unbounded_shl(len as u32));

        Self { len, len_mask, bits: value & len_mask }
    }

    #[inline]
    pub fn set(&mut self, index: u8) {
        assert!(index < self.len);

        // Modulo needed to prevent overflow when shifting by 8.
        let mask = 1 << (index % 8); // example: 0000 0100
        self.bits |= mask;
    }

    #[inline]
    pub fn unset(&mut self, index: u8) {
        assert!(index < self.len);

        // Modulo needed to prevent overflow when shifting by 8.
        let mask = !(1 << (index % 8)); // example: 1111 1011
        self.bits &= mask;
    }

    #[inline]
    pub fn set_block(&mut self, index: u8, block_len: u8) {
        assert!(index + block_len - 1 < self.len);

        // Create mask
        let mut mask = u8::MAX; // 1111 1111
        mask <<= block_len; // 1111 1000
        mask = !mask; // 0000 0111
        mask <<= index; // 0000 1110

        self.bits |= mask;
    }

    #[inline]
    pub fn unset_block(&mut self, index: u8, block_len: u8) {
        assert!(index + block_len - 1 < self.len);

        // Create mask
        let mut mask = u8::MAX; // 1111 1111
        mask <<= block_len; // 1111 1000
        mask = !mask; // 0000 0111
        mask <<= index; // 0000 1110
        mask = !mask; // 1111 0001

        self.bits &= mask;
    }

    pub fn ones(&self) -> OneBits {
        OneBits { state: self.bits }
    }

    pub fn zeros(&self) -> ZeroBits {
        ZeroBits { state: self.bits, len: self.len }
    }

    pub fn groups(&self, group_size: u32) -> GroupBits {
        GroupBits::new(&self.bits, group_size)
    }
}

// Iterator
pub struct OneBits {
    state: u8,
}

impl Iterator for OneBits {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.state != 0 {
            // Count trailing zeros. The amount of trailing zeros is the correct
            // index, because 0-indexing is used. Therefore no further
            // correction is needed.
            let i = u8::trailing_zeros(self.state);

            // Unset the found bit (same code as in the `unset` method).
            // Modulo needed to prevent overflow when shifting by 8.
            let mask = !(1 << (i % 8));
            self.state &= mask;

            // Return the index
            Some(i)
        } else {
            None
        }
    }
}

// Iterator
pub struct ZeroBits {
    state: u8,
    len: u8,
}

impl Iterator for ZeroBits {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.state != u8::MAX {
            // println!("state = {:08b}", self.state);
            // Count the trailing ones. Because of 0-indexing the resulting
            // is already correct and does not need to be corrected.
            let i = u8::trailing_ones(self.state);

            // If the found index is out of the valid range, return `None`.
            if i >= self.len as u32 {
                return None;
            }

            // Set the found bit (same code as in the `set` method).
            // Modulo needed to prevent overflow when shifting by 8.
            let mask = 1 << (i % 8);
            self.state |= mask;

            // Return the index
            Some(i)
        } else {
            None
        }
    }
}

// Iterator for finding groups of set bits.
pub struct GroupBits<'a> {
    value: &'a u8,

    index: u8,
    size: u8,
}

impl<'a> GroupBits<'a> {
    pub fn new(value: &'a u8, size: u32) -> Self {
        assert!(size > 0);
        assert!(size < u8::BITS);

        let index = 0;
        let size = size as u8;

        Self { value, index, size }
    }
}

impl<'a> Iterator for GroupBits<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 8 {
            // Calculate mask
            let mut mask = u8::MAX >> ((u8::BITS - self.size as u32) % 8);
            mask <<= self.index;

            let check = self.value & mask;

            if check == mask {
                let result = self.index;
                self.index += 1;
                return Some(result);
            }

            self.index += 1;
        }

        None

        // while self.mask != self.end_mask {
        //     // Perform check
        //     let check = self.value & self.mask;

        //     if check == self.mask {
        //         let result = self.index;

        //         self.mask <<= 1;
        //         self.index += 1;

        //         return Some(result);

        //     } else {
        //         self.mask <<= 1;
        //         self.index += 1;
        //     }
        // }

        // None
    }
}

// Bits | Bits
impl std::ops::BitOr for Bits {
    type Output = Bits;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        // Ensure both objects have the same length
        assert_eq!(self.len, rhs.len);

        // Perform operation
        self.bits |= rhs.bits;

        // Return
        self
    }
}

// Bits | u8
impl std::ops::BitOr<u8> for Bits {
    type Output = Bits;

    #[inline]
    fn bitor(mut self, rhs: u8) -> Self::Output {
        // Perform operation
        self.bits |= rhs;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}

// Bits & Bits
impl std::ops::BitAnd for Bits {
    type Output = Bits;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        // Ensure both objects have the same length
        assert_eq!(self.len, rhs.len);

        // Perform operation
        self.bits &= rhs.bits;

        // Return
        self
    }
}

// Bits & u8
impl std::ops::BitAnd<u8> for Bits {
    type Output = Bits;

    #[inline]
    fn bitand(mut self, rhs: u8) -> Self::Output {
        // Perform operation
        self.bits &= rhs;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}

// Bits << u8
impl std::ops::Shl<u8> for Bits {
    type Output = Self;

    #[inline]
    fn shl(mut self, rhs: u8) -> Self::Output {
        // Perform operation
        self.bits <<= rhs;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}

// Bits >> u8
impl std::ops::Shr<u8> for Bits {
    type Output = Self;

    #[inline]
    fn shr(mut self, rhs: u8) -> Self::Output {
        // Perform operation
        self.bits >>= rhs;

        // Length mask not needed

        // Return
        self
    }
}

// Bits <<= u8
impl std::ops::ShlAssign<u8> for Bits {
    #[inline]
    fn shl_assign(&mut self, rhs: u8) {
        // Perform operation
        self.bits <<= rhs;

        // Apply length mask
        self.bits &= self.len_mask;
    }
}

// Bits >>= u8
impl std::ops::ShrAssign<u8> for Bits {
    #[inline]
    fn shr_assign(&mut self, rhs: u8) {
        // Perform operation
        self.bits >>= rhs;

        // Length mask not needed
    }
}

// ! Bits (not)
impl std::ops::Not for Bits {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self::Output {
        // Perform operation
        self.bits = !self.bits;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}

// Bits + Bits
impl std::ops::Add for Bits {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        // Ensure both objects have the same length
        assert_eq!(self.len, rhs.len);

        // Perform operation
        self.bits += rhs.bits;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}

// Bits - Bits
impl std::ops::Sub for Bits {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        // Ensure both objects have the same length
        assert_eq!(self.len, rhs.len);

        // Perform operation
        self.bits -= rhs.bits;

        // Apply length mask
        self.bits &= self.len_mask;

        // Return
        self
    }
}
