use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;

#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        
        Matrix(matrix)
	}
}

pub trait Scalar: 
Add<Output = Self> +
Sub<Output = Self> +
Mul<Output = Self> +
Div<Output = Self> +
Debug + 
Clone +
Sized
{
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for f32 {
    type Item = Self;
    fn zero() -> Self {
        0.0
    }
    
    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {
    type Item = Self;
    fn zero() -> Self {
        0.0
    }
    
    fn one() -> Self {
        1.0
    }
}

impl Scalar for i32 {
    type Item = Self;
    fn zero() -> Self {
        0
    }
    
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    type Item = Self;
    fn zero() -> Self {
        0
    }
    
    fn one() -> Self {
        1
    }
}

impl Scalar for u32 {
    type Item = Self;
    fn zero() -> Self {
        0
    }
    
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    type Item = Self;
    fn zero() -> Self {
        0
    }
    
    fn one() -> Self {
        1
    }
}