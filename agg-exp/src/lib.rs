#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
use core::ops::{Add, Div, Mul};

pub trait Expert<P> {
    // TODO: Should this be mutable?
    fn predict(&mut self, t: usize) -> P;
}

pub fn weighted_average<P, W, F, const N: usize>(
    predictions: &[P; N],
    weights: &[W; N],
    t: usize,
    convex: Option<F>,
) -> P
where
    F: Fn([W; N], [P; N]) -> P,
    W: Mul<P, Output = P>,
    P: Add<Output = P> + Div<Output = P>,
{
    if let convex_f = Some(convex) {
        // $\hat{p} = \sum_{i=1}{N} w_{i,t-1} f_{i,t} / \sum_{j=1}{N} w_{j,t-1}$
        let top = weights
            .into_iter()
            .zip(predictions)
            .map(|(w_i, f_i)| w_i * f_i)
            .reduce(|a, b| a + b)
            .unwrap();
        let bot = predictions.into_iter().reduce(|a, b| a + b).unwrap();
        top / bot
    } else {
        todo!("The function isn't convex; we need to utilize randomized prediction.");
    }
}