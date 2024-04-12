use crate::Filter;
use core::marker::PhantomData;

struct AlphaBeta<const N: usize, P> {
    phantom: PhantomData<P>,
}

impl<const N: usize> AlphaBeta<N, f32> {
    /// The state transition matrix: $F(n-1)$.
    fn state_transition_matrix(update_period: f32) -> [[f32; N]; N] {
        let mut fN = [[0.0f32; N]; N];
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    fN[i][j] = 1.0f32; // Diagonal.
                } else if i < j {
                    fN[i][j] = 0.0f32; // LHS.
                } else {
                    fN[i][j] = update_period;
                    for _ in 0..(j - (i + 1)) {
                        // TODO: this is busted.
                        // update_period * update_period / 2.
                        todo!();
                    }
                }
            }
        }

        fN
    }
}

impl<const N: usize> Filter<N, f32> for AlphaBeta<N, f32> {
    fn predict(T: f32, F: &[[f32; N]; N], X_s: &[f32; N]) -> [f32; N] {
        todo!();
    }
}
