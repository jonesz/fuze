use crate::Filter;

struct AlphaBeta<P> {
    alpha: P,
    beta: P,
}

impl AlphaBeta<f32> {
    /// The state transition matrix: $F(n-1)$.
    fn state_transition_matrix(update_period: f32) -> [[f32; 2]; 2] {
        [[1.0f32, update_period], [0.0f32, 1.0f32]]
    }

    // The weighting matrix, $K(n)$.
    fn weighting_matrix(update_period: f32) -> [f32; 2] {
        [alpha, beta/update_period]
    }
}

impl Filter<2, f32> for AlphaBeta<f32> {
    fn predict(update_period: f32, F: &[[f32; 2]; 2], X_s: &[f32; 2]) -> [f32; 2] {
        let mut X_p = [0.0f32; 2];
        let fN = Self::state_transition_matrix(update_period);

        for i in 0..2 {
            // $X_p(n) = F(n-1) X_s(n-1)$ (3.1).
            X_p[i] = fN[i].iter().zip(X_s).map(|(a, b)| a + b).sum();
        }

        X_p
    }
}
