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

impl<T: std::fmt::Debug + std::ops::Add<Output = T> + Clone + Scalar<Item = T>> Add for Matrix<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Option<Self>  {
        let mut m : Matrix<T> = Matrix(vec![vec![T::zero();self.0[0].len()];self.0.len()]);
        if self.0.len() == other.0.len() && self.0[0].len() == other.0[0].len() {
            for i in 0..self.0.len() {
                for j in 0..self.0[i].len() {
                    m.0[i][j] = self.0[i][j].clone()+other.0[i][j].clone();
                }
            }
            Some(m)
        }else{
            None
        }
    }
}

impl<T: std::fmt::Debug + std::ops::Sub<Output = T> + Clone + Scalar<Item = T>> Sub for Matrix<T> {
    type Output = Option<Self>;
    fn sub(self, other: Self) -> Option<Self>  {
        let mut m : Matrix<T> = Matrix(vec![vec![T::zero();self.0[0].len()];self.0.len()]);
        if self.0.len() == other.0.len() && self.0[0].len() == other.0[0].len() {
            for i in 0..self.0.len() {
                for j in 0..self.0[i].len() {
                    m.0[i][j] = self.0[i][j].clone()-other.0[i][j].clone();
                }
            }
            Some(m)
        }else{
            None
        }
    }
}


