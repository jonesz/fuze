use dst::approx::KX;
use dst::comb::Dempster;
use dst::dst::{bel, comb_approx};

#[test]
fn films_high_conflict() {
    // https://en.wikipedia.org/wiki/Dempster%E2%80%93Shafer_theory#Example_producing_correct_results_in_case_of_high_conflict
    const FILM_X: usize = 0b001;
    const FILM_Y: usize = 0b010;
    const FILM_Z: usize = 0b100;
    const FILMS_HIGH_CONFLICT: [[(usize, f32); 2]; 2] = [
        [(FILM_X, 0.99f32), (FILM_Y, 0.01f32)],
        [(FILM_Z, 0.99f32), (FILM_Y, 0.01f32)],
    ];

    let bba = comb_approx::<2, usize, f32, KX, Dempster>(FILMS_HIGH_CONFLICT);

    const EPS: f32 = 0.001f32;

    // `FILM_Y` should be `1.0`~.
    assert!((bel(&bba, &FILM_Y) - 1.0f32).abs() < EPS);
    assert!(bel(&bba, &FILM_X) < EPS);
    assert!(bel(&bba, &FILM_Z) < EPS);
}
