use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;

#[derive(Debug,PartialEq,Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

    pub fn row(&self, index : usize) -> Vec<T> {
        for i in 0..self.0.len() {
            if i == index {
                return self.0[i].clone()
            }
        }
        vec![]
    }

    pub fn col(&self, index : usize) -> Vec<T> {
        let mut res = vec![];
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                if j == index {
                    res.push(self.0[i][j].clone())
                }
            }
        }
        res
    }
}

pub trait Scalar: 
Add<Output = Self> +
Sub<Output = Self> +
Mul<Output = Self> +
Div<Output = Self> +
Debug + 
Clone +
PartialEq +
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

impl<T: std::fmt::Debug + std::ops::Mul<Output = T> + Clone + Scalar<Item = T>> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, other: Self) -> Option<Self>  {
        let mut m : Matrix<T> = Matrix(vec![vec![T::zero();self.0[0].len()];self.0.len()]);
        if self.0.len() == other.0.len() && self.0[0].len() == other.0[0].len() {
            for i in 0..self.0.len() {
                for j in 0..self.0[i].len() {
                    let mut sum = T::zero();
                    for k in 0..self.0[i].len() {
                        // [vec![0, 1], vec![0, 0]] * [vec![0, 0], vec![1, 0]]
                        // row [0,1] * col [0,1]
                        // row [0,0] * col [0,0]
                        // ((0*0)+(1*1),(0*0)+(0*0)) -> (1,0)
                        sum = sum + (self.0[i][k].clone() * other.0[k][j].clone());
                    }
                    m.0[i][j] = sum;
                }
            }
            Some(m)
        }else{
            None
        }
    }
}



