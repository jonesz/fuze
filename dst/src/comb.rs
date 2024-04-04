use crate::product;

pub fn comb_dempster_q<const N: usize, S>(bba_s: [&[(S, f32)]; N], q: S) -> f32
where
    S: Copy + crate::set::SetOperations,
{
    // TODO: Does this first `map` "complete", placing a massive value onto the
    // stack? The below `fold` can be computed for each iteration of the `map`.
    let (k, m) = product::CartesianProduct::new(bba_s)
        .map(|item| {
            item.into_iter()
                .reduce(|(acc_s, acc_m), (set, mass)| (acc_s.intersection(&set), acc_m * mass))
                .unwrap()
        })
        .fold((0.0f32, 0.0f32), |(acc_k, acc_m), (set, mass)| {
            // K = Sum{A = \empty} m; M = Sum{A = Q} m.
            match (set.is_empty(), set.is_subset(&q) && q.is_subset(&set)) {
                (true, false) => (acc_k + mass, acc_m),
                (false, true) => (acc_k, acc_m + mass),
                (_, _) => (acc_k, acc_m),
            }
        });

    (1.0 / (1.0 - k)) * m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comb_dempster_q() {
        // https://en.wikipedia.org/wiki/Dempster%E2%80%93Shafer_theory#Example_producing_correct_results_in_case_of_high_conflict
        const FILM_X: usize = 0b001;
        const FILM_Y: usize = 0b010;
        const FILM_Z: usize = 0b100;

        const FILMS_HIGH_CONFLICT: [&[(usize, f32)]; 2] = [
            &[(FILM_X, 0.99f32), (FILM_Y, 0.01f32)],
            &[(FILM_Z, 0.99f32), (FILM_Y, 0.01f32)],
        ];

        // TODO: Determine what this epsilon should be.
        let eps = 0.001f32;

        assert!((comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_Y) - 1.0f32).abs() < eps);
        assert!(comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_X) < eps);
        assert!(comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_Z) < eps);
    }
}
