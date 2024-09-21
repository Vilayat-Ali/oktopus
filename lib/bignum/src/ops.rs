use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::BigNumber;

impl Add for BigNumber {
    type Output = BigNumber;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for BigNumber {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Sub for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign for BigNumber {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Mul for BigNumber {
    type Output = BigNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for BigNumber {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Div for BigNumber {
    type Output = BigNumber;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl DivAssign for BigNumber {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}
