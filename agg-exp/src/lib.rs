#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
use core::ops::{Add, Div, Mul};

pub trait Expert<P> {
    // TODO: Should this be mutable?
    fn predict(&mut self, t: usize) -> P;
}

pub fn weighted_average_convex<P, W, const N: usize>(predictions: &[P; N], weights: &[W; N]) -> P
where
    for<'a> &'a W: Mul<&'a P, Output = P>,
    P: Add<Output = P> + Div<Output = P> + Copy,
{
    // PLG - 2.1 Weighted Average Prediction (pg. 9).
    // $\hat{p} = \sum_{i=1}{N} w_{i,t-1} f_{i,t} / \sum_{j=1}{N} w_{j,t-1}$
    let top = weights
        .iter()
        .zip(predictions)
        .map(|(w_i, f_i)| w_i * f_i)
        .reduce(|a, b| a + b)
        .unwrap();

    let bot = predictions
        .iter()
        // TODO: So we're iterating over `&P`; the reduction isn't fun because
        // we can compute `&P + P`, `&P + &P`, or `P + P`, so we introduce
        // a copy before. Can we do this without the copy?
        .map(|a| *a)
        .reduce(|a, b| a + b)
        .unwrap();

    top / bot
}
