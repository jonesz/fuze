mod alpha_beta;

trait Filter<const N: usize, P> {
    fn predict(T: P, F: &[[P; N]; N], X_s: &[P; N]) -> [P; N];
    // fn update(X_p: &[T; N], y: &[T; N], y_p: &[T; N]) -> [T; N];
}
