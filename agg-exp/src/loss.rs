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
    t.iter()
        .zip(p)
        .map(|(y_t, p_t)| (y_t - p_t) * (y_t - p_t)).sum()
}
