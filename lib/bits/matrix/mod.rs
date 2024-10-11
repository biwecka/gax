// Modules /////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests;
mod generator;

// 8x_ Matrix Implementations //////////////////////////////////////////////////
crate::generate_matrix!(pub struct BitsMatrix8x8 {
    col_type: crate::Bits8,
    col_base: u8,

    row_type: crate::Bits8,
    row_base: u8,
});

crate::generate_matrix!(pub struct BitsMatrix8x16 {
    col_type: crate::Bits8,
    col_base: u8,

    row_type: crate::Bits16,
    row_base: u16,
});

crate::generate_matrix!(pub struct BitsMatrix8x32 {
    col_type: crate::Bits8,
    col_base: u8,

    row_type: crate::Bits32,
    row_base: u32,
});

crate::generate_matrix!(pub struct BitsMatrix8x64 {
    col_type: crate::Bits8,
    col_base: u8,

    row_type: crate::Bits64,
    row_base: u64,
});

crate::generate_matrix!(pub struct BitsMatrix8x128 {
    col_type: crate::Bits8,
    col_base: u8,

    row_type: crate::Bits128,
    row_base: u128,
});


// 16x_ Matrix Implementations /////////////////////////////////////////////////
crate::generate_matrix!(pub struct BitsMatrix16x8 {
    col_type: crate::Bits16,
    col_base: u16,

    row_type: crate::Bits8,
    row_base: u8,
});

crate::generate_matrix!(pub struct BitsMatrix16x16 {
    col_type: crate::Bits16,
    col_base: u16,

    row_type: crate::Bits16,
    row_base: u16,
});

crate::generate_matrix!(pub struct BitsMatrix16x32 {
    col_type: crate::Bits16,
    col_base: u16,

    row_type: crate::Bits32,
    row_base: u32,
});

crate::generate_matrix!(pub struct BitsMatrix16x64 {
    col_type: crate::Bits16,
    col_base: u16,

    row_type: crate::Bits64,
    row_base: u64,
});

crate::generate_matrix!(pub struct BitsMatrix16x128 {
    col_type: crate::Bits16,
    col_base: u16,

    row_type: crate::Bits128,
    row_base: u128,
});

// 32x_ Matrix Implementations /////////////////////////////////////////////////
crate::generate_matrix!(pub struct BitsMatrix32x8 {
    col_type: crate::Bits32,
    col_base: u32,

    row_type: crate::Bits8,
    row_base: u8,
});

crate::generate_matrix!(pub struct BitsMatrix32x16 {
    col_type: crate::Bits32,
    col_base: u32,

    row_type: crate::Bits16,
    row_base: u16,
});

crate::generate_matrix!(pub struct BitsMatrix32x32 {
    col_type: crate::Bits32,
    col_base: u32,

    row_type: crate::Bits32,
    row_base: u32,
});

crate::generate_matrix!(pub struct BitsMatrix32x64 {
    col_type: crate::Bits32,
    col_base: u32,

    row_type: crate::Bits64,
    row_base: u64,
});

crate::generate_matrix!(pub struct BitsMatrix32x128 {
    col_type: crate::Bits32,
    col_base: u32,

    row_type: crate::Bits128,
    row_base: u128,
});

// 64x_ Matrix Implementations /////////////////////////////////////////////////
crate::generate_matrix!(pub struct BitsMatrix64x8 {
    col_type: crate::Bits64,
    col_base: u64,

    row_type: crate::Bits8,
    row_base: u8,
});

crate::generate_matrix!(pub struct BitsMatrix64x16 {
    col_type: crate::Bits64,
    col_base: u64,

    row_type: crate::Bits16,
    row_base: u16,
});

crate::generate_matrix!(pub struct BitsMatrix64x32 {
    col_type: crate::Bits64,
    col_base: u64,

    row_type: crate::Bits32,
    row_base: u32,
});

crate::generate_matrix!(pub struct BitsMatrix64x64 {
    col_type: crate::Bits64,
    col_base: u64,

    row_type: crate::Bits64,
    row_base: u64,
});

crate::generate_matrix!(pub struct BitsMatrix64x128 {
    col_type: crate::Bits64,
    col_base: u64,

    row_type: crate::Bits128,
    row_base: u128,
});

// 128x_ Matrix Implementations ////////////////////////////////////////////////
crate::generate_matrix!(pub struct BitsMatrix128x8 {
    col_type: crate::Bits128,
    col_base: u128,

    row_type: crate::Bits8,
    row_base: u8,
});

crate::generate_matrix!(pub struct BitsMatrix128x16 {
    col_type: crate::Bits128,
    col_base: u128,

    row_type: crate::Bits16,
    row_base: u16,
});

crate::generate_matrix!(pub struct BitsMatrix128x32 {
    col_type: crate::Bits128,
    col_base: u128,

    row_type: crate::Bits32,
    row_base: u32,
});

crate::generate_matrix!(pub struct BitsMatrix128x64 {
    col_type: crate::Bits128,
    col_base: u128,

    row_type: crate::Bits64,
    row_base: u64,
});

crate::generate_matrix!(pub struct BitsMatrix128x128 {
    col_type: crate::Bits128,
    col_base: u128,

    row_type: crate::Bits128,
    row_base: u128,
});

////////////////////////////////////////////////////////////////////////////////