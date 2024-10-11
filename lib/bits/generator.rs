#[macro_export]
macro_rules! generate_bits {
    (
        $vis:vis struct $name:ident {
            type: $ty:ty,
            iterators:
                ones    : $ones_iter:ident,
                zeros   : $zeros_iter:ident,
                group   : $group_iter:ident,
                holes   : $holes_iter:ident,
        }
    ) => {

        /// Bits struct
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        $vis struct $name {
            len         : $ty,
            len_mask    : $ty,

            pub data    : $ty,
        }

        /// Function which formats u8 into its binary representations in blocks
        /// of 4 bit.
        fn uint_bits(int: $ty) -> String {
            format!("{:0pad$b}", int, pad=<$ty>::BITS as usize)
                .chars()
                .rev()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().rev().collect::<String>())
                .rev()
                .collect::<Vec<String>>()
                .join(" ")
        }

        // Display for bits struct
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = uint_bits(self.data);
                write!(f, "{str}")
            }
        }

        // Debug for bits struct
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("len      ", &self.len.to_string())
                    .field("len_mask ", &uint_bits(self.len_mask))
                    .field("data     ", &uint_bits(self.data))
                    .finish()
            }
        }

        // Basic implementations
        impl $name {
            /// Constructor of a new bits struct
            pub fn new(len: $ty, value: $ty) -> Self {
                assert!(len <= <$ty>::BITS as $ty);

                // Calculate length mask which assures, which is used to ensure
                // that the unused bits in the data are set to 0.
                let len_mask = !(<$ty>::MAX.unbounded_shl(len as u32));

                // Create the bits struct
                Self { len, len_mask, data: value & len_mask }
            }

            /// Construct bits struct from a vector of bits
            pub fn from_arr(len: $ty, bits: &[$ty]) -> Self {
                assert!(bits.len() as $ty <= len);
                assert!(!bits.iter().any(|x| *x >= 2));

                // Create "empty" bits struct
                let mut result = Self::new(len, 0);

                // Populate data
                for (i, val) in bits.iter().rev().enumerate() {
                    if *val == 1 {
                        result.set(i as $ty);
                    }
                }

                // Return
                result
            }

            /// Get length.
            pub fn len(&self) -> $ty {
                self.len
            }

            /// Set the bit at the given index to `1`.
            #[inline]
            pub fn set(&mut self, index: $ty) {
                assert!(index < self.len);

                // Mask example: 0000 0100
                // Modulo needed to prevent overflow when shifting by 8.
                let mask = 1 << (index); // % <$ty>::BITS as $ty);
                self.data |= mask;
            }

            /// Set the bit ad the given index to `0`.
            #[inline]
            pub fn unset(&mut self, index: $ty) {
                assert!(index < self.len);

                // Mask example: 1111 1011
                // Modulo needed to prevent overflow when shifting by 8.
                let mask = !(1 << (index)); // % <$ty>::BITS as $ty));
                self.data &= mask;
            }

            #[inline]
            pub fn set_block(&mut self, index: $ty, block_len: $ty) {
                assert!(index + block_len - 1 < self.len);

                // Create mask
                let mut mask = <$ty>::MAX;          // 1111 1111
                mask <<= block_len;                 // 1111 1000
                mask = !mask;                       // 0000 0111
                mask <<= index;                     // 0000 1110

                self.data |= mask;
            }

            #[inline]
            pub fn unset_block(&mut self, index: $ty, block_len: $ty) {
                assert!(index + block_len - 1 < self.len);

                // Create mask
                let mut mask = <$ty>::MAX;          // 1111 1111
                mask <<= block_len;                 // 1111 1000
                mask = !mask;                       // 0000 0111
                mask <<= index;                     // 0000 1110
                mask = !mask;                       // 1111 0001

                self.data &= mask;
            }

            /// Get the state of the bit at the given index.
            #[inline]
            pub fn get(&self, index: $ty) -> bool {
                assert!(index < self.len);

                let mask = 1 << index;
                let result = self.data & mask;

                // Return (if result != 0, then the bit is set)
                result != 0
            }

            /// Check if all bits are zero.
            #[inline]
            pub fn is_zero(&self) -> bool {
                self.data == 0
            }

            /// Create an iterator over all `1`'s in the bits data.
            pub fn ones(&self) -> $ones_iter {
                $ones_iter { state: self.data }
            }

            /// Create an iterator over all `0`'s in the bits data.
            pub fn zeros(&self) -> $zeros_iter {
                $zeros_iter { state: self.data, len: self.len }
            }

            /// Create an iterator over all `1`-groups of the given group_size.
            pub fn groups(&self, group_size: u32) -> $group_iter {
                $group_iter::new(&self.data, group_size, self.len)
            }

            /// Create an iterator over all `0`-groups of the given group_size.
            pub fn holes(&self, group_size: u32) -> $holes_iter {
                $holes_iter::new(self.data, group_size, self.len)
            }

            /// Calculate all blocks of `1`'s. This operation does NOT count
            /// groups of `1`'s twice! Bigger groups have precedence over
            /// smaller groups.
            ///
            /// Example:
            /// data = 1101 1101
            /// result = [
            ///     0 => 0000 0000
            ///     1 => 0000 0001
            ///     2 => 0100 0000
            ///     3 => 0000 0100
            /// ]
            ///
            /// As the result shows, the resulting matrix contains the starting
            /// indices of the groups at the respective index. The outer index
            /// represents the SIZE of the `1`-block.
            pub fn blocks(mut self) -> [$name; <$ty>::BITS as usize] {
                // Create result matrix
                let mut result: [$name; <$ty>::BITS as usize] =
                    [$name :: new(self.len, 0); <$ty>::BITS as usize];

                // Calculate maximal block size
                let max_block_size = (<$ty>::BITS as $ty).min(self.len);

                // Declare temporary variable. This variable is necessary,
                // because mutating `self` is not possible, as long as it is
                // borrowed by the `.groups()` method.
                let mut tmp = self.clone();

                for d in (1..=max_block_size).rev() {
                    for i in self.groups(d as u32) {
                        // Set index in result
                        result[d as usize].set(i);

                        // Unset block (in temporary variable).
                        tmp.unset_block(i, d);
                    }

                    // Apply the changes (of unsetting blocks) from the
                    // temporary variable to "self".
                    self = tmp;
                }

                // Return the result
                result
            }
        }


        /// Iterator that yiels the indices of all `1` in a bits struct.
        pub struct $ones_iter {
            state: $ty
        }

        impl Iterator for $ones_iter {
            type Item = u32; // return value of `trailing_zeros`

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.state != 0 {
                    // Count trailing zeros. The amount of trailing zeros is
                    // the correct index, because 0-indexing is used. Therefore
                    // no further correction is needed.
                    let i = <$ty>::trailing_zeros(self.state);

                    // Unset the found bit (same code as in the `unset` method).
                    // Modulo needed to prevent overflow when shifting by 8.
                    let mask = !(1 << (i)); // % <$ty>::BITS));
                    self.state &= mask;

                    // Return the index
                    Some(i)

                } else {
                    None
                }
            }
        }

        /// Iterator that yiels the indices of all `0` in a bits struct.
        pub struct $zeros_iter {
            state: $ty,
            len: $ty,
        }

        impl Iterator for $zeros_iter {
            type Item = u32; // return value of `trailing_ones`

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.state != <$ty>::MAX {
                    // Count the trailing ones. Because of 0-indexing the resulting
                    // is already correct and does not need to be corrected.
                    let i = <$ty>::trailing_ones(self.state);

                    // If the found index is out of the valid range, return `None`.
                    if i >= self.len as u32 {
                        return None;
                    }

                    // Set the found bit (same code as in the `set` method).
                    // Modulo needed to prevent overflow when shifting by 8.
                    let mask = 1 << (i); // % <$ty>::BITS);
                    self.state |= mask;

                    // Return the index
                    Some(i)

                } else {
                    None
                }
            }
        }

        /// Iterator that yiels the starting indices of all `1` groups/segments
        /// in a bits struct.
        pub struct $group_iter<'a> {
            value: &'a $ty,

            init_mask: $ty,
            index: $ty,
            max_index: $ty,
        }

        impl<'a> $group_iter<'a> {
            pub fn new(value: &'a $ty, size: u32, len: $ty) -> Self {
                assert!(size > 0);
                assert!(size <= <$ty>::BITS);
                assert!(len >= size as $ty);

                // Example size=2: 1111_1111 -> 0000_0011
                let init_mask = <$ty>::MAX >> (
                    (<$ty>::BITS - size as u32) // % <$ty>::BITS
                );

                let index = 0;
                let max_index = len - size as $ty;

                Self { value, init_mask, index, max_index }
            }
        }

        impl<'a> Iterator for $group_iter<'a> {
            type Item = $ty;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                while self.index <= self.max_index {
                    // Calculate mask
                    let mask = self.init_mask << self.index;

                    let check = self.value & mask;

                    if check == mask {
                        let result = self.index;
                        self.index += 1;
                        return Some(result);
                    }

                    self.index += 1;
                }

                None
            }
        }

        /// Iterator that yiels the starting indices of all `0` groups/segments
        /// in a bits struct.
        pub struct $holes_iter {
            value: $ty,

            init_mask: $ty,
            index: $ty,
            max_index: $ty,
        }

        impl $holes_iter {
            pub fn new(value: $ty, size: u32, len: $ty) -> Self {
                assert!(size > 0);
                assert!(size <= <$ty>::BITS);

                // Invert value
                let value = !value;

                // Example size=2: 1111_1111 -> 0000_0011
                let init_mask = <$ty>::MAX >> (
                    (<$ty>::BITS - size as u32) // % <$ty>::BITS
                );

                let index = 0;
                let max_index = len - size as $ty;

                Self { value, init_mask, index, max_index }
            }
        }

        impl Iterator for $holes_iter {
            type Item = $ty;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                while self.index <= self.max_index {
                    // Calculate mask
                    let mask = self.init_mask << self.index;

                    let check = self.value & mask;

                    if check == mask {
                        let result = self.index;
                        self.index += 1;
                        return Some(result);
                    }

                    self.index += 1;
                }

                None
            }
        }

        // Bits | Bits
        impl std::ops::BitOr for $name {
            type Output = Self;

            #[inline]
            fn bitor(mut self, rhs: Self) -> Self::Output {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data |= rhs.data;

                // Return
                self
            }
        }

        // Bits |= Bits
        impl std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data |= rhs.data;
            }
        }

        // Bits | u8
        impl std::ops::BitOr<$ty> for $name {
            type Output = Self;

            #[inline]
            fn bitor(mut self, rhs: $ty) -> Self::Output {
                // Perform operation
                self.data |= rhs;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits |= u8
        impl std::ops::BitOrAssign<$ty> for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $ty) {
                // Perform operation
                self.data |= rhs;

                // Apply length mask
                self.data &= self.len_mask;
            }
        }

        // Bits & Bits
        impl std::ops::BitAnd for $name {
            type Output = Self;

            #[inline]
            fn bitand(mut self, rhs: Self) -> Self::Output {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data &= rhs.data;

                // Return
                self
            }
        }

        // Bits &= Bits
        impl std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data &= rhs.data;
            }
        }

        // Bits & u8
        impl std::ops::BitAnd<$ty> for $name {
            type Output = Self;

            #[inline]
            fn bitand(mut self, rhs: $ty) -> Self::Output {
                // Perform operation
                self.data &= rhs;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits &= u8
        impl std::ops::BitAndAssign<$ty> for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $ty) {
                // Perform operation
                self.data &= rhs;

                // Apply length mask
                self.data &= self.len_mask;
            }
        }

        // Bits ^ Bits (XOR)
        impl std::ops::BitXor for $name {
            type Output = Self;

            #[inline]
            fn bitxor(mut self, rhs: Self) -> Self::Output {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data ^= rhs.data;

                // Return
                self
            }
        }

        // Bits ^= Bits (XOR assign)
        impl std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data ^= rhs.data;
            }
        }

        // Bits ^ u8 (XOR)
        impl std::ops::BitXor<$ty> for $name {
            type Output = Self;

            #[inline]
            fn bitxor(mut self, rhs: $ty) -> Self::Output {
                // Perform operation
                self.data ^= rhs;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits ^= u8 (XOR assign)
        impl std::ops::BitXorAssign<$ty> for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $ty) {
                // Perform operation
                self.data ^= rhs;

                // Apply length mask
                self.data &= self.len_mask;
            }
        }


        // Bits << u8
        impl std::ops::Shl<$ty> for $name {
            type Output = Self;

            #[inline]
            fn shl(mut self, rhs: $ty) -> Self::Output {
                // Perform operation
                self.data <<= rhs;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits <<= u8
        impl std::ops::ShlAssign<$ty> for $name {
            #[inline]
            fn shl_assign(&mut self, rhs: $ty) {
                // Perform operation
                self.data <<= rhs;

                // Apply length mask
                self.data &= self.len_mask;
            }
        }

        // Bits >> u8
        impl std::ops::Shr<$ty> for $name {
            type Output = Self;

            #[inline]
            fn shr(mut self, rhs: $ty) -> Self::Output {
                // Perform operation
                self.data >>= rhs;

                // Length mask not needed

                // Return
                self
            }
        }

        // Bits >>= u8
        impl std::ops::ShrAssign<$ty> for $name {
            #[inline]
            fn shr_assign(&mut self, rhs: $ty) {
                // Perform operation
                self.data >>= rhs;

                // Length mask not needed
            }
        }

        // ! Bits (not)
        impl std::ops::Not for $name {
            type Output = Self;

            #[inline]
            fn not(mut self) -> Self::Output {
                // Perform operation
                self.data = !self.data;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits + Bits
        impl std::ops::Add for $name {
            type Output = Self;

            #[inline]
            fn add(mut self, rhs: Self) -> Self::Output {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data += rhs.data;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }

        // Bits - Bits
        impl std::ops::Sub for $name {
            type Output = Self;

            #[inline]
            fn sub(mut self, rhs: Self) -> Self::Output {
                // Ensure both objects have the same length
                assert_eq!(self.len, rhs.len);

                // Perform operation
                self.data -= rhs.data;

                // Apply length mask
                self.data &= self.len_mask;

                // Return
                self
            }
        }
    };
}
