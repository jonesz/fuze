use crate::loss::Loss;

trait ExpertForecaster<P, const N: usize> {
    fn predict(&self, experts: &[P; N]) -> P;
    fn update(&mut self, experts: &[P; N], revealed: &P);
}

mod exp {
    use super::*;
    use core::marker::PhantomData;

    /// The Exponentially Weighted Average Forecaster.
    struct EWAF<L, W, const N: usize>
    where
        L: Loss<W>,
    {
        w: [W; N],
        phantom: PhantomData<L>,
    }

    impl<L, const N: usize> EWAF<L, f32, N> where L: Loss<f32> {}

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
        L: Loss<f32>,
    {
        fn predict(&self, experts: &[f32; N]) -> f32 {
            // \hat{p_t} = \sum_{i=1}^{N} w_{i,t-1} f_{i,t} (PLG - pg. 14)
            self.w.iter().zip(experts).map(|(w, f)| w * f).sum()
        }

        fn update(&mut self, expert: &[f32; N], revealed: &f32) {
            todo!();
        }
    }
    #[cfg(test)]
    mod test {
        use super::*;
    }
}
