mod alpha_beta;

trait Filter<const N: usize, P> {
    fn predict(update_period: P, X_s: &[P; N]) -> [P; N];
    fn update(update_period: P, X_p: &[T; N], y: &[T; N]) -> [T; N];
}
