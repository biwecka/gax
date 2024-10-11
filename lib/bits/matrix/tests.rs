// Imports /////////////////////////////////////////////////////////////////////
use crate::matrix::BitsMatrix8x16;
use crate::{Bits16, Bits8};

// Basic Tests /////////////////////////////////////////////////////////////////
#[test]
fn set_row() {
    let mut m = BitsMatrix8x16::new(8, 16);
    m.set_row(3, Bits16::new(16, 37));

    // Check via row
    assert_eq!(m.row(3), &Bits16::new(16, 37));

    // Check via columns
    let mut v = Bits16::new(16, 0);
    for (i, c) in m.cols().iter().enumerate() {
        if c.get(3) {
            v.set(15 - i as u16);
        }
    }
    assert_eq!(v, Bits16::new(16, 37));
}

#[test]
fn set_col() {
    let mut m = BitsMatrix8x16::new(8, 16);
    m.set_col(2, Bits8::new(8, 7));

    // Check via column
    assert_eq!(m.col(2), &Bits8::new(8, 7));

    // Check via rows
    let mut v = Bits8::new(8, 0);
    for (i, r) in m.rows().iter().enumerate() {
        if r.get(2) {
            v.set(7 - i as u8);
        }
    }

    assert_eq!(v, Bits8::new(8, 7));
}

#[test]
fn set_values_and_retrieve_rows_and_columns() {
    let mut m = BitsMatrix8x16::new(6, 10);
    m.set(5, 9);
    m.set(5, 6);
    m.set(5, 2);
    m.set(4, 3);
    m.set(3, 5);
    m.set(2, 8);
    m.set(1, 7);
    m.set(1, 2);
    m.set(1, 1);
    m.set(0, 6);
    m.set(0, 4);

    // Check rows
    assert_eq!(
        m.row(0),
        &Bits16::from_arr(10, &[0, 0, 0, 1, 0, 1, 0, 0, 0, 0]),
    );

    assert_eq!(
        m.row(1),
        &Bits16::from_arr(10, &[0, 0, 1, 0, 0, 0, 0, 1, 1, 0]),
    );

    assert_eq!(
        m.row(2),
        &Bits16::from_arr(10, &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
    );

    assert_eq!(
        m.row(3),
        &Bits16::from_arr(10, &[0, 0, 0, 0, 1, 0, 0, 0, 0, 0]),
    );

    assert_eq!(
        m.row(4),
        &Bits16::from_arr(10, &[0, 0, 0, 0, 0, 0, 1, 0, 0, 0]),
    );

    assert_eq!(
        m.row(5),
        &Bits16::from_arr(10, &[1, 0, 0, 1, 0, 0, 0, 1, 0, 0]),
    );

    // Check columns
    assert_eq!(m.col(0), &Bits8::from_arr(6, &[0, 0, 0, 0, 0, 0]),);

    assert_eq!(m.col(1), &Bits8::from_arr(6, &[0, 0, 0, 0, 1, 0]),);

    assert_eq!(m.col(2), &Bits8::from_arr(6, &[1, 0, 0, 0, 1, 0]),);

    assert_eq!(m.col(3), &Bits8::from_arr(6, &[0, 1, 0, 0, 0, 0]),);

    assert_eq!(m.col(4), &Bits8::from_arr(6, &[0, 0, 0, 0, 0, 1]),);

    assert_eq!(m.col(5), &Bits8::from_arr(6, &[0, 0, 1, 0, 0, 0]),);

    assert_eq!(m.col(6), &Bits8::from_arr(6, &[1, 0, 0, 0, 0, 1]),);

    assert_eq!(m.col(7), &Bits8::from_arr(6, &[0, 0, 0, 0, 1, 0]),);

    assert_eq!(m.col(8), &Bits8::from_arr(6, &[0, 0, 0, 1, 0, 0]),);

    assert_eq!(m.col(9), &Bits8::from_arr(6, &[1, 0, 0, 0, 0, 0]),);
}

////////////////////////////////////////////////////////////////////////////////
