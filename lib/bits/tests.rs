use crate::{Bits32, Bits8};

#[test]
fn from_arr_0() {
    let a = Bits8::from_arr(6, &[1, 0, 0, 1, 1, 0]);

    let mut b = Bits8::new(6, 0);
    b.set(1);
    b.set(2);
    b.set(5);

    assert_eq!(a, b);
}

#[test]
fn from_arr_1() {
    let a = Bits8::from_arr(4, &[1, 0]);

    let mut b = Bits8::new(4, 0);
    b.set(1);

    assert_eq!(a, b);
}

#[test]
fn length_mask() {
    let b = Bits8::new(3, 0b0000_1101);

    assert_eq!(b.data, 0b0000_0101);
}

#[test]
fn max_length() {
    let _b = Bits8::new(8, 0b1001_0101);

    assert!(true);
}

#[test]
fn set_bit() {
    let mut b = Bits8::new(8, 0b0111_1111);

    b.set(7);
    assert_eq!(b.data, u8::MAX);
}

#[test]
fn unset_bit() {
    let mut b = Bits8::new(8, 0b0000_1010);
    b.unset(3);

    assert_eq!(b.data, 0b0000_0010);
}

#[test]
fn get_bit() {
    let b = Bits8::new(6, 0b0101_1101);

    assert!(b.get(0));
    assert!(!b.get(1));
    assert!(b.get(4));
    assert!(!b.get(5));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn get_bit_out_of_length() {
    let b = Bits8::new(6, 0b0101_1101);
    assert!(b.get(6));
}

#[test]
fn ones_max_len() {
    let b = Bits8::new(8, 0b1001_1101);
    let ones = b.ones().collect::<Vec<_>>();

    assert_eq!(ones, vec![0, 2, 3, 4, 7]);
}

#[test]
fn ones_not_max_len() {
    let b = Bits8::new(5, 0b1001_1101);
    let ones = b.ones().collect::<Vec<_>>();

    assert_eq!(ones, vec![0, 2, 3, 4]);
}

#[test]
fn zeros_max_len() {
    let b = Bits8::new(8, 0b1001_1101);
    let zeros = b.zeros().collect::<Vec<_>>();

    assert_eq!(zeros, vec![1, 5, 6]);
}

#[test]
fn zeros_not_max_len() {
    let b = Bits8::new(5, 0b1001_1101);
    let zeros = b.zeros().collect::<Vec<_>>();

    assert_eq!(zeros, vec![1]);
}

#[test]
fn group_1_2_3_4_5_6() {
    //                          28   24   20   16   12    8    4    0
    let b = Bits32::new(32, 0b1111_1011_1011_1110_1000_0101_1110_1011);
    let group_1 = b.groups(1).collect::<Vec<_>>();
    assert_eq!(
        group_1,
        vec![
            0, 1, 3, 5, 6, 7, 8, 10, 15, 17, 18, 19, 20, 21, 23, 24, 25, 27,
            28, 29, 30, 31
        ]
    );

    let group_2 = b.groups(2).collect::<Vec<_>>();
    assert_eq!(
        group_2,
        vec![0, 5, 6, 7, 17, 18, 19, 20, 23, 24, 27, 28, 29, 30]
    );

    let group_3 = b.groups(3).collect::<Vec<_>>();
    assert_eq!(group_3, vec![5, 6, 17, 18, 19, 23, 27, 28, 29]);

    let group_4 = b.groups(4).collect::<Vec<_>>();
    assert_eq!(group_4, vec![5, 17, 18, 27, 28]);

    let group_5 = b.groups(5).collect::<Vec<_>>();
    assert_eq!(group_5, vec![17, 27]);

    let group_6 = b.groups(6).collect::<Vec<_>>();
    assert_eq!(group_6, vec![]);
}

#[test]
fn blocks_1() {
    let b = Bits8::new(6, 0b1011_1011);
    let blocks = b.blocks();
    assert_eq!(
        blocks,
        [
            Bits8::new(6, 0b0000_0000), // 0
            Bits8::new(6, 0b0000_0000), // 1
            Bits8::new(6, 0b0000_0001), // 2
            Bits8::new(6, 0b0000_1000), // 3
            Bits8::new(6, 0b0000_0000), // 4
            Bits8::new(6, 0b0000_0000), // 5
            Bits8::new(6, 0b0000_0000), // 6
            Bits8::new(6, 0b0000_0000), // 7
        ]
    )
}

#[test]
fn blocks_2() {
    let b = Bits8::new(8, 0b1011_1011);
    let blocks = b.blocks();
    assert_eq!(
        blocks,
        [
            Bits8::new(8, 0b0000_0000), // 0
            Bits8::new(8, 0b1000_0000), // 1
            Bits8::new(8, 0b0000_0001), // 2
            Bits8::new(8, 0b0000_1000), // 3
            Bits8::new(8, 0b0000_0000), // 4
            Bits8::new(8, 0b0000_0000), // 5
            Bits8::new(8, 0b0000_0000), // 6
            Bits8::new(8, 0b0000_0000), // 7
        ]
    )
}

#[test]
fn bit_or() {
    let b0 = Bits8::new(6, 0b1110_0110);
    let b1 = Bits8::new(6, 0b1001_0101);

    let res = b0 | b1;
    assert_eq!(res.data, 0b0011_0111);

    let b2 = Bits8::new(8, 0b0110_1010);
    let res = b2 | 0b1011_1011;
    assert_eq!(res.data, 0b1111_1011);
}

#[test]
fn bit_and() {
    let b0 = Bits8::new(6, 0b1110_0110);
    let b1 = Bits8::new(6, 0b1001_0101);

    let res = b0 & b1;
    assert_eq!(res.data, 0b0000_0100);

    let b2 = Bits8::new(8, 0b0110_1010);
    let res = b2 & 0b1011_1011;
    assert_eq!(res.data, 0b0010_1010);
}

#[test]
fn shl_shr() {
    let b0 = Bits8::new(6, 0b0010_1010);

    let res = b0 << 1;
    assert_eq!(res.data, 0b0001_0100);

    let res = b0 >> 1;
    assert_eq!(res.data, 0b0001_0101);

    let res = b0 >> 2;
    assert_eq!(res.data, 0b0000_1010);

    let mut b1 = Bits8::new(6, 0b1001_0101);
    b1 <<= 2;
    assert_eq!(b1.data, 0b0001_0100);

    b1 <<= 2;
    assert_eq!(b1.data, 0b0001_0000);

    b1 >>= 4;
    assert_eq!(b1.data, 0b0000_0001);

    b1 >>= 1;
    assert_eq!(b1.data, 0b0000_0000);
}

#[test]
fn not_invert() {
    let b0 = Bits8::new(6, 0b1011_0101);
    let res = !b0;

    assert_eq!(res.data, 0b0000_1010);
}

#[test]
fn add() {
    let b0 = Bits8::new(6, 0b1100_1001);
    let b1 = Bits8::new(6, 0b0001_0000);
    let res = b0 + b1;
    assert_eq!(res.data, 9 + 16);

    let b2 = Bits8::new(8, 0b0000_0110);
    let b3 = Bits8::new(8, 0b0000_1101);
    let res = b2 + b3;
    assert_eq!(res.data, 6 + 13);
}

#[test]
fn sub() {
    let b0 = Bits8::new(6, 0b1100_1001);
    let b1 = Bits8::new(6, 0b0001_0000);
    let res = b1 - b0;
    assert_eq!(res.data, 16 - 9);

    let b2 = Bits8::new(8, 0b0000_0110);
    let b3 = Bits8::new(8, 0b0000_1101);
    let res = b3 - b2;
    assert_eq!(res.data, 13 - 6);
}
