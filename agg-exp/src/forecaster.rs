use crate::loss::Loss;

trait ExpertForecaster<P, const N: usize> {
    fn predict(&self, experts: &[P; N]) -> P;
    fn update(&mut self, experts: &[P; N], revealed: &P);
}

mod exp {
    use super::*;

    /// The Exponentially Weighted Average Forecaster.
    struct EWAF<W, const N: usize> {
        w: [W; N],
    }

    impl<const N: usize> EWAF<f32, N> {}

    impl<const N: usize> Forecaster<f32, N> for EWAF<f32, N> {
        fn predict(&self, expert: &[f32; N]) -> f32 {
            todo!()
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
