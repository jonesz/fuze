use core::ops::{Add, Mul, Sub};

pub fn l1<P>(t: &[P], p: &[P]) -> P {
    todo!();
}

/// L2 loss.
pub fn l2<P>(t: &[P], p: &[P]) -> P
where
    for<'a> &'a P: Sub<&'a P, Output = P>,
    P: Mul<Output = P> + Add<Output = P> + Copy,
{
    t.iter()
        .zip(p)
        .map(|(y_t, p_t)| y_t - p_t)
        // TODO: This requires a Copy trait, could be pushed upwards
        // into map as `(y_t - p_t) * (y_t - p_t)`. If that's done,
        // the Copy trait can be removed.
        .map(|a| a * a)
        // TODO: Utilize `.sum()`.
        .reduce(|a, b| a + b)
        .unwrap()
}
