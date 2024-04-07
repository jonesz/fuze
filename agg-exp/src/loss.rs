//! Loss functions.
use core::ops::{Mul, Sub};
use core::iter::Sum;

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
