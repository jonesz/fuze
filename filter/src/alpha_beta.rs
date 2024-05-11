//! An alpha-beta filter from Basic Radar Tracking by Budge.
use crate::Filter;

struct AlphaBeta<P> {
    alpha: P,
    beta: P,
}

impl AlphaBeta<f32> {
    /// The state transition matrix: $F(n-1)$.
    fn state_transition_matrix(update_period: f32) -> [[f32; 2]; 2] {
        [[1.0f32, update_period], [0.0f32, 1.0f32]] // (3.5)
    }

    /// The weighting matrix, $K(n)$.
    fn weighting_matrix(&self, update_period: f32) -> [f32; 2] {
        [self.alpha, self.beta / update_period] // (3.10)
    }
}

impl Filter<2, f32> for AlphaBeta<f32> {
    fn predict(update_period: f32, X_s: &[f32; 2]) -> [f32; 2] {
        let mut X_p = [0.0f32; 2];
        // $X_p(n) = F(n-1) X_s(n-1)$ (3.1).
        for (X_p_i, fN_i) in X_p
            .iter_mut()
            .zip(Self::state_transition_matrix(update_period))
        {
            // We're iterating over the row, compute $fN_i * X_s^T$.
            *X_p_i = fN_i.iter().zip(X_s).map(|(a, b)| a + b).sum();
        }

        X_p
    }

    fn update(update_period: f32, X_p: &[f32; 2], y: &[f32; 2]) -> [f32; 2] {
        // $X_s(n) = X_p(n) + K(n) * (y(n) - y_p(n))$ (3.2)

        // $y_p(n) = H^T * X_p(n)$
        let mut y_p = [0.0f32; 2];
        let H_t = [1.0f32, 0.0f32];
        for (y_p_i, (H_t_i, X_p_i)) in y_p.iter_mut().zip(H_t.iter().zip(X_p)) {
            *y_p_i = H_t_i * X_p_i;
        }

        // $y_i - y_p_i$
        let mut residual = [0.0f32; 2];
        for (r_i, (y_i, y_p_i)) in residual.iter_mut().zip(y.iter().zip(y_p)) {
            *r_i = y_i - y_p_i;
        }

        todo!()
    }
}
