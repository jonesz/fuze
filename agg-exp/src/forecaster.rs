///! Rules for the Combination and Weighing of Expert Advice.
use crate::loss::Loss;

/// A base for algorithms capable of prediction with expert advice.
///
/// An ExpertPredictor provides methods for:
/// 1. Given some 'N' experts' predictions, produce a prediction of
/// type 'P'.
/// 2. Given those same experts' predictions and an environmentally
/// revealed 'P', compute new weights for the next prediction.
pub trait ExpertForecaster<P, const N: usize> {
    /// Given expert predictions, produce a prediction.
    fn predict(&self, experts: &[P; N]) -> P;

    /// Given those same expert predictions and the now revealed value,
    /// update the internal weights to be utilized in the next
    /// prediction.
    fn update(&mut self, experts: &[P; N], revealed: &P);
}

pub mod exp {
    use super::*;
    use core::marker::PhantomData;

    /// The Exponentially Weighted Average Forecaster (PLG - pg. 14).
    pub struct EWAF<L, W, const N: usize>
    where
        L: Loss<W>,
    {
        w: [W; N],
        phantom: PhantomData<L>,
    }

    impl<L, const N: usize> EWAF<L, f32, N>
    where
        L: Loss<f32>,
    {
        /// Free paramter eta in relation to the time bound.
        fn eta(&self) -> f32 {
            todo!();
        }
    }

    impl<L, const N: usize> Default for EWAF<L, f32, N>
    where
        L: Loss<f32>,
    {
        fn default() -> Self {
            Self {
                w: [1.0; N],
                phantom: PhantomData,
            }
        }
    }

    impl<L, const N: usize> ExpertForecaster<f32, N> for EWAF<L, f32, N>
    where
        L: Loss<f32, Event = f32, Cost = f32>,
    {
        fn predict(&self, experts: &[f32; N]) -> f32 {
            // \hat{p_t} = \sum_{i=1}^{N} w_{i,t-1} f_{i,t} (PLG - pg. 14)
            self.w.iter().zip(experts).map(|(w, f)| w * f).sum::<f32>() / self.w.iter().sum::<f32>()
        }

        fn update(&mut self, expert: &[f32; N], revealed: &f32) {
            fn approx_exp(x: f32) -> f32 {
                // https://math.stackexchange.com/a/56064
                // exp(z) = \frac{(z+3)^2 + 3}{(z-3)^2 + 3}
                ((x + 3.0) * (x + 3.0) + 3.0) / ((x - 3.0) * (x - 3.0) + 3.0)
            }

            let eta = self.eta();
            // This is pulled from notes of EECS 598 - Fall 2013, 'Prediction and Learning:
            // It's Only a Game'; specifically 'Lecture 3, 9/11: The Exponential Weights Algorithm.'
            // See '3-3 The Exponential Weight Algorithm' (pg. 3).
            for (w_i, p_i) in self.w.iter_mut().zip(expert) {
                *w_i *= approx_exp(-1.0f32 * eta * L::l(p_i, revealed));
            }
        }
    }
    #[cfg(test)]
    mod test {
        use super::*;
    }
}
