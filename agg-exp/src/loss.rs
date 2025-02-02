//! Loss functions.
use core::iter::Sum;
use core::ops::{Div, Mul, Sub};

/// Loss function.
pub trait Loss<E, C> {
    /// Compute the loss from a, b.
    fn l(a: &E, b: &E) -> C;
}

/// L1 loss.
#[derive(Debug)]
pub struct L1();

impl<const N: usize> Loss<[f32; N], f32> for L1 {
    fn l(a: &[f32; N], b: &[f32; N]) -> f32 {
        // https://www.reddit.com/r/rust/comments/15ue8z0/comment/jwp4pz7/
        fn abs(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & (i32::MAX as u32))
        }

        a.iter().zip(b).map(|(a_t, b_t)| abs(a_t - b_t)).sum()
    }
}

impl Loss<f32, f32> for L1 {
    fn l(a: &f32, b: &f32) -> f32 {
        // https://www.reddit.com/r/rust/comments/15ue8z0/comment/jwp4pz7/
        fn abs(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & (i32::MAX as u32))
        }

        abs(a - b)
    }
}

/// L2 loss.
#[derive(Debug)]
pub struct L2();

impl<T, const N: usize> Loss<[T; N], T> for L2
where
    for<'a> &'a T: Sub<Output = T>,
    T: Mul<Output = T> + Sum,
{
    fn l(a: &[T; N], b: &[T; N]) -> T {
        a.iter()
            .zip(b)
            .map(|(a_t, b_t)| (a_t - b_t) * (a_t - b_t))
            .sum()
    }
}

impl<T> Loss<T, T> for L2
where
    for<'a> &'a T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    fn l(a: &T, b: &T) -> T {
        (a - b) * (a - b)
    }
}

/// MSE for 1-dimensional vectors.
pub fn mse<const N: usize, T>(a: &[T; N], b: &[T; N]) -> T
where
    for<'a> &'a T: Sub<Output = T>,
    T: Mul<Output = T> + Div<Output = T> + Sum + From<usize>,
{
    a.iter().zip(b).map(|(a_t, b_t)| L2::l(a_t, b_t)).sum::<T>() / Into::<T>::into(N)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_l2() {
        let t = &[0f32, 1.0, 2.0, 3.0, 4.0];
        let p = &[5f32, 6.0, 7.0, 8.0, 9.0];
        let r = 125f32;
        assert_eq!(L2::l(t, p), r);
    }
}
