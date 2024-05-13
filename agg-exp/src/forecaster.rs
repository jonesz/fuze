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

    /// The Exponentially Weighted Average Forecaster (PLG - pg. 14).
    pub struct EWAF<L, W, const N: usize> {
        w: [W; N],
        phantom: PhantomData<L>,
    }

    impl<L, const N: usize> EWAF<L, f32, N>
    where
        L: Loss<f32, f32>,
    {
        /// Free paramter eta in relation to the time bound.
        fn eta(&self) -> f32 {
            1.0f32
            // todo!();
        }

        /// Given a known horizon `n`, compute the optimal \n parameter.
        fn _eta_known_horizon(n: usize) -> f32 {
            fn approx_sqrt(_x: f32) -> f32 {
                todo!();
            }

            fn approx_ln(_x: usize) -> f32 {
                todo!();
            }

            approx_sqrt((n as f32 / 2f32) * approx_ln(N))
        }

        /// An \eta parameter that varies based on the round number.
        fn _eta_time_varying(t: usize) -> f32 {
            todo!()
        }
    }

    impl<L, const N: usize> Default for EWAF<L, f32, N>
    where
        L: Loss<f32, f32>,
    {
        fn default() -> Self {
            Self {
                w: [1.0; N],
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
            // \hat{p_t} = \sum_{i=1}^{N} w_{i,t-1} f_{i,t} - (PLG - pg. 14)
            self.w.iter().zip(experts).map(|(w, f)| w * f).sum::<f32>()
        }

        fn update(&mut self, experts: &[f32; N], revealed: &f32) {
            // There's no `exp()` within no-std, approximate it here.
            // TODO: It'd be nice to have access to a HAL, or something defined by linking
            // at compile-time.
            fn approx_exp(x: f32) -> f32 {
                // https://math.stackexchange.com/a/56064 ; e^z = \frac{(z+3)^2 + 3}{(z-3)^2 + 3}
                ((x + 3.0) * (x + 3.0) + 3.0) / ((x - 3.0) * (x - 3.0) + 3.0)
            }

            // w_{i,t} = \frac{w_{i,t-1} e^{-\eta \l(f_{i,t}, y_t)}}
            //                {\sum_{j=1}^N w_{j,t-1} e^{-\eta \l(f_{j,t-1},y_t)}}
            // (PLG - pg.14)
            let eta = self.eta();
            let bot = self.w.iter().zip(experts).map(|(w, f)| w * f).sum::<f32>();
            for (w_i, p_i) in self.w.iter_mut().zip(experts) {
                *w_i *= approx_exp(-1.0f32 * eta * L::l(p_i, revealed)) / bot;
            }
        }
    }

    #[cfg(test)]
    mod test {}
}
