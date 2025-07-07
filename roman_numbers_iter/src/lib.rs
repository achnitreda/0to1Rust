#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

use RomanDigit::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub (Vec<RomanDigit>, u32));

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => todo!(),
        }
    }
}

fn repeat_digit(vec: &mut Vec<RomanDigit>, digit: &[RomanDigit]) {
    for i in digit {
        vec.push(*i);
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let copy = value;
        let mut result = Vec::new();

        let mapping: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        for &(val, digits) in mapping {
            while value >= val {
                value -= val;
                repeat_digit(&mut result, digits)
            }
        }

        RomanNumber((result, copy))
    }
}

impl Iterator for RomanNumber {
    type Item = Self;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.0.1 > 4000 {
            return None;
        }
        self.0.1 += 1;
        Some(RomanNumber::from(self.0.1))
    }
}