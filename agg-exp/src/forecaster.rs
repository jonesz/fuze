/// Rules for the Combination and Weighing of Expert Advice.
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

/// Code related to the Exponential Weighted Average Forecaster.
pub mod exp {
    use super::*;
    use core::marker::PhantomData;

    #[derive(Debug)]
    enum EtaMethod {
        KnownHorizon(usize),
        RoundDependent,
    }

    /// The Exponentially Weighted Average Forecaster (PLG - pg. 14).
    #[derive(Debug)]
    pub struct EWAF<L, W, const N: usize> {
        w: [W; N],
        eta: EtaMethod,
        t: usize,

        phantom: PhantomData<L>,
    }

    impl<L, const N: usize> EWAF<L, f32, N>
    where
        L: Loss<f32, f32>,
    {
        /// Free paramter eta in relation to the time bound.
        fn eta(&self) -> f32 {
            // "2.2 An Optimal Bound", (PLG - pg. 16)
            // \eta = \sqrt{8 ln{N} / n}
            let kh = |n: usize| -> f32 { f32::sqrt(8.0f32 * f32::ln(N as f32) / n as f32) };
            // "2.3 Bounds That Hold Uniformly over Time", (PLG - pg. 17)
            // \eta = \sqrt{8 ln{N} / t}
            let rd = |t: usize| -> f32 { f32::sqrt(8.0f32 * f32::ln(N as f32) / t as f32) };

            match self.eta {
                EtaMethod::KnownHorizon(n) => kh(n),
                EtaMethod::RoundDependent => rd(self.t),
            }
        }
    }

    impl<L, const N: usize> Default for EWAF<L, f32, N>
    where
        L: Loss<f32, f32>,
    {
        fn default() -> Self {
            Self {
                w: [1.0; N],
                eta: EtaMethod::RoundDependent,
                t: 0usize,
                phantom: PhantomData,
            }
        }
    }

    // The implementation from PLG.
    impl<L, const N: usize> ExpertForecaster<f32, N> for EWAF<L, f32, N>
    where
        L: Loss<f32, f32>,
    {
        fn predict(&self, experts: &[f32; N]) -> f32 {
            // \hat{p_t} = \frac{\sum_{i=1}^{N} w_{i,t-1} f_{i,t}} - (PLG - pg. 9)
            //                  {\sum_{j=1}^{N} w_{j,t-1}}
            self.w.iter().zip(experts).map(|(w, f)| w * f).sum::<f32>() / self.w.iter().sum::<f32>()

            // For the below equation, consider the first prediction: with `w_i` all initialized
            // to 1, the initial predictions won't be scaled! I'm going with the above which
            // can be found in PLG and appears in "EECS598: Prediction and Learning, Lecture 3: The
            // Exponential Weights Algorithm".

            // \hat{p_t} = \frac{\sum_{i=1}^{N} w_{i,t-1} f_{i,t}} - (PLG - pg. 14)
            // self.w.iter().zip(experts).map(|(w, f)| w * f).sum::<f32>()
        }

        fn update(&mut self, experts: &[f32; N], revealed: &f32) {
            self.t += 1;
            let eta = self.eta();

            // w_{i,t} = w_{i,t-1} e^{-\eta \l(f_{i,t}, y_t)} - EECS598... "The Exponential Weights Algorithm".
            for (w_i, p_i) in self.w.iter_mut().zip(experts) {
                *w_i *= f32::exp(-1.0f32 * eta * L::l(p_i, revealed));
            }

            // The below matches the second equation in `predict()`.

            // w_{i,t} = \frac{w_{i,t-1} e^{-\eta \l(f_{i,t}, y_t)}}
            //                {\sum_{j=1}^N w_{j,t-1} e^{-\eta \l(f_{j,t-1},y_t)}}
            // (PLG - pg.14)
            // let bot = self.w.iter().sum::<f32>();
            // for (w_i, p_i) in self.w.iter_mut().zip(experts) {
            //     *w_i = f32::exp(-1.0f32 * eta * L::l(p_i, revealed)) / bot;
            // }
        }
    }

    #[cfg(test)]
    mod test {}
}
