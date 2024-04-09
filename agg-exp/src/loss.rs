//! Loss functions.
use core::iter::Sum;
use core::ops::{Mul, Sub};

pub trait Loss<T> {
    type Event;
    type Cost;

    fn l(a: &Self::Event, b: &Self::Event) -> Self::Cost;
}

/// L1 loss.
struct L1();

impl<const N: usize> Loss<[f32; N]> for L1 {
    type Event = [f32; N];
    type Cost = f32;

    fn l(a: &Self::Event, b: &Self::Event) -> Self::Cost {
        // https://www.reddit.com/r/rust/comments/15ue8z0/comment/jwp4pz7/
        fn abs(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & (i32::MAX as u32))
        }

        a.iter().zip(b).map(|(a_t, b_t)| abs(a_t - b_t)).sum()
    }
}

/// L2 loss.
struct L2();

impl<T, const N: usize> Loss<[T; N]> for L2
where
    for<'a> &'a T: Sub<Output = T>,
    T: Mul<Output = T> + Sum,
{
    type Event = [T; N];
    type Cost = T;

    fn l(a: &Self::Event, b: &Self::Event) -> Self::Cost {
        a.iter()
            .zip(b)
            .map(|(a_t, b_t)| (a_t - b_t) * (a_t - b_t))
            .sum()
    }
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
        let t = &[0f32, 1.0, 2.0, 3.0, 4.0];
        let p = &[5f32, 6.0, 7.0, 8.0, 9.0];
        let r = 125f32;
        assert_eq!(L2::l(t, p), r);
    }
}
