use num_traits::{Num, ToPrimitive, Zero};
use std::ops::{Add, Deref, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct TripleNums<T: Num + ToPrimitive + Copy>(pub [T; 3]);
impl<T: Num + ToPrimitive + Copy> TripleNums<T> {
    #[inline]
    pub fn to_f64(self) -> TripleNums<f64> {
        TripleNums([
            self[0].to_f64().unwrap(),
            self[1].to_f64().unwrap(),
            self[2].to_f64().unwrap(),
        ])
    }
    #[inline]
    pub fn to_u8(self) -> TripleNums<u8> {
        TripleNums([
            self[0].to_u8().unwrap(),
            self[1].to_u8().unwrap(),
            self[2].to_u8().unwrap(),
        ])
    }
}
impl<T: Num + ToPrimitive + Copy> Zero for TripleNums<T> {
    fn zero() -> Self {
        TripleNums([T::zero(); 3])
    }
    fn is_zero(&self) -> bool {
        self.iter().all(|x| x.is_zero())
    }
}
impl<T: Num + ToPrimitive + Copy> Deref for TripleNums<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Num + ToPrimitive + Copy> Add for TripleNums<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}
impl<T: Num + ToPrimitive + Copy> Sub for TripleNums<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}
impl<T: Num + ToPrimitive + Copy> Mul<T> for TripleNums<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}
impl<T: Num + ToPrimitive + Copy> Mul for TripleNums<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}
impl<T: Num + ToPrimitive + Copy> Div<T> for TripleNums<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}
