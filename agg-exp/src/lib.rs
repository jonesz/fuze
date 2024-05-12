//! Prediction with Expert Advice.
//!
//! These procedures allow for the weighing and combination of multiple expert
//! predictions with the specific goal of minimizing loss in respect to the
//! *best* expert prediction. Implementations are taken from
//! 'Prediction, Learning, and Games' (2006) by Cesa-Bianchi and Lugosi.
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

/// Routines for continual prediction with expert advice.
pub mod forecaster;
mod loss;

use core::ops::{Add, Div, Mul, Sub};

/// An expert observes some environment 'E', producing a prediction 'P'.
pub trait Expert<E, P> {
    ///
    fn predict(&self, t: usize) -> P;
}

///
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
        .copied()
        .reduce(|a, b| a + b)
        .unwrap();

    top / bot
}

/// A forecaster's cumulative regret in regard to specific expert E.
fn _cumulative_regret<P, S, L>(revealed: &[P], p_hat: &[P], prediction: &[P], loss: L) -> S
where
    L: Fn(&P, &P) -> S,
    S: Add<Output = S> + Sub<Output = S>,
{
    // PLG - (pg. 8).
    // R_{E,n} = sum_{i=1}{N}(loss(\hat{p_t}, y_t) - loss(f_{E,t},y_t)) = \hat{L_n} - L_{E,n}
    revealed
        .iter()
        .zip(p_hat.iter().zip(prediction))
        .map(|(y_t, (p_t, f_t))| loss(p_t, y_t) - loss(f_t, y_t))
        // TODO: Rather than utilizing reduce, we could use Sum?
        .reduce(|a, b| a + b)
        .unwrap()
}

///
pub fn exponential_average_update<P, W, const N: usize>(
    _revealed: &P,
    _t: usize,
    predictions: &[P; N],
    weights: &[W; N],
    _n: &W,
) -> [W; N]
where
    W: Clone,
{
    // TODO: Is this a `Clone` or a `Copy`?
    let mut w_t: [W; N] = weights.clone();

    for (_w_j_t, _p_j) in w_t.iter_mut().zip(predictions) {}

    w_t
}
