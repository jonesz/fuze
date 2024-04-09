//! Loss functions.
use core::ops::{Mul, Sub};
use core::iter::Sum;

pub trait Loss<E, F> {
    fn l(a: &E, b: &E) -> F;
}

/// L2 loss.
struct L2();

impl<const N: usize> Loss<[f32; N], f32> for L2 {
    fn l(a: &[f32; N], b: &[f32; N]) -> f32 {
        a.iter()
            .zip(b)
            .map(|(a_t, b_t)| (a_t - b_t) * (a_t - b_t)).sum()
    }
}

/// L1 loss.
pub fn l1<P>(t: &[P], p: &[P]) -> P {
    todo!();
}

/// L2 loss.
pub fn l2<P>(t: &[P], p: &[P]) -> P
where
    for<'a> &'a P: Sub<&'a P, Output = P>,
    P: Mul<Output = P> + Sum,
{
    // TODO: Handle unsigned types (usize could underflow!).
    t.iter()
        .zip(p)
        .map(|(y_t, p_t)| (y_t - p_t) * (y_t - p_t)).sum()
}

/// Mean Squared Error.
pub fn mse<P>(t: &[P], p: &[P]) -> P
where
    for<'a> &'a P: Sub<&'a P, Output = P>,
    P: Mul<Output = P> + Sum,
{
    todo!("1/n * l2(t, p)") // * l2(t, p)
}
    

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_l2() {
        let t = &[0i32, 1, 2, 3, 4];
        let p = &[5i32, 6, 7, 8, 9];
        let r = 125i32;
        assert_eq!(l2(t, p), r);
    }
}
